# Webassembly and rust

- Install `cargo-wasi`

```shell
cargo install cargo-wasi
```

- Install wasmtime (a wasm runtime)

```shell
curl https://wasmtime.dev/install.sh -sSf | bash
# you'll probably need to resource your shell or open a new shell for the wasi commands to become available on the shell path
```

- Create a new cargo project

- Run project with the new wasi command

```shell
cargo wasi run
```