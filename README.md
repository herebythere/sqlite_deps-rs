# Sqlite Deps - rs

This library isolates SQLite resources into a single dependency for rustlang projects.

[![Tests](https://github.com/herebythere/sqlite_deps-rs/actions/workflows/Tests.yml/badge.svg)](https://github.com/herebythere/sqlite_deps-rs/actions/workflows/Tests.yml)

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

`sqlite_deps-rs` is released under the BSD 3-Clause License
