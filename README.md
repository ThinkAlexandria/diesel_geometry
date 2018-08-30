Support for geometric types and geometric functions for Diesel.
====================
[![Crates.io](https://img.shields.io/crates/v/diesel_geometry.svg)](https://crates.io/crates/diesel_geometry)

API Documentation [latest release](https://docs.rs/diesel_geometry)

`diesel_geometry` provides geometric types and geometric functions.

## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.

#### Testing

To run the integration tests on your machine, you need to have read and write
access to a running instance of Postgres. The database is not permanently
altered because all tests are wrapped in a single transaction that is rolled
back.

When running cargo test set the environment variable `PG_DATABASE_URL` with
the Postgres connection string for your database.

**Example**

You can start a Postgres database locally using docker with:

```
docker run -d --rm --name postgres -p 5432:5432 postgres:10
```

And then run:

```
PG_DATABASE_URL=postgres://postgres:postgres@localhost:5432 cargo test
```
