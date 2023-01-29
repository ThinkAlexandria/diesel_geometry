#[macro_use]
extern crate cfg_if;
extern crate dotenv;

use diesel::{prelude::*, connection::SimpleConnection};
use diesel_geometry::prelude::*;
use self::dotenv::dotenv;

cfg_if! {
    if #[cfg(feature = "postgres")] {
        #[allow(dead_code)]
        type DB = diesel::pg::Pg;

        fn connection_no_transaction() -> PgConnection {
            let connection_url = database_url_from_env("PG_DATABASE_URL");
            PgConnection::establish(&connection_url).unwrap()
        }

        fn connection_no_data() -> PgConnection {
            let mut connection = connection_no_transaction();
            connection.begin_test_transaction().unwrap();
            connection.batch_execute("DROP TABLE IF EXISTS drawings CASCADE").unwrap();
            connection.batch_execute("DROP TABLE IF EXISTS shapes CASCADE").unwrap();

            connection
        }

        #[allow(dead_code)]
        fn establish_connection() -> PgConnection {
            let mut connection = connection_no_data();

            connection.batch_execute("CREATE TABLE drawings (
                id SERIAL PRIMARY KEY,
                title VARCHAR NOT NULL
            )").unwrap();
            connection.batch_execute("INSERT INTO drawings (title) VALUES ('Cubism'), ('Airplanes')").unwrap();

            connection.batch_execute("CREATE TABLE shapes (
                id SERIAL PRIMARY KEY,
                drawing_id INTEGER NOT NULL,
                centroid POINT
            )").unwrap();
            connection.batch_execute("INSERT INTO shapes (drawing_id, centroid) VALUES
                               (1, point '(0, 0)'),
                               (2, point '(1,2)')").unwrap();


            connection
        }
    } else {
        compile_error!(
            "At least one backend must be used to test this crate.\n \
            Pass argument `--features \"<backend>\"` with one or more of the following backends, \
            'mysql' or 'postgres''. \n\n \
            ex. cargo test --features \"mysql postgres\"\n"
        );
    }
}

fn database_url_from_env(backend_specific_env_var: &str) -> String {
    use std::env;

    dotenv().ok();

    env::var(backend_specific_env_var)
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set in order to run tests")
}

mod schema {
    table! {
        drawings {
            id -> Integer,
            title -> VarChar,
        }
    }

    table! {
        use diesel::sql_types::*;
        use diesel_geometry::sql_types::Point;
        shapes {
            id -> Integer,
            drawing_id -> Integer,
            centroid -> Point,
        }
    }

    joinable!(shapes -> drawings (drawing_id));
    allow_tables_to_appear_in_same_query!(drawings, shapes);
}
