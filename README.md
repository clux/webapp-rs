# Rust microservice setup with rocket and diesel
Provided as a proof of concept.

## Setup
Initial dependencies can be installed and verified using:

```sh
sudo pacman -S postgresql-libs # need libpq locally for dev builds and diesel_cli
cargo install diesel_cli --no-default-features --features postgres # migration tool
rustup override set nightly # for rocket
```

Replace `pacman` with the corresponding package manager on your distro.

## Developing
A musl build takes some time, so for a quick iterations, it's easiest to just run debug builds against a seeded, static postgres container:

```sh
source env.sh
docker run --rm -d \
  -p "5432:5432" \
  --expose 5432 \
  -e POSTGRES_DB="$POSTGRES_DB" \
  -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
  -e POSTGRES_USER="$POSTGRES_USER" \
  -it postgres:9.6
sleep 5 # wait for postgres to initialize
diesel migration run # seed with tables in migrations dir
cargo build
```

then iterate with:

```sh
cargo run
```

and test against a running app using a second shell:

```sh
./test.sh
```

## Building
For the production image, build the executable with the [muslrust](https://github.com/clux/muslrust) image, then build the docker images with `docker-compose`:

**THIS DOESN'T WORK YET**

```sh
docker run \
  -v cargo-cache:/root/.cargo \
  -v "$PWD:/volume" -w /volume \
  --rm -it clux/muslrust cargo build --release
chown $USER:$USER -R target
strip target/x86_64-unknown-linux-musl/release/webapp
docker-compose build
```

It can then be run locally using:

```sh
docker network create webapp
docker-compose up
```

