NAME=webapp-rs
VERSION=$(shell git rev-parse HEAD)
SEMVER_VERSION=$(shell grep version Cargo.toml | awk -F"\"" '{print $$2}' | head -n 1)
REPO=clux
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
	rustup override set $$(cat .rustup)

test:
	./test.sh

up: has_secrets
	docker-compose up -d

compose: has_secrets
	@echo "Slow-automatic compose and migrate on CI"
	docker-compose up -d db
	sleep 10
	make migrate
	docker-compose up -d web
	docker-compose logs web

migrate:
	docker run --rm \
		-v "$$PWD:/volume" \
		-w /volume \
		--net=host \
		-e DATABASE_URL="postgres://$${POSTGRES_USER}:$${POSTGRES_PASSWORD}@$${POSTGRES_DB_URL}/$${POSTGRES_DB}" \
		-it clux/diesel-cli diesel migration run

run: has_secrets has_postgres
	cargo run

compile:
	docker run --rm \
		-v cargo-cache:/root/.cargo \
		-v $$PWD:/volume \
		-w /volume \
		-it clux/muslrust \
		cargo build --release
	sudo chown $$USER:$$USER -R target
	strip target/x86_64-unknown-linux-musl/release/webapp
	mv target/x86_64-unknown-linux-musl/release/webapp .

build:
	@echo "Reusing built binary in current directory from make compile"
	@ls -lah ./webapp
	docker build -t $(REPO)/$(NAME):$(VERSION) .

tag-latest: build
	docker tag $(REPO)/$(NAME):$(VERSION) $(REPO)/$(NAME):latest
	docker push $(REPO)/$(NAME):latest

tag-semver: build
	if curl -sSL https://registry.hub.docker.com/v1/repositories/$(REPO)/$(NAME)/tags | jq -r ".[].name" | grep -q $(SEMVER_VERSION); then \
		echo "Tag $(SEMVER_VERSION) already exists" && exit 1 ;\
	fi
	docker tag $(REPO)/$(NAME):$(VERSION) $(REPO)/$(NAME):$(SEMVER_VERSION)
	docker push $(REPO)/$(NAME):$(SEMVER_VERSION)
