## rust api

### set up

```
docker compose up -d --build
```

each task defined in Makefile.toml can be excuted by `cargo make`

```
cargo make run

// run [tasks.run-in-docker]
cargo make run-in-docker
```

Optionally, conducive commands are executable.  

```

// you can debug with...
cargo mak --loglevel verbose run
```

### recommended packages

cargo-make

```
cargo install --force cargo-make
```