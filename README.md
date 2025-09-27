# EXAMPLE

```bash
maturin develop -m wrapped_example_rs/Cargo.toml --release
```


```bash
maturin build -m wrapped_example_rs/Cargo.toml \
      --release --strip --out dist \
      --compatibility musllinux_1_2
```