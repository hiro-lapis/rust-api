[tasks.compose-up-jaeger]
extend = "set-env-docker"
command = "docker"
args = ["compose", "up", "-d", "jaeger"]## rust api

This repository is the manual copy of [rusty-book-manager](https://github.com/rust-web-app-book/rusty-book-manager).  

### set up

```
docker compose up -d --build
```

each task defined in Makefile.toml can be excuted by `cargo make run`  
Note that `cargo run` seems to work, but fails in reading env vars.  

```
// install cargo
cargo install --force cargo-make


// run [tasks.run-in-docker]
cargo make run-in-docker

// during development, if you want to restart local
cargo make compose-remove \
cargo make build \
cargo make initial-setup \
cargo make run
```

Optionally, conducive commands are executable.  

```
// test
cargo nextest run

// generate migration file
cargo make gen-migrate

// execute migration
cargo make migrate

// revert migration
cargo makme sqlx migrate revert

// you can debug with...
cargo mak --loglevel verbose run

// start creating new crate
cargo new --lib <crate name>
```

### logging

`http://localhost:16686` is serverd as tracing logger by jaeger container.  

`http://127.0.0.1:8080/docs` is served as OpenApi docs
