# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105)

# 1.2.2 (Apr 11, 2018)

## Docs

- Added changelog, code of conduct, github issue template

- Improved library level documentation.

# 1.2.1 (Apr 8, 2018)

## Docs

- Added repository link to Cargo.toml

# 1.2.0 (Apr 8, 2018)

## Added

- Added support for PostgresSQL [Box](https://www.postgresql.org/docs/current/static/datatype-geometric.html) type.

- Added support for PostgresSQL [Point](https://www.postgresql.org/docs/current/static/datatype-geometric.html) type.

- Added support for Postgres `<@` "is contained by" operator using the `is_contained_by` method.

- Added support for Postgre `~=` "same as" operator using `same_as` method.
