//! PostgreSQL related query builder extensions
//!
//! Everything in this module is re-exported from database agnostic locations.
//! You should rely on the re-exports rather than this module directly. It is
//! kept separate purely for documentation purposes.

pub(crate) mod expression_methods;
#[doc(hidden)]
pub mod operators;

/// PostgreSQL specific expression DSL methods.
///
/// This module will be glob imported by
/// [`diesel_geometry::dsl`](::diesel_geometry::dsl) when compiled with the `feature =
/// "postgres"` flag.
pub mod dsl {}
