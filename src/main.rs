// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
#[macro_use]
extern crate lazy_static;

mod common;
mod config;
mod converter;
mod db;
mod default;
mod error;
mod handlers;
mod models;
mod ops;
mod schema;

use actix_web::{middleware::Logger, App, HttpServer};
use config::CONFIG;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    CONFIG.init();
    db::establish_connection().expect("Could not connect to database");

    info!("Server starting on {}:{}", CONFIG.ip_addr, CONFIG.port_num);

    let tls_config = match ops::server::build_tls_configuration() {
        Ok(tls_config) => tls_config,
        Err(e) => {
            panic!("Failed to build the tls configuration. Error: {:?}", e);
        }
    };

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            // enc_keys
            .service(handlers::enc_keys::get)
            .service(handlers::enc_keys::post)
            // dec_keys
            .service(handlers::dec_keys::get)
            .service(handlers::dec_keys::post)
    })
    .on_connect(ops::server::add_cert_info_to_request_body)
    .workers(2)
    .bind_openssl((CONFIG.ip_addr.clone(), CONFIG.port_num), tls_config)?
    .run()
    .await
}
