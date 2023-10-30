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
    | sh -s -- --default-toolchain=1.72.1 -y

# Copy source files
RUN mkdir -p /usr/src/merqury/etsi_014_ref_impl
WORKDIR /usr/src/merqury/etsi_014_ref_impl

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN ${HOME}/.cargo/bin/cargo build --release

# Create certificates folder
RUN mkdir -p /usr/certs

ENTRYPOINT [ "/usr/src/merqury/etsi_014_ref_impl/target/release/etsi_gs_qkd_014_referenceimplementation" ]
