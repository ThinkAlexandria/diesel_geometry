use diesel::prelude::*;

cfg_if! {
    if #[cfg(feature = "postgres")] {
        extern crate dotenv;

        pub type TestConnection = PgConnection;

        pub fn connection() -> TestConnection {
            let mut conn = PgConnection::establish(&database_url()).unwrap();
            conn.begin_test_transaction().unwrap();
            conn
        }

        pub fn database_url() -> String {
            dotenv::var("PG_DATABASE_URL")
                .or_else(|_| dotenv::var("DATABASE_URL"))
                .expect("DATABASE_URL must be set in order to run tests")
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
