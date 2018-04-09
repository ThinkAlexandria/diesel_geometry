//! Adds various methods to construct new expressions. These traits are exported
//! by default, and implemented automatically.
//!
//! You can rely on the methods provided by this trait existing on any
//! `Expression` of the appropriate type. You should not rely on the specific
//! traits existing, their names, or their organization.

#[cfg(feature = "postgres")]
#[doc(inline)]
pub use pg::expression::expression_methods::*;
