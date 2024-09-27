# SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: AGPL-3.0-only

FROM ubuntu:22.04

RUN apt update     && \
    apt upgrade -y && \
    apt install -y    \
    build-essential   \
    curl              \
    libpq-dev         \
    libssl-dev        \
    pkg-config        \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs \
    | sh -s -- --default-toolchain=1.80.1 -y

# Mount resources for compilation
RUN mkdir -p /usr/src/merqury/etsi_014_ref_impl
WORKDIR /usr/src/merqury/etsi_014_ref_impl

RUN --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=sql,target=sql \
    --mount=type=bind,source=.sqlx,target=.sqlx \
    <<EOF
set -e
SQLX_OFFLINE=true ${HOME}/.cargo/bin/cargo build --locked --release
EOF

# Create certificates folder
RUN mkdir -p /usr/certs

ENTRYPOINT [ "/usr/src/merqury/etsi_014_ref_impl/target/release/etsi_gs_qkd_014_referenceimplementation" ]
