// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use actix_web::HttpRequest;

use crate::error::{Error, ServerError};
use log::error;

#[derive(Debug)]
pub struct ConnectionInfo {
    pub sans: Vec<String>,
    pub sae_id: String,
}

impl ConnectionInfo {
    pub fn try_from_request(request: &HttpRequest) -> Result<&Self, Error> {
        match request.conn_data::<Result<ConnectionInfo, ServerError>>() {
            Some(Ok(conn_info)) => Ok(conn_info),
            Some(Err(e)) => {
                error!("Error retrieving peer connection info. Error: {e}");
                Err(Error::internal_server_error())
            }
            None => {
                error!("Failed to extract 'sae_id' from request");
                Err(Error::internal_server_error())
            }
        }
    }
}
