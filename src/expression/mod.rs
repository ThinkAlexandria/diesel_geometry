//! AST types representing various typed SQL expressions.
//!
//! Almost all types implement either [`Expression`](::diesel::Expression) or
//! [`AsExpression`](::diesel::AsExpression).

pub mod dsl {
    #[cfg(feature = "postgres")]
    pub use pg::expression::dsl::*;

}
