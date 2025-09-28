# EXAMPLE

## Develop

```bash
maturin develop -m wrapped_example_rs/Cargo.toml --release

pip uninstall -y wrapped-example1 wrapped-example wrapped_example wrapped_example_core || true
```

## Prod

```bash
maturin build -m wrapped_example_rs/Cargo.toml \
      --release --strip --out dist \
      --compatibility musllinux_1_2

poetry instal --with prod # install the wheel
poetry install --sync  # remove the installed wheel
```
