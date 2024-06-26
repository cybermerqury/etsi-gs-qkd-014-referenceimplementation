![build workflow](https://github.com/cybermerqury/etsi-gs-qkd-014-referenceimplementation/actions/workflows/build.yml/badge.svg)

# Description

This project provides a reference implementation to the
[ETSI GS QKD 014 v1.1.1](https://www.etsi.org/deliver/etsi_gs/QKD/001_099/014/01.01.01_60/gs_QKD014v010101p.pdf)
standard.

# Installation

## Reference OS

```
Ubuntu 22.04.1 LTS
```

This implementation has been developed and tested on Ubuntu 22.04.1 LTS.
While no OS specific components are present in this implementation, no
guarantees are made that it works on other OSs.
If you encounter issues deploying this implementation on another OS, reach out
and we will try our best to make it work on your setup.

## Required OS packages

The server requires the following packages to be installed:
* pkg-config
* libssl-dev

On Ubuntu, these can be installed using
```bash
sudo apt install pkg-config libssl-dev
```
## Database management packages

The [SQLx](https://docs.rs/sqlx/latest/sqlx/) rust library is used to handle database
operations, including migrations.
SQLx has a dependency on the `libpq` library which needs to be installed
before installing the `sqlx-cli` utility.
On Ubuntu, the `libpq` library can be installed using
```bash
sudo apt install libpq-dev
```
To install the SQLx command line interface, run the command
```bash
cargo install sqlx-cli
```
To confirm that installation was successful, run the command
```bash
sqlx --version
```
The output should be similar to
```
sqlx-cli 0.7.4
```

# Set up

## Start database

Run

```bash
make db_start
```

This will create and launch a docker container running a postgres database running at `DATABASE_URL`.

## Diesel migrations

To run the diesel migration SQL scripts and set-up the database, run the
following command:

```bash
make db_migration
```

This command will execute the `up.sql` scripts in the `migrations` folder that
have not yet been executed on the database.

# ETSI QKD 014 Standard

The ETSI QKD 014 standard requires that mutual TLS (mTLS) authentication is
performed.
This requires both the server and the client to authenticate each other.
Most commonly, only the server's certificate is authenticated by the client,
with the server doing no such authentication.
Due to the sensitivity of the application, mTLS is required.

## Secure Application Entity (SAE)

The SAE, is referred to the client, because it is the entity that will be
issuing the requests.

## Key Management Entity (KME)

The KME, is the application that this server aims to emulate.
It will be referred to as the server.

# Certificates

A root CA will be generated and used to sign both the SAE (client) and KME
(server) certificates.
This is to emulate a certificate that is signed by a trusted Certificate
Authority (CA).

## Generate a self-signed certificate

The generation of a certificate involves the
* Generation of a private key,
* Generation of a certificate signing request, and
* Signing of the certificate.

The generation of a private key and self-signed certificate can be done using a
single command.
The `nodes` option is set such that the private key is not passphrase protected.
```bash
openssl req -x509           \
    -newkey rsa:4096        \
    -days 3650              \
    -nodes                  \
    -keyout test.key        \
    -out test.crt           \
    -subj "/CN=test_123456" \
    -addext "subjectAltName=IP:127.0.0.1"
```

## Certificate generation

All the below commands are part of the included `makefile` in the `certs`
directory.
Certificates can be generated by issuing the command
```bash
make certs
```

### Root CA

Generate a password protected root certificate.
```bash
openssl req -x509                                            \
    -newkey rsa:4096                                         \
    -days 365                                                \
    -subj "/C=MT/CN=www.merqury.eu/O=Merqury\ Cybersecurity" \
    -keyout root.key                                         \
    -out root.crt
```

### KME Certificate

Generate the KME private key and a Certificate Signing Request (CSR).
```bash
openssl req                                          \
    -newkey rsa:4096                                 \
    -nodes                                           \
    -days 365                                        \
    -subj "/C=MT/O=Key Management Ltd/CN=kme_123456" \
    -keyout kme.key                                  \
    -out kme.csr
```

Create an extensions file to specify the alternative names
```bash
echo "subjectAltName = IP:127.0.0.1" >> kme.ext
```

Sign the certificate using the root CA's key
```bash
openssl x509 -req    \
    -in kme.csr      \
    -CA root.crt     \
    -CAkey root.key  \
    -set_serial 01   \
    -days 365        \
    -extfile kme.ext \
    -out kme.crt
```

### SAE Certificate

Generate the sae private key and a Certificate Signing Request (CSR).
```bash
openssl req                                              \
    -newkey rsa:4096                                     \
    -nodes                                               \
    -days 365                                            \
    -subj "/C=MT/O=Secure Application Ltd/CN=sae_123456" \
    -keyout sae.key                                      \
    -out sae.csr
```
Create an extensions file to specify the alternative names
```bash
echo "subjectAltName = IP:127.0.0.1" >> sae.ext
```

Sign the certificate using the root CA's key
```bash
openssl x509 -req    \
    -in sae.csr      \
    -CA root.crt     \
    -CAkey root.key  \
    -set_serial 01   \
    -days 365        \
    -extfile sae.ext \
    -out sae.crt
```
## Utilities

To view the private key contents
```bash
openssl pkey -in test.key -text -noout
```

To extract the public key from the private key
```bash
openssl pkey -in test.key -pubout -out server-public.key
```

To view the Certificate Signing Request (CSR) contents
```bash
openssl req -text -in test.csr -noout
```

To examine the certificate
```bash
openssl x509 -text -in test.crt -noout
```

# Environment variables

| Variable name                       | Description                           |
|-------------------------------------|---------------------------------------|
|ETSI_014_REF_IMPL_IP_ADDR            | Ip address the server will bind to.   |
|ETSI_014_REF_IMPL_PORT_NUM           | The port number the server listens on.|
|ETSI_014_REF_IMPL_DB_URL             | Database URL.                         |
|ETSI_014_REF_IMPL_TLS_ROOT_CRT       | Root CA certificate.                  |
|ETSI_014_REF_IMPL_TLS_PRIVATE_KEY    | Private key.                          |
|ETSI_014_REF_IMPL_TLS_CER            | TLS certificate (public key).         |
|ETSI_014_REF_IMPL_NUM_WORKER_THREADS | Number of threads the server will use.|

# Examples

The `examples` folder contains multiple bash scripts that show the user how to
launch and interact with the reference implementation.
The `enc_keys.sh` and `dec_keys.sh` scripts send requests using `curl` to the
web service.
The `run_server.sh` script launches a server instance with the required
environment variables.

The Makefile allows the user to run these scripts in a coordinated way.

```bash
make run_server
```

Runs the server using the same database created with `make db_start`.

```bash
make get_enc_key
```
Retrieves an encryption key.

```bash
make post_enc_key
```

Retrieves 3 encryption keys.

```bash
make get_dec_key KEY=XXX
```

Retrieves the decryption key with key-id `XXX`.

```bash
make post_get_key KEYS='XXX YYY ZZZ ...'
```

Retrieves the decryption keys with IDs `XXX`, `YYY`, `ZZZ` and so on.

# License

© 2023 Merqury Cybersecurity Ltd.

This project is licensed under the
[GNU Affero General Public License v3.0 only](https://www.gnu.org/licenses/agpl-3.0.txt).
If you would like to use this product under a different license, kindly contact
us on [info@merqury.eu](mailto:info@merqury.eu).

# Acknowledgements

This software has been developed in the projects EQUO (European QUantum
ecOsystems) which is funded by the European Commission in the Digital Europe
Programme under the grant agreement No 101091561 and PRISM (Physical Security
for Public Infrastructure in Malta) which is co-funded by the European Union
under the Digital Europe Programme grant agreement number 101111875.
