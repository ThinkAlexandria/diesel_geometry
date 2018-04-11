//! Types which represent a SQL data type.
//!
//! The structs in this module are *only* used as markers to represent a SQL type.
//! They should never be used in your structs.
//! If you'd like to know the rust types which can be used for a given SQL type,
//! see the documentation for that SQL type.
//!
//! Any backend specific types are re-exported through this module

#[cfg(feature = "postgres")]
pub use pg::types::sql_types::*;
