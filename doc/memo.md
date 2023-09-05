# Liens divers et variés sur le projet


- main
    - appstate
    - cnx postgres sqlite
    - load lexic lex_portail...application...table
    - déclaration des routes
- routes    router                              cruder                                  tpl_base
- get                                                                               tpl_portail
- get                                                                               tpl_application
- get   route_views.rs          view        list.rs     crud_list   record_elements tpl_view        tpl_view_elements
- get   route_form.rs           form        read.rs     crud_read   record_elements tpl_form        tpl_form_elements
- get   route_edit.rs           edit        read.rs     crud_read   record_elements tpl_edit        tpl_edit_element
- get   route_add.rs            add                                 record_elements tpl_add         tpl_edit_element
- post  route_edit_post.rs      edit_post   update.rs   crud_update
- post  route_add_post.rs       add_post    insert.rs   crud_insert


## Cargo
- cargo install cargo-watch
- cargo watch -x run
- https://rurust.github.io/cargo-docs-ru/environment-variables.html

## RUST
- Le book https://doc.rust-lang.org/book/title-page.html
- https://blog.guillaume-gomez.fr/Rust/1/1
- https://rust.developpez.com/cours/
- https://rust.developpez.com/tutoriels/rust-par-l-exemple/
- [Read the tutorial article](https://docs.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust/)
- https://blog.logrocket.com/understanding-rust-string-str/

## modules
- https://larouille.github.io/modules/#mettre-notre-module-dans-un-fichier-dedie


## ACTIX
- https://docs.rs/actix-web/latest/actix_web/index.html
- https://actix.rs/docs
- https://github.com/TechEmpower/FrameworkBenchmarks/tree/master/frameworks/Rust/actix
- https://www.lpalmieri.com/posts/session-based-authentication-in-rust/
- https://turreta.com/blog/2020/06/11/working-with-actix-web-http-requests/

## Middleware
- https://github.com/actix/examples/tree/master/middleware

## Templates
- https://keats.github.io/tera/
- https://nivethan.dev/devlog/a-web-app-in-rust.html

- https://djc.github.io/askama/
- https://github.com/Ghostff/actix-web-mvc-boilerplate/tree/master
- https://lib.rs/crates/askama
- https://github.com/actix/examples/blob/master/templating/askama/src/main.rs
- https://djc.github.io/askama/askama.html

## SQLX
- le BASIC https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/
- https://codevoweb.com/rust-build-a-crud-api-with-sqlx-and-postgresql/
- https://www.opensourceforu.com/2022/05/sqlx-the-rust-sql-toolkit/

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

# Graphique - plotters
- https://github.com/plotters-rs/plotters

# source
- https://github.com/evoxmusic/twitter-clone-rust/blob/487198ee7b306f36dbab01f40a44345f85387db2/src/main.rs

# sqlx postgres
- https://codevoweb.com/rust-build-a-crud-api-with-sqlx-and-postgresql/
- https://github.com/wpcodevo/rust-postgres-crud-sqlx
- *** https://www.opensourceforu.com/2022/05/sqlx-the-rust-sql-toolkit/

# sqlx map
- https://gist.github.com/jeremychone/34d1e3daffc38eb602b1a9ab21298d10

# MVC
- https://github.com/Nickforall/Iron-MVC

# TESTS
## list tweets
curl http://localhost:8080/tweets
## create a tweet
curl -X POST -d '{"message": "This is a tweet"}' -H "Content-type: application/json" http://localhost:8080/tweets

# Forms
- https://dev.to/chaudharypraveen98/form-validation-in-rust-404l
- https://github.com/Keats/validator
- ko pb dépendances https://github.com/edward-shen/actix-csrf
- https://kvnallsn.github.io/actix-web-database-identity/actix_web/middleware/csrf/index.html

# Message Flash
- https://docs.rs/actix-web-flash-messages/latest/actix_web_flash_messages/index.html

# Code
```rust
// fusion
a.into_iter().map(|(k, v)| b.insert(k, v));
```
```rust
// Mutating one map
fn merge1(map1: &mut HashMap<(), ()>, map2: HashMap<(), ()>) {
    map1.extend(map2);
}

// Without mutation
fn merge2(map1: HashMap<(), ()>, map2: HashMap<(), ()>) -> HashMap<(), ()> {
    map1.into_iter().chain(map2).collect()
}

// If you only have a reference to the map to be merged in
fn merge_from_ref(map: &mut HashMap<(), ()>, map_ref: &HashMap<(), ()>) {
    map.extend(map_ref.into_iter().map(|(k, v)| (k.clone(), v.clone())));
}
```
```rust
```
```rust
```
```rust
```
```rust
```
```rust
```

