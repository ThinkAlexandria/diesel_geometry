use diesel::expression::{AsExpression, Expression};

use super::operators::{IsContainedBy, SameAs};
use sql_types::{self, Circle, Point};

pub trait PgSameAsExpressionMethods<ST: diesel::sql_types::SingleValue>: Expression + Sized {
    /// Creates a PostgresSQL `~=`  expression.
    ///
    /// The "same as" operator, ~=, represents the usual notion of equality for the `point`, `box`,
    /// `polygon`, and `circle` types. Some of these types have an = operator, but = compares for
    /// equal areas only. The other scalar operators (<= and so on) likewise compare areas for
    /// these types.
    ///
    /// # Example
    /// ```rust
    /// # #![allow(dead_code)]
    /// # #[macro_use] extern crate diesel;
    /// # extern crate diesel_geometry;
    /// # include!("../../doctest_setup.rs");
    /// # use diesel_geometry::data_types::PgPoint;
    /// #
    /// # fn main() {
    /// #     use schema::shapes::dsl::*;
    /// #     let mut connection = establish_connection();
    /// let found_drawing_id = shapes
    ///     .select(drawing_id)
    ///     .filter(centroid.same_as(PgPoint(1.0, 2.0)))
    ///     .first(&mut connection);
    /// assert_eq!(Ok(2), found_drawing_id);
    /// # }
    fn same_as<T>(self: Self, other: T) -> SameAs<Self, T::Expression>
    where
        T: AsExpression<ST>,
    {
        SameAs::new(self, other.as_expression())
    }
}

impl<T: Expression<SqlType = Point>> PgSameAsExpressionMethods<Point> for T {}
impl<T: Expression<SqlType = sql_types::Box>> PgSameAsExpressionMethods<sql_types::Box> for T {}
impl<T: Expression<SqlType = Circle>> PgSameAsExpressionMethods<Circle> for T {}

pub trait PgIsContainedByExpressionMethods<ST: diesel::sql_types::SingleValue>: Expression + Sized {
    /// Creates a PostgresSQL `<@` expression.
    ///
    /// For geometric types.
    ///
    /// # Example
    /// ```rust
    /// # #![allow(dead_code)]
    /// # #[macro_use] extern crate diesel;
    /// # extern crate diesel_geometry;
    /// # include!("../../doctest_setup.rs");
    /// # use diesel_geometry::data_types::{PgBox, PgPoint};
    /// # use diesel_geometry::sql_types;
    /// #
    /// # fn main() {
    /// #     use schema::shapes::dsl::*;
    /// #     let mut connection = establish_connection();
    /// // Looking for point at (1,2)
    /// let found_drawing_id = shapes
    ///     .select(drawing_id)
    ///     .filter(
    ///         centroid.is_contained_by(
    ///             PgBox(PgPoint(0.5, 1.5), PgPoint(3.0,5.0)).into_sql::<sql_types::Box>()
    ///         )
    ///     )
    ///     .first(&mut connection);
    /// assert_eq!(Ok(2), found_drawing_id);
    /// # }
    fn is_contained_by<T>(self: Self, other: T) -> IsContainedBy<Self, T::Expression>
    where
        T: AsExpression<ST>,
    {
        IsContainedBy::new(self, other.as_expression())
    }
}

// A Circle can contain a Point or a Circle but not a Box
pub trait CanBeContainedByCircle {}
impl CanBeContainedByCircle for Point {}
impl CanBeContainedByCircle for Circle {}

// A Box can contain a Point, Circle, and a Box
pub trait CanBeContainedByBox {}
impl CanBeContainedByBox for Point {}
impl CanBeContainedByBox for Circle {}
impl CanBeContainedByBox for sql_types::Box {}

impl<T> PgIsContainedByExpressionMethods<Circle> for T
where
    T: Expression,
    T::SqlType: CanBeContainedByCircle,
{}
impl<T> PgIsContainedByExpressionMethods<sql_types::Box> for T
where
    T: Expression,
    T::SqlType: CanBeContainedByBox,
{}
