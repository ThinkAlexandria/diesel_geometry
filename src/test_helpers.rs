use diesel::prelude::*;

cfg_if! {
    if #[cfg(feature = "postgres")] {
        extern crate dotenv;

        pub type TestConnection = PgConnection;

        pub fn connection() -> TestConnection {
            let conn = PgConnection::establish(&database_url()).unwrap();
            conn.begin_test_transaction().unwrap();
            conn
        }

        pub fn database_url() -> String {
            dotenv::var("PG_DATABASE_URL")
                .or_else(|_| dotenv::var("DATABASE_URL"))
                .expect("DATABASE_URL must be set in order to run tests")
        }

        use diesel::serialize::Output;
        use diesel::sql_types::TypeMetadata;
        /// Returns a `Output` suitable for testing `ToSql` implementations.
        /// Unsafe to use for testing types which perform dynamic metadata lookup.
        pub fn create_testing_output<DB: TypeMetadata>() -> Output<'static, Vec<u8>, DB> {
            use std::mem;
            #[cfg_attr(feature = "clippy", allow(invalid_ref))]
            Output::new(Vec::new(), unsafe { mem::uninitialized() })
        }
    } else {
        compile_error!(
            "At least one backend must be used to test this crate.\n \
            Pass argument `--features \"<backend>\"` with one or more of the following backends, \
            'mysql' or 'postgres'. \n\n \
            ex. cargo test --features \"mysql postgres\"\n"
        );
    }
}
