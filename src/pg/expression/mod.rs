pub(crate) mod expression_methods;
#[doc(hidden)]
pub mod operators;

/// PostgreSQL specific expression DSL methods.
///
/// This module will be glob imported by
/// [`diesel::dsl`](../../../dsl/index.html) when compiled with the `feature =
/// "postgres"` flag.
pub mod dsl {}
