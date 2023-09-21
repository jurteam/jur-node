# Speed builds up

There are a few things that one could do to speed up
the builds.

### Caching: sccache

Firstly, install sccache:

```shell
cargo install sccache
```

Set the RUSTC_WRAPPER environment variable:

```shell
echo "export RUSTC_WRAPPER=sccache" >> ~/.cargo/env
```

### Turn fsync() syscalls into no-ops: libeatmydata

Additionally, one could install [libeatmydata](https://www.flamingspork.com/projects/libeatmydata/).

It is distributed by the vast majority of Linux distributions. For instance, on Debian
one could do:

```shell
sudo apt install libeatmydata
```

On MacOS, it could be found in Homebrew's taps:

```shell
brew install libeatmydata
```

Finally, just wrap `cargo build` calls with the program `eatmydata`:

```shell
eatmydata cargo build --dev
```
