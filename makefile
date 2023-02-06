# SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: AGPL-3.0-only

.PHONY: db_container db_start db_stop db_clean_container db_clean_container_and_data build build_release clean

db_container:
	docker-compose up --no-start

db_start:
	docker-compose start

db_stop:
	docker-compose stop

db_clean_container:
	docker-compose down

db_clean_container_and_data:
	docker-compose down -v --rmi local

build:
	cargo build

build_release:
	cargo build --release

clean:
	cargo clean
