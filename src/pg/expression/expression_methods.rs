use diesel::expression::{AsExpression, Expression};
use diesel::pg::expression::operators::IsContainedBy;

use super::operators::SameAs;
use sql_types::{self, Point};

pub trait PgSameAsExpressionMethods<ST>: Expression + Sized {
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
    /// #     let connection = establish_connection();
    /// let found_drawing_id = shapes
    ///     .select(drawing_id)
    ///     .filter(centroid.same_as(PgPoint(1.0, 2.0)))
    ///     .first(&connection);
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

pub trait PgIsContainedByExpressionMethods<ST>: Expression + Sized {
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
    /// #
    /// # fn main() {
    /// #     use schema::shapes::dsl::*;
    /// #     let connection = establish_connection();
    /// // Looking for point at (1,2)
    /// let found_drawing_id = shapes
    ///     .select(drawing_id)
    ///     .filter(centroid.is_contained_by(PgBox(PgPoint(0.5, 1.5), PgPoint(3.0, 5.0))))
    ///     .first(&connection);
    /// assert_eq!(Ok(2), found_drawing_id);
    /// # }
    fn is_contained_by<T>(self: Self, other: T) -> IsContainedBy<Self, T::Expression>
    where
        T: AsExpression<ST>,
    {
        IsContainedBy::new(self, other.as_expression())
    }
}

impl<T> PgIsContainedByExpressionMethods<sql_types::Box> for T
where
    T: Expression<SqlType = Point>,
{
}
