FROM ubuntu:22.04

RUN apt update     && \
    apt upgrade -y && \
    apt install -y    \
    libpq-dev         \
    libssl-dev        \
    && rm -rf /var/lib/apt/lists/*

# Create certificates folder
RUN mkdir -p /usr/certs

WORKDIR /bin/
COPY target/release/etsi_gs_qkd_014_referenceimplementation ./

ENTRYPOINT [ "/bin/etsi_gs_qkd_014_referenceimplementation" ]
