// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Error;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use log::error;
use std::env;

pub fn establish_connection() -> Result<PgConnection, Error> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(e) => {
            error!(
                "Failed to extract the 'DATABASE_URL' from the .env file. Error: {:?}",
                e
            );
            return Err(Error::internal_server_error());
        }
    };

    match PgConnection::establish(&database_url) {
        Ok(conn) => Ok(conn),
        Err(e) => {
            error!("Failed to connect to the database. Error: {:?}", e);
            Err(Error::internal_server_error())
        }
    }
}
