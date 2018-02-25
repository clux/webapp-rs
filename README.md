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

Note that this is a JSON REST API only.

## Docker only
You can develop and test production equivalents without rust, and without local dependencies outside a few docker images and evars.

The general flow is:

```sh
make compile
source env.sh
make compose
make migrate
make test
```

If you think about it properly for kubernetes, these steps work out nicely:

The migration step can be an init step before the app container starts, but after the postgres container has initialised. Explicitly it's only doing `diesel migration run` in the [diesel-cli](https://github.com/clux/diesel-cli) container. This maps perfectly onto [Init Containers on Kubernetes](https://kubernetes.io/docs/concepts/workloads/pods/init-containers/).

The compile step could be baked into a multistep docker build to be able to simple `make compose` instead of `make compile` first. However, doing efficient caching of slow rust builds for this is complicated. Even if you were using compose, you wouldn't use a compose file with a `build` key in production anyway. CI would build and push your production image after testing, and the orchestrator would compose.

## Caveats
**NB:** Static linkage build used in docker build currently requires a patched `pq-sys` crate for muslrust build (see [clux/muslrust#19](https://github.com/clux/muslrust/issues/19)).
