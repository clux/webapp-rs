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

```sh
make prod-build
make prod-run
```
