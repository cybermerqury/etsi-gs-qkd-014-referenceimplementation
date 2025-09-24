// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
mod common;
mod config;
mod converter;
mod db;
mod default;
mod error;
mod handlers;
mod models;
mod ops;

use actix_web::{middleware::Logger, App, HttpServer};
use config::CONFIG;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    db::establish_connection().await.expect("Could not connect to database");

    info!("Server starting on {}:{}", CONFIG.ip_addr, CONFIG.port_num);

    rustls::crypto::aws_lc_rs::default_provider().install_default().expect(
        "Should have installed the expected crypto provider successfully.",
    );

    let tls_config = ops::server::build_tls_configuration();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            // status
            .service(handlers::status::get)
            // enc_keys
            .service(handlers::enc_keys::get)
            .service(handlers::enc_keys::post)
            // dec_keys
            .service(handlers::dec_keys::get)
            .service(handlers::dec_keys::post)
    })
    .on_connect(ops::server::add_cert_info_to_request_body)
    .workers(CONFIG.num_workers.into())
    .bind_rustls_0_23((CONFIG.ip_addr.clone(), CONFIG.port_num), tls_config)?
    .run()
    .await
}
