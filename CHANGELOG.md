# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105)

# 1.3.0 (Aug 30, 2018)

## Docs

- Added contributor documentation for running tests locally to README.

- Fixed typographical errors in CHANGELOG and README.

- Added documentation for using diesel.toml with this crate.

## Added

- Added support for PostgreSQL [Circle](https://www.postgresql.org/docs/current/static/datatype-geometric.html) type.

- Added support for serde for all geometry types. To use it, add `features = ["serde"]`.

- Added support for `diesel 1.3.?`

# 1.2.2 (Apr 11, 2018)

## Docs

- Added changelog, code of conduct, GitHub issue template.

- Improved library level documentation.

# 1.2.1 (Apr 8, 2018)

## Docs

- Added repository link to Cargo.toml.

# 1.2.0 (Apr 8, 2018)

## Added

- Added support for PostgreSQL [Box](https://www.postgresql.org/docs/current/static/datatype-geometric.html) type.

- Added support for PostgreSQL [Point](https://www.postgresql.org/docs/current/static/datatype-geometric.html) type.

- Added support for Postgres `<@` "is contained by" operator using the `is_contained_by` method.

- Added support for Postgres `~=` "same as" operator using `same_as` method.
