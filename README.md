# Sqlite Deps - rs

This library isolates SQLite resources into a single dependency for rustlang projects.

It provides:
- the `Connection` struct from the [rusqlite](https://crates.io/crates/rusqlite) library.
- a pool of `Connection`

## How to use

```rs
use sqlite_deps::{ConnectionPool, Params};

// filepath, connection count limit
let pool = ConnectionPool::from_params(Params {
    db_filepath: PathBuf::from("./filepath/to/sqlite.db")m
    connection_limit: 4,
});

let conn = pool.get_connection()?;
let _ = pool.set_connection(conn)?;
```

# License

`sqlite_connection_pool-rs` is released under the BSD 3-Clause License