# Liens divers et variés sur le projet

## RUST
- Le book https://doc.rust-lang.org/book/title-page.html
- https://blog.guillaume-gomez.fr/Rust/1/1
- https://rust.developpez.com/cours/
- https://rust.developpez.com/tutoriels/rust-par-l-exemple/

## ACTIX
- https://docs.rs/actix-web/latest/actix_web/index.html
- https://actix.rs/docs
- https://github.com/TechEmpower/FrameworkBenchmarks/tree/master/frameworks/Rust/actix

## Templates
- https://github.com/Ghostff/actix-web-mvc-boilerplate/tree/master
- https://lib.rs/crates/askama
- https://github.com/actix/examples/blob/master/templating/askama/src/main.rs

## SQLX
- https://betterprogramming.pub/how-to-interact-with-postgresql-from-rust-using-sqlx-cfa2a7c758e7
- le BASIC https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
- https://codevoweb.com/rust-build-a-crud-api-with-sqlx-and-postgresql/
- https://www.opensourceforu.com/2022/05/sqlx-the-rust-sql-toolkit/

```toml
[dependencies]
sqlx = { version = "0.6", features = [ "runtime-tokio-rustls", "postgres", "macros" ] }
```

`cargo install sqlx-cli --no-default-features --features rustls,postgres`

## YAML

- https://tms-dev-blog.com/how-to-read-and-write-yaml-in-rust-with-serde/
- https://docs.rs/serde_yaml/latest/serde_yaml/
- https://serde.rs/data-model.html

## Scheduler
- https://github.com/lholden/job_scheduler/blob/master/Cargo.toml
- https://lib.rs/crates/tokio-cron-scheduler
- https://lib.rs/crates/tokio-console

## mdBook
- https://crisal.io/tmp/book-example/book/index.html
- https://rust-lang.github.io/mdBook/format/mdbook.html

## binance
- https://tms-dev-blog.com/how-to-technical-indicators-with-rust-and-binance/
- https://github.com/tmsdev82/rust-binance-technical-indicators-tutorial
- https://tms-dev-blog.com/easily-connect-to-binance-websocket-streams-with-rust/

## finance yahoo
- https://docs.rs/yahoo_finance_api/latest/yahoo_finance_api/
- https://crates.io/crates/yahoo_finance_api
- https://crates.io/crates/yahoo-finance/0.2.0
- https://github.com/fbriden/yahoo-finance-rs
- sudo apt-get install pkg-config libssl-dev

# Nom de baptème du framework

    RUSTIX = `RUST` + act`IX`-web

# Graphique - plotters
- https://github.com/plotters-rs/plotters