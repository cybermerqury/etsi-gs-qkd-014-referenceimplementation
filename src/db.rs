// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use crate::config::CONFIG;
use crate::error::Error;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use log::error;

pub fn establish_connection() -> Result<PgConnection, Error> {
    match PgConnection::establish(&CONFIG.db_url) {
        Ok(conn) => Ok(conn),
        Err(e) => {
            error!("Failed to connect to the database. Error: {:?}", e);
            Err(Error::internal_server_error())
        }
    }
}
