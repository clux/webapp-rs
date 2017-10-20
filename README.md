# Rust microservice setup with rocket and postgres diesel
Provided as a proof of concept.

## Developing
Install dependencies, setup `rustup` env, then start a dev postgres with credentials:

```sh
sudo pacman -S postgresql-libs # or distro equivalent
make setup
source env.sh
make db
```

then run and test in parallel with:

```sh
make run
make test
```

## Production
NB: this currently requires a patched `pq-sys` crate for muslrust build (see [clux/muslrust#19](https://github.com/clux/muslrust/issues/19))

```sh
make prod-build
source env.sh
make prod-run
```

The first time you setup the compose you need to also:

```sh
source env.sh
diesel migration run
```

but after that `make test` will succeed.
