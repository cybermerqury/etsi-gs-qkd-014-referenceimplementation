// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
use std::any::Any;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::Arc;

use actix_tls::accept::rustls_0_23::TlsStream;
use actix_web::dev::Extensions;
use actix_web::rt::net::TcpStream;
use log::{debug, trace};
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::WebPkiClientVerifier;
use rustls::{RootCertStore, ServerConfig};
use x509_parser::prelude::{GeneralName, X509Certificate};

use crate::config::CONFIG;
use crate::error::ServerError;
use crate::models::connection_info::ConnectionInfo;

pub fn add_cert_info_to_request_body(
    connection: &dyn Any,
    data: &mut Extensions,
) {
    let Some(tls_socket) = connection.downcast_ref::<TlsStream<TcpStream>>()
    else {
        // This is safe because actix-web docs confirm this is a fixed type based on which `bind_*` function we call during server setup.
        unreachable!(
            "Only TLS connections are supported and only using RusTLS v0.23."
        );
    };

    let conn_info = get_cert_details(tls_socket);
    debug!("Extracted connection information: {:?}", &conn_info);

    data.insert(conn_info);
}

fn get_cert_details(
    tls_conn: &TlsStream<TcpStream>,
) -> Result<ConnectionInfo, ServerError> {
    let cert = get_peer_cert(tls_conn)?;

    trace!("Getting subject alternative names.");

    let sans = raw_sans_to_strings(&cert)?;
    let sae_id = get_local_sae_id(&cert)?;

    Ok(ConnectionInfo { sans, sae_id })
}

fn get_peer_cert<'a>(
    tls_conn: &'a TlsStream<TcpStream>,
) -> Result<X509Certificate<'a>, ServerError> {
    let (_, tls_session) = tls_conn.get_ref();

    let raw_cert = tls_session
        .peer_certificates()
        .and_then(|iter| iter.first())
        .ok_or(ServerError::MissingData(
            "Unable to retrieve peer (client) cert from TLS session.",
        ))?;

    let (_, parsed_cert) =
        x509_parser::parse_x509_certificate(raw_cert.as_ref())?;

    Ok(parsed_cert)
}

fn get_local_sae_id(cert: &X509Certificate) -> Result<String, ServerError> {
    let cn_result =
        cert.subject().iter_common_name().next().ok_or_else(|| {
            ServerError::MissingData(
                "Could not extract SAE ID from Common Name: No CN found.",
            )
        })?;

    let sae_id = cn_result.as_str().map(str::to_string)?;

    Ok(sae_id)
}

fn raw_sans_to_strings(
    cert: &X509Certificate,
) -> Result<Vec<String>, ServerError> {
    fn try_sans_to_ip_string(sans: &GeneralName<'_>) -> Option<String> {
        match sans {
            GeneralName::DNSName(s) => Some(s.to_string()),
            GeneralName::IPAddress(ip) => match ip.len() {
                4 => {
                    Some(Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]).to_string())
                }
                16 => {
                    let bytes: [u8; 16] = ip[0..16].try_into().ok()?;
                    Some(Ipv6Addr::from(bytes).to_string())
                }
                _ => None,
            },
            _ => None,
        }
    }

    let raw_sans =
        cert.subject_alternative_name()?.ok_or(ServerError::MissingData(
            "Subject Alternative Names block in certificate missing.",
        ))?;

    // Only retain the DNS and IPAddress names, in case other types are present.
    let sans = raw_sans
        .value
        .general_names
        .iter()
        .filter_map(try_sans_to_ip_string)
        .collect::<Vec<_>>();

    Ok(sans)
}

pub fn build_tls_configuration() -> ServerConfig {
    debug!("Loading CA cert.");
    let root_store = {
        let mut roots = RootCertStore::empty();
        let cert = CertificateDer::from_pem_file(&CONFIG.root_crt).expect(
            "Expected a to parse and load a root certificate at the given path.",
        );

        roots
            .add(cert)
            .expect("Should have appended the root cert successfully.");

        Arc::new(roots)
    };

    debug!("Loading server cert.");
    let server_cert = CertificateDer::from_pem_file(&CONFIG.public_crt).expect(
        "Expected to find a valid PEM-encoded server certificate file.",
    );

    debug!("Loading server key.");
    let server_key = PrivateKeyDer::from_pem_file(&CONFIG.private_key)
        .expect("Expected to find a valid PEM-encoded private key file.");

    let client_verifier = WebPkiClientVerifier::builder(root_store)
        .build()
        .expect("Expected client cert verifier to be built successfully.");

    let config = ServerConfig::builder()
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(vec![server_cert], server_key)
        .expect("Server TLS config should have built successfully. There might be a mismatch between the private key and server cert.");

    config
}
