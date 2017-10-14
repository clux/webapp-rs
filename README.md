# Rust microservice setup with rocket and diesel
Provided as a proof of concept.

## Developing
A musl builds takes some time, so for a quick iterations, it's easiest to just run debug builds against a static postgres container:

```sh
sudo pacman -S postgresql-libs # need libpq locally for diesel
cargo install diesel_cli --no-default-features --features postgres
source env.sh
docker run --rm -d \
  -p "5432:5432" \
  --expose 5432 \
  -e POSTGRES_DB="$POSTGRES_DB" \
  -e POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
  -e POSTGRES_USER="$POSTGRES_USER" \
  -it postgres:9.6
```

and develop on a debug build in a separate shell

```sh
source env.sh
diesel migration run
cargo run
```

## Building
For the production image, build the executable with the [muslrust](https://github.com/clux/muslrust) image, then build the docker images with `docker-compose`:

**THIS DOESN'T WORK YET**

```sh
docker run \
  -v cargo-cache:/root/.cargo \
  -v "$PWD:/volume" -w /volume \
  --rm -it clux/muslrust cargo build --release
docker-compose build
```

It can then be run locally using:

```sh
docker network create webapp
docker-compose up
```

