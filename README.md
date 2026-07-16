# Sqlite Deps - rs

This library isolates SQLite resources into a single dependency for rustlang projects.

[![Tests](https://github.com/herebythere/sqlite_deps-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/herebythere/sqlite_deps-rs/actions/workflows/tests.yml)

## How to use

### Connection

The `sqlite_deps` is a facade for the certain structs in the [`rusqlite`](https://github.com/rusqlite/rusqlite) library. 

```rs
use sqlite_deps::Connection;
```

### Connection Pool

The following example demonstrates how to create a `ConnectionPool`.

```rs
use sqlite_deps::{ConnectionPool, Params};

let pool = ConnectionPool::from_params(Params {
    filepath: PathBuf::from("./filepath/to/sqlite.db"),
    number_of_connections: 4,
});

let conn = pool.get_connection()?;
let _ = pool.set_connection(conn)?;
```

# License

`sqlite_deps-rs` is released under the BSD 3-Clause License
