pub mod geometric;

pub mod sql_types {
    /// The PostgreSQL [Point](https://www.postgresql.org/docs/current/static/datatype-geometric.html) type.
    ///
    /// ### [`ToSql`](::diesel::serialize::ToSql) impls
    ///
    /// - [`PgPoint`](::pg::data_types::PgPoint)
    ///
    /// ### [`FromSql`](::diesel::deserialize::FromSql) impls
    ///
    /// - [`PgPoint`](::pg::data_types::PgPoint)
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #![allow(dead_code)]
    /// # #[macro_use] extern crate diesel;
    /// # extern crate diesel_geometry;
    /// # include!("../../doctest_setup.rs");
    /// use diesel_geometry::data_types::PgPoint;
    ///
    ///
    /// table! {
    ///     use diesel::sql_types::*;
    ///     use diesel_geometry::sql_types::Point;
    ///     items {
    ///         id -> Integer,
    ///         name -> VarChar,
    ///         location -> Point,
    ///     }
    /// }
    ///
    /// # fn main() {
    /// #     use diesel::insert_into;
    /// #     use items::dsl::*;
    /// #     let connection = connection_no_data();
    /// #     connection.execute("CREATE TABLE items (
    /// #         id SERIAL PRIMARY KEY,
    /// #         name VARCHAR NOT NULL,
    /// #         location POINT NOT NULL
    /// #     )").unwrap();
    /// let inserted_location = insert_into(items)
    ///     .values((name.eq("Shiny Thing"), location.eq(PgPoint(3.1, 9.4))))
    ///     .returning(location)
    ///     .get_result(&connection);
    /// assert_eq!(Ok(PgPoint(3.1, 9.4)), inserted_location);
    /// # }
    /// ```
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[postgres(oid = "600", array_oid = "1017")]
    pub struct Point;

    /// The PostgreSQL [Box](https://www.postgresql.org/docs/current/static/datatype-geometric.html) type.
    ///
    /// ### [`ToSql`](::diesel::serialize::ToSql) impls
    ///
    /// - [`PgBox`](::pg::data_types::PgBox)
    ///
    /// ### [`FromSql`](::diesel::deserialize::FromSql) impls
    ///
    /// - [`PgBox`](::pg::data_types::PgBox)
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #![allow(dead_code)]
    /// # #[macro_use] extern crate diesel;
    /// # extern crate diesel_geometry;
    /// # include!("../../doctest_setup.rs");
    /// use diesel_geometry::data_types::{PgPoint, PgBox};
    ///
    ///
    /// table! {
    ///     use diesel::sql_types::*;
    ///     use diesel_geometry::sql_types::Point;
    ///     items {
    ///         id -> Integer,
    ///         name -> VarChar,
    ///         location -> Point,
    ///     }
    /// }
    ///
    /// # fn main() {
    /// #     use diesel::insert_into;
    /// #     use diesel_geometry::prelude::*;
    /// #     use items::dsl::*;
    /// #     let connection = connection_no_data();
    /// #     connection.execute("CREATE TABLE items (
    /// #         id SERIAL PRIMARY KEY,
    /// #         name VARCHAR NOT NULL,
    /// #         location POINT NOT NULL
    /// #     )").unwrap();
    /// insert_into(items)
    ///     .values((name.eq("Shiny Thing"), location.eq(PgPoint(3.1, 9.4))))
    ///     .returning(location)
    ///     .execute(&connection)
    ///     .unwrap();
    /// let inserted_location = items
    ///     .select(location)
    ///     .filter(location.is_contained_by(PgBox(PgPoint(0.,0.), PgPoint(10., 10.))))
    ///     .first(&connection);
    /// assert_eq!(Ok(PgPoint(3.1, 9.4)), inserted_location);
    /// # }
    /// ```
    #[derive(Debug, Clone, Copy, Default, QueryId, SqlType)]
    #[postgres(oid = "603", array_oid = "1020")]
    pub struct Box;
}
