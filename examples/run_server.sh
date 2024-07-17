#!/usr/bin/env bash
# SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: AGPL-3.0-only

script_dir="$( cd "$(dirname "$0")" || exit 1; pwd -P )"
ETSI_014_REF_IMPL_TLS_ROOT_CRT=${CERTS_DIR}/root.crt \
ETSI_014_REF_IMPL_TLS_PRIVATE_KEY=${CERTS_DIR}/kme_001.key \
ETSI_014_REF_IMPL_TLS_CERT=${CERTS_DIR}/kme_001.crt \
SQLX_OFFLINE=true cargo run
