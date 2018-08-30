//! Support for Geometric types under PostgreSQL.

use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::io::prelude::*;

use diesel::deserialize::{self, FromSql};
use diesel::expression::bound::Bound;
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Nullable;
use sql_types::{self, Circle, Point};

/// Point is represented in Postgres as a tuple of 64 bit floating point values (x, y).  This
/// struct is a dumb wrapper type, meant only to indicate the tuple's meaning.
#[derive(Debug, Clone, PartialEq, Copy, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "Point"]
pub struct PgPoint(pub f64, pub f64);

impl FromSql<Point, Pg> for PgPoint {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let mut bytes = not_none!(bytes);
        let x = bytes.read_f64::<NetworkEndian>()?;
        let y = bytes.read_f64::<NetworkEndian>()?;
        Ok(PgPoint(x, y))
    }
}

impl ToSql<Point, Pg> for PgPoint {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_f64::<NetworkEndian>(self.0)?;
        out.write_f64::<NetworkEndian>(self.1)?;
        Ok(IsNull::No)
    }
}

/// Box is represented in Postgres as a tuple of points `(lower left, upper
/// right)`. This struct is a dumb wrapper type, meant only to indicate the tuple's meaning.
#[derive(Debug, Clone, PartialEq, Copy, FromSqlRow)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(AsExpression)]
#[sql_type = "sql_types::Box"]
pub struct PgBox(pub PgPoint, pub PgPoint);

// We must manually derive AsExpression because sql_types::Box would conflict with the builtin Box
// type

//impl AsExpression<sql_types::Box> for PgBox {
//    type Expression = Bound<sql_types::Box, Self>;
//
//    fn as_expression(self) -> Self::Expression {
//        Bound::new(self)
//    }
//}
//
//impl<'a> AsExpression<sql_types::Box> for &'a PgBox {
//    type Expression = Bound<sql_types::Box, Self>;
//
//    fn as_expression(self) -> Self::Expression {
//        Bound::new(self)
//    }
//}
//
//impl AsExpression<Nullable<sql_types::Box>> for PgBox {
//    type Expression = Bound<Nullable<sql_types::Box>, Self>;
//
//    fn as_expression(self) -> Self::Expression {
//        Bound::new(self)
//    }
//}
//
//impl<'a> AsExpression<Nullable<sql_types::Box>> for &'a PgBox {
//    type Expression = Bound<Nullable<sql_types::Box>, Self>;
//
//    fn as_expression(self) -> Self::Expression {
//        Bound::new(self)
//    }
//}

// https://github.com/postgres/postgres/blob/9d4649ca49416111aee2c84b7e4441a0b7aa2fac/src/backend/utils/adt/geo_ops.c

impl FromSql<sql_types::Box, Pg> for PgBox {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        let (upper_bytes, lower_bytes) = bytes.split_at(16);
        // By convention the box is written as (lower left, upper right) and is stored as [ high.x,
        // high,y, low.x, low.y ].
        let upper = PgPoint::from_sql(Some(upper_bytes))?;
        let lower = PgPoint::from_sql(Some(lower_bytes))?;
        Ok(PgBox(lower, upper))
    }
}

impl ToSql<sql_types::Box, Pg> for PgBox {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        // By convention the box is written as (lower left, upper right)
        // and is stored as [ high.x, high,y, low.x, low.y ]. Postgres will reorder the corners if
        // necessary. We write the points assuming the Box is following convention.
        <PgPoint as ToSql<Point, Pg>>::to_sql(&self.1, out)?;
        <PgPoint as ToSql<Point, Pg>>::to_sql(&self.0, out)?;

        Ok(IsNull::No)
    }
}

//impl ToSql<Nullable<sql_types::Box>, Pg> for PgBox {
//    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
//        ToSql::<sql_types::Box, Pg>::to_sql(self, out)
//    }
//}

/// Circle is represented in Postgres as a tuple of center point and radius `(center, radius)`.
/// This struct is a dumb wrapper type, meant only to indicate the tuple's meaning.
#[derive(Debug, Clone, PartialEq, Copy, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "Circle"]
pub struct PgCircle(pub PgPoint, pub f64);

impl FromSql<sql_types::Circle, Pg> for PgCircle {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        let (center_bytes, mut radius_bytes) = bytes.split_at(16);
        let center = PgPoint::from_sql(Some(center_bytes))?;
        let radius = radius_bytes.read_f64::<NetworkEndian>()?;
        Ok(PgCircle(center, radius))
    }
}

impl ToSql<sql_types::Circle, Pg> for PgCircle {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <PgPoint as ToSql<Point, Pg>>::to_sql(&self.0, out)?;
        out.write_f64::<NetworkEndian>(self.1)?;

        Ok(IsNull::No)
    }
}

#[cfg(test)]
mod tests {
    use diesel;
    use diesel::deserialize::FromSql;
    use diesel::dsl::sql;
    use diesel::pg::Pg;
    use diesel::prelude::*;
    use diesel::select;
    use diesel::serialize::ToSql;

    use expression_methods::*;
    use pg::types::geometric::{PgBox, PgCircle, PgPoint};
    use sql_types::{self, Circle, Point};
    use test_helpers::{connection, create_testing_output};

