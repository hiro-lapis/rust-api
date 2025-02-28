## rust api

### set up

```
docker compose up -d --build
```

each task defined in Makefile.toml can be excuted by `cargo make`

```
// run [tasks.run]
cargo make run

// run [tasks.run-in-docker]
cargo make run-in-docker
```

### recommended packages

cargo-make

```
cargo install --force cargo-make
```