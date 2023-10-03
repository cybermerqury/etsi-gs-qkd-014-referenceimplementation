# SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: AGPL-3.0-only

include ./.env

.PHONY: db_container db_start db_stop db_clean_container db_clean_container_and_data build build_release run_tests run_server clean

CERTS_DIR?=$(abspath ./certs)

db_container:
	docker-compose up --no-start

db_start: db_container
	docker-compose start

db_stop:
	docker-compose stop

db_clean_container:
	docker-compose down

db_clean_container_and_data:
	docker-compose down -v --rmi local

build:
	@cargo build --workspace

build_release:
	@cargo build --release --workspace

run_server:
	CERTS_DIR=$(CERTS_DIR) \
	DATABASE_URL=$(DATABASE_URL) \
	./examples/run_server.sh

run_tests:
	@cargo test

clean:
	@cargo clean
