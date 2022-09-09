#!/usr/bin/env bash

set -euo pipefail

dropdb -U postgres -h localhost dvdrental --force || true
createdb -U postgres -h localhost dvdrental
PG_PASSWORD=postgres pg_restore --no-privileges --no-owner -e -U postgres -h localhost -d dvdrental ./dvdrental.tar