    #[test]
    fn point_roundtrip() {
        let mut bytes = create_testing_output();
        let input_point = PgPoint(4.5, 3439.1);
        ToSql::<Point, Pg>::to_sql(&input_point, &mut bytes).unwrap();
        let output_point: PgPoint = FromSql::from_sql(Some(bytes.as_ref())).unwrap();
        assert_eq!(input_point, output_point);
    }

    #[test]
    fn no_point_from_sql() {
        let uuid: Result<PgPoint, _> = FromSql::<Point, Pg>::from_sql(None);
        assert_eq!(
            uuid.unwrap_err().description(),
            "Unexpected null for non-null column"
        );
    }

    #[test]
    fn point_encodes_correctly() {
        let connection = connection();
        let point = PgPoint(3.0, 4.0);
        let query = select(sql::<Point>("point '(3, 4)'").same_as(point));
        assert!(query.get_result::<bool>(&connection).unwrap());
    }

    mod schema {
        table! {
            use sql_types::Point;
            use diesel::sql_types::*;
            items {
                id -> Integer,
                name -> VarChar,
                location -> Point,
            }
        }

        table! {
            use diesel::sql_types::*;
            use sql_types::Box;
            box_roundtrip {
                id -> Integer,
                boxes -> Nullable<Box>,
            }
        }
        table! {
            use diesel::sql_types::*;
            use sql_types::Circle;
            circle_roundtrip {
                id -> Integer,
                circles -> Nullable<Circle>,
            }
        }
    }

    #[test]
    fn point_is_insertable() {
        // Compile check that PgPoint can be used in insertable context,
        use self::schema::items;
        #[derive(Debug, Clone, Copy, Insertable)]
        #[table_name = "items"]
        struct NewItem {
            name: &'static str,
            location: ::pg::types::geometric::PgPoint,
        }
        //use self::schema::items::dsl::*;
        let _query_location = diesel::insert_into(items::table)
            .values(&NewItem {
                name: "Shiny Thing",
                location: PgPoint(3.1, 9.4),
            }).returning(schema::items::dsl::location);
    }

    #[test]
    fn point_is_queryable() {
        let connection = connection();
        // Compile check that PgPoint can be used in queryable context,
        #[derive(Debug, Clone, Queryable)]
        struct Item {
            id: i32,
            name: String,
            location: ::pg::types::geometric::PgPoint,
        }
        use self::schema::items::dsl::*;
        let _query_row = items
            .filter(id.eq(1))
            .filter(location.same_as(PgPoint(3.1, 9.4)))
            .get_result::<Item>(&connection);
    }

    #[test]
    fn box_roundtrip() {
        let connection = connection();
        connection
            .execute(
                "CREATE TABLE box_roundtrip (
            id SERIAL PRIMARY KEY,
            boxes BOX
        )",
            ).unwrap();
        use self::schema::box_roundtrip;
        #[derive(Debug, PartialEq, Insertable, Queryable)]
        #[table_name = "box_roundtrip"]
        struct Roundtrip {
            id: i32,
            boxes: Option<::pg::types::geometric::PgBox>,
        }
        let data = Roundtrip {
            id: 6,
            boxes: Some(PgBox(PgPoint(0., 0.), PgPoint(3., 4.))),
        };
        diesel::insert_into(box_roundtrip::table)
            .values(&data)
            .execute(&connection)
            .unwrap();
        let x = box_roundtrip::table.first::<Roundtrip>(&connection);
        match x {
            Ok(record) => assert_eq!(data, record),
            Err(_) => panic!(),
        }
    }

    use diesel::expression::AsExpression;

    #[test]
    fn point_contained_queries() {
        let connection = connection();
        let point = PgPoint(1., 1.);
        let bounding_box = PgBox(PgPoint(0., 0.), PgPoint(2., 2.));
        let bounding_circle = PgCircle(PgPoint(0., 0.), 3.0);
        let is_contained = diesel::select(
            point
                .into_sql::<Point>()
                .is_contained_by(bounding_circle.into_sql::<Circle>()),
        ).get_result::<bool>(&connection)
        .unwrap();
        assert!(is_contained);
        let is_contained = diesel::select(
            AsExpression::<Point>::as_expression(point)
                .is_contained_by(bounding_box.into_sql::<sql_types::Box>()),
        ).get_result::<bool>(&connection)
        .unwrap();
        assert!(is_contained);
    }

    #[test]
    fn circle_roundtrip() {
        let connection = connection();
        connection
            .execute(
                "CREATE TABLE circle_roundtrip (
            id SERIAL PRIMARY KEY,
            circles CIRCLE
        )",
            ).unwrap();
        use self::schema::circle_roundtrip;
        #[derive(Debug, PartialEq, Insertable, Queryable)]
        #[table_name = "circle_roundtrip"]
        struct Roundtrip {
            id: i32,
            circles: Option<::pg::types::geometric::PgCircle>,
        }
        let data = Roundtrip {
            id: 6,
            circles: Some(PgCircle(PgPoint(3., 4.), 1.5)),
        };
        diesel::insert_into(circle_roundtrip::table)
            .values(&data)
            .execute(&connection)
            .unwrap();
        let x = circle_roundtrip::table.first::<Roundtrip>(&connection);
        match x {
            Ok(record) => assert_eq!(data, record),
            Err(_) => panic!(),
        }
    }
}
