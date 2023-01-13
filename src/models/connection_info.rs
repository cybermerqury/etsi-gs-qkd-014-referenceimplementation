// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use actix_web::HttpRequest;

use crate::error::Error;
use log::error;

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub sae_id: String,
}

impl ConnectionInfo {
    pub fn new(request: &HttpRequest) -> Result<Self, Error> {
        match request.conn_data::<ConnectionInfo>() {
            Some(conn_info) => Ok(conn_info.clone()),
            None => {
                error!("Failed to extract 'sae_id' from request");
                Err(Error::internal_server_error())
            }
        }
    }
}
