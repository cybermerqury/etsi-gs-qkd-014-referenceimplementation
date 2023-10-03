#!/usr/bin/env bash
# SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: AGPL-3.0-only

script_dir="$( cd "$(dirname "$0")" || exit 1; pwd -P )"
certs_dir=$CERTS_DIR

ETSI_014_REF_IMPL_IP_ADDR=127.0.0.1 \
ETSI_014_REF_IMPL_PORT_NUM=8443 \
ETSI_014_REF_IMPL_DB_URL=${DATABASE_URL} \
ETSI_014_REF_IMPL_TLS_ROOT_CRT=${certs_dir}/root.crt \
ETSI_014_REF_IMPL_TLS_PRIVATE_KEY=${certs_dir}/kme_001.key \
ETSI_014_REF_IMPL_TLS_CERT=${certs_dir}/kme_001.crt \
ETSI_014_REF_IMPL_NUM_WORKER_THREADS=2 \
cargo run
