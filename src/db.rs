// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use crate::config::CONFIG;
use crate::error::Error;
use log::error;
use sqlx::PgPool;

pub async fn establish_connection() -> Result<PgPool, Error> {
    match PgPool::connect(&CONFIG.db_url).await {
        Ok(pool) => Ok(pool),
        Err(e) => {
            error!("Failed to connect to the database. Error: {:?}", e);
            Err(Error::internal_server_error())
        }
    }
}
