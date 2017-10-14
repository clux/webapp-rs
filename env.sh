#!/bin/bash

export POSTGRES_USER=clux
export POSTGRES_PASSWORD=foobar
export POSTGRES_DB_URL=0.0.0.0
export POSTGRES_DB=postgres
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_DB_URL}/${POSTGRES_DB}"
