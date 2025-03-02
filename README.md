## rust api

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
```

Optionally, conducive commands are executable.  

```
// test
cargo nextest run

// you can debug with...
cargo mak --loglevel verbose run

// start creating new crate
cargo new --lib <crate name>
```