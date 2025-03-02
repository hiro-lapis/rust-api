## rust api

### set up

```
docker compose up -d --build
```

each task defined in Makefile.toml can be excuted by `cargo make`

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
```