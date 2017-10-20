SHELL := /bin/bash

has_secrets:
	@[[ $$POSTGRES_DB ]] || (echo "source env.sh first"; exit 2)

no_postgres:
	@[ -z "$$(docker ps -q -f ancestor="postgres:9.6")" ] || (echo "db running"; exit 2)
has_postgres:
	@[ -n "$$(docker ps -q -f ancestor="postgres:9.6")" ] || (echo "db not running"; exit 2)

db: has_secrets	no_postgres
	@echo "Starting postgres container"
	docker run --rm -d \
		-p "5432:5432" \
		--expose 5432 \
		-e POSTGRES_DB="$$POSTGRES_DB" \
		-e POSTGRES_PASSWORD="$$POSTGRES_PASSWORD" \
		-e POSTGRES_USER="$$POSTGRES_USER" \
		-it postgres:9.6
	sleep 5 # wait for postgres to initialize
	diesel migration run # seed with tables in migrations dir
	diesel print-schema > src/schema.rs

stop:
	@docker ps -aq | xargs -r docker rm -f
	@pkill webapp || true

setup:
	cargo install diesel_cli --no-default-features --features postgres
	rustup override set nightly

run: has_secrets has_postgres
	cargo run

test:
	./test.sh

prod-build:
	@echo "Starting production docker build"
	docker run \
		-v cargo-cache:/root/.cargo \
		-v "$$PWD:/volume" \
		--rm -it clux/muslrust cargo build --release
	sudo chown $$USER:$$USER -R target
	strip target/x86_64-unknown-linux-musl/release/webapp
	docker-compose build

prod-run: has_secrets
	@echo "Starting docker compose"
	@docker network inspect webapp || docker network create webapp
	docker-compose up
