# SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: AGPL-3.0-only

# Include statements.
include ./.env

# Environment variables.
CURDIR=$(dir $(realpath $(lastword $(MAKEFILE_LIST))))
CERTS_DIR?=$(CURDIR)certs

.PHONY: setup run_server run_server_release stop clean
.PHONY: get_enc_key post_enc_key get_dec_key post_dec_key run_tests
.PHONY: db_start db_migration db_clean_container_and_data
.PHONY: build build_release build_image build_clean
.SILENT:

# ------------------------------------------------------------------------------
# Environment setup.
# ------------------

setup:
	$(MAKE) -C certs certs
	$(MAKE) db_start
	$(MAKE) db_migration

run_server: build
	CERTS_DIR=$(CERTS_DIR) \
	ETSI_014_REF_IMPL_DB_URL=$(DATABASE_URL) \
	ETSI_014_REF_IMPL_IP_ADDR=$(ETSI_014_REF_IMPL_IP_ADDR) \
	ETSI_014_REF_IMPL_NUM_WORKER_THREADS=$(ETSI_014_REF_IMPL_NUM_WORKER_THREADS) \
	ETSI_014_REF_IMPL_PORT_NUM=$(ETSI_014_REF_IMPL_PORT_NUM) \
	./examples/run_server.sh

run_server_release: build_release
	CERTS_DIR=$(CERTS_DIR) \
	ETSI_014_REF_IMPL_DB_URL=$(DATABASE_URL) \
	ETSI_014_REF_IMPL_IP_ADDR=$(ETSI_014_REF_IMPL_IP_ADDR) \
	ETSI_014_REF_IMPL_NUM_WORKER_THREADS=$(ETSI_014_REF_IMPL_NUM_WORKER_THREADS) \
	ETSI_014_REF_IMPL_PORT_NUM=$(ETSI_014_REF_IMPL_PORT_NUM) \
	./examples/run_server.sh "--release"

stop:
	docker compose stop

clean:
	$(MAKE) -C certs clean
	$(MAKE) db_clean_container_and_data
	$(MAKE) build_clean

# ------------------------------------------------------------------------------
# Tests.
# ------

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
	@if [ -z "$(KEY)" ]; then \
		echo "Please set the KEY variable to a valid key UUID."; \
		echo "Example: "; \
		echo "> make get_dec_key KEY='cae8af25-c1a6-4ed9-aff4-b48390665001'"; \
		echo; \
		exit 1; \
	fi

	CERTS_DIR=$(CERTS_DIR) \
	ETSI_014_REF_IMPL_PORT_NUM=$(ETSI_014_REF_IMPL_PORT_NUM) \
	ETSI_014_REF_IMPL_IP_ADDR=$(ETSI_014_REF_IMPL_IP_ADDR) \
	./examples/dec_keys.sh GET $(KEY)

post_dec_key:
	@if [ -z "$(KEYS)" ]; then \
		echo "Please set the KEYS variable to valid key UUID/s."; \
		echo "Example: "; \
		echo "> make get_dec_key KEYS='9738c6d2-8e1c-4bc7-b4aa-f8880abf2cb8 6dfc12b0-0ca8-4e28-8f5c-89168426c76a'"; \
		echo; \
		exit 1; \
	fi

	CERTS_DIR=$(CERTS_DIR) \
	ETSI_014_REF_IMPL_PORT_NUM=$(ETSI_014_REF_IMPL_PORT_NUM) \
	ETSI_014_REF_IMPL_IP_ADDR=$(ETSI_014_REF_IMPL_IP_ADDR) \
	./examples/dec_keys.sh POST $(KEYS)

run_tests:
	SQLX_OFFLINE=true cargo test

# ------------------------------------------------------------------------------
# Database.
# ---------

db_start:
	docker compose up -d

db_migration:
	until docker compose exec key_db pg_isready; \
		do sleep 1; \
	done

	sqlx migrate run --database-url $(DATABASE_URL)

db_clean_container_and_data:
	docker compose down -v --rmi local

# ------------------------------------------------------------------------------
# Build resources.
# ----------------

build:
	SQLX_OFFLINE=true cargo build --workspace

build_release:
	SQLX_OFFLINE=true cargo build --release --workspace

build_image: build_release
	docker build -t merqury/etsi_014_ref_impl:1.1.1 -f Dockerfile .

build_clean:
	cargo clean

# ------------------------------------------------------------------------------
