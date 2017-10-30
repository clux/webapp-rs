# Rust microservice setup with rocket and postgres diesel
All running from a [<7MB app container](./Dockerfile) along the postgres container.

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
make build
source env.sh
make up
```

Then you can seed its database and test against it with:

```sh
source env.sh
make migration
make test
```
