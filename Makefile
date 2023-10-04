# SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: AGPL-3.0-only

# include ./.env

DATABASE_PORT?=10000
DATABASE_HOST?=127.0.0.1
DATABASE_USER?=db_user
DATABASE_PASSWORD?=db_password
DATABASE_URL?=postgres://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${DATABASE_PORT}/key_store
ETSI_014_REF_IMPL_PORT_NUM?=8443
ETSI_014_REF_IMPL_IP_ADDR?=${DATABASE_HOST}
ETSI_014_REF_IMPL_NUM_WORKER_THREADS?=2
CERTS_DIR?=$(abspath ./certs)

.PHONY:
	db_container
	db_start
	db_stop
	db_migration
	db_clean_container
	db_clean_container_and_data
	build
	build_release
	run_tests
	run_server
	clean


db_container:
	DATABASE_HOST=$(DATABASE_HOST) \
	DATABASE_PORT=$(DATABASE_PORT) \
	DATABASE_USER=$(DATABASE_USER) \
	DATABASE_PASSWORD=$(DATABASE_PASSWORD) \
	docker-compose up --no-start

db_start: db_container
	DATABASE_HOST=$(DATABASE_HOST) \
	DATABASE_PORT=$(DATABASE_PORT) \
	DATABASE_USER=$(DATABASE_USER) \
	DATABASE_PASSWORD=$(DATABASE_PASSWORD) \
	docker-compose start

db_migration:
	DATABASE_URL=$(DATABASE_URL) \
	diesel migration run

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
	ETSI_014_REF_IMPL_PORT_NUM=$(ETSI_014_REF_IMPL_PORT_NUM) \
	ETSI_014_REF_IMPL_IP_ADDR=$(ETSI_014_REF_IMPL_IP_ADDR) \
	ETSI_014_REF_IMPL_NUM_WORKER_THREADS=$(ETSI_014_REF_IMPL_NUM_WORKER_THREADS) \
	ETSI_014_REF_IMPL_DB_URL=$(DATABASE_URL) \
	./examples/run_server.sh

get_enc_key:
	CERTS_DIR=$(CERTS_DIR) \
	ETSI_014_REF_IMPL_PORT_NUM=$(ETSI_014_REF_IMPL_PORT_NUM) \
	ETSI_014_REF_IMPL_IP_ADDR=$(ETSI_014_REF_IMPL_IP_ADDR) \
	./examples/enc_keys.sh GET

post_enc_key:
	CERTS_DIR=$(CERTS_DIR) \
	ETSI_014_REF_IMPL_PORT_NUM=$(ETSI_014_REF_IMPL_PORT_NUM) \
	ETSI_014_REF_IMPL_IP_ADDR=$(ETSI_014_REF_IMPL_IP_ADDR) \
	./examples/enc_keys.sh POST

get_dec_key:
	CERTS_DIR=$(CERTS_DIR) \
	ETSI_014_REF_IMPL_PORT_NUM=$(ETSI_014_REF_IMPL_PORT_NUM) \
	ETSI_014_REF_IMPL_IP_ADDR=$(ETSI_014_REF_IMPL_IP_ADDR) \
	./examples/dec_keys.sh GET $(KEY)

post_dec_key:
	CERTS_DIR=$(CERTS_DIR) \
	ETSI_014_REF_IMPL_PORT_NUM=$(ETSI_014_REF_IMPL_PORT_NUM) \
	ETSI_014_REF_IMPL_IP_ADDR=$(ETSI_014_REF_IMPL_IP_ADDR) \
	./examples/dec_keys.sh POST $(KEYS)
run_tests:
	@cargo test

clean:
	@cargo clean
