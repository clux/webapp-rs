# Rust microservice setup with rocket and postgres diesel
[![CircleCI](https://circleci.com/gh/clux/webapp-rs/tree/master.svg?style=shield)](https://circleci.com/gh/clux/webapp-rs/tree/master)
[![docker pulls](https://img.shields.io/docker/pulls/clux/webapp-rs.svg)](
https://hub.docker.com/r/clux/webapp-rs/)
[![docker image info](https://images.microbadger.com/badges/image/clux/webapp-rs.svg)](http://microbadger.com/images/clux/webapp-rs)
[![docker tag](https://images.microbadger.com/badges/version/clux/webapp-rs.svg)](https://hub.docker.com/r/clux/webapp-rs/tags/)

A JSON API microservice with database backing all running from a [<7MB container](./Dockerfile).

## Developing Easily
Install dependencies, setup `rustup` env, then start a dev postgres with credentials:

```sh
sudo pacman -S postgresql-libs # or distro equivalent
make setup
source env.sh
make db # runs postgres in a container
```

then run and test in parallel with:

```sh
make run
make test
```

This workflow does not require any docker builds, nor any local installation of postgres (just the client library to talk to the containerised version).

## Docker only
You can develop and test production equivalents without rust, without local postgres, without postgres libs, and without diesel-cli locally:

This is the production equivalent flow:

```sh
# Build the app with clux/muslrust
make compile
# Run the app + db with docker-compose
source env.sh
make up
# Use clux/diesel-cli to run migrations
make migrate
# Verify
make test
```

If you think about it properly for kubernetes, these steps work out nicely:

The migration step can be an init step before the app container starts, but after the postgres container has initialised. Explicitly it's only doing `diesel migration run` in the [diesel-cli](https://github.com/clux/diesel-cli) container. This maps perfectly onto [Init Containers on Kubernetes](https://kubernetes.io/docs/concepts/workloads/pods/init-containers/).

The compile step could be baked into a multistep docker build to be able to simple `make compose` instead of `make compile` first. However, doing efficient caching of slow rust builds for this is complicated. Even if you were using compose, you wouldn't use a compose file with a `build` key in production anyway. CI would build and push your production image after testing, and the orchestrator would compose.

## Caveats
**NB:** Static linkage build used in docker build currently requires a patched `pq-sys` crate for muslrust build (see [clux/muslrust#19](https://github.com/clux/muslrust/issues/19)).
