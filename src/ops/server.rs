// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use crate::models::connection_info::ConnectionInfo;
use actix_tls::accept::openssl::TlsStream;
use actix_web::dev::Extensions;
use actix_web::rt::net::TcpStream;
use log::debug;
use openssl::error::ErrorStack;
use openssl::nid::Nid;
use openssl::ssl::{
    SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod, SslVerifyMode,
};
use std::any::Any;

pub fn add_cert_info_to_request(connection: &dyn Any, data: &mut Extensions) {
    let tls_socket = connection
        .downcast_ref::<TlsStream<TcpStream>>()
        .expect("Socket should be of type TLSStream.");

    let conn_info = extract_conn_info_from_socket(tls_socket);
    debug!("Extracted connection information: {:?}", &conn_info);

    data.insert(conn_info);
}

pub fn build_tls_configuration() -> Result<SslAcceptorBuilder, ErrorStack> {
    let mut builder = SslAcceptor::mozilla_modern_v5(SslMethod::tls())?;

    builder.set_ca_file("certs/root.crt")?;
    builder.set_private_key_file("certs/kme_001.key", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("certs/kme_001.crt")?;

    builder
        .set_verify(SslVerifyMode::PEER | SslVerifyMode::FAIL_IF_NO_PEER_CERT);

    Ok(builder)
}

fn extract_conn_info_from_socket(
    tls_socket: &TlsStream<TcpStream>,
) -> ConnectionInfo {
    let cert = tls_socket
        .ssl()
        .peer_certificate()
        .expect("Peer certificate should always be provided");

    let mut common_name_entries =
        cert.subject_name().entries_by_nid(Nid::COMMONNAME);

    let common_name_entry = common_name_entries
        .next()
        .expect("Failed to retrieve first common name entry");

    ConnectionInfo {
        sae_id: common_name_entry
            .data()
            .as_utf8()
            .expect("Could not convert common name entry to string")
            .to_string(),
    }
}
