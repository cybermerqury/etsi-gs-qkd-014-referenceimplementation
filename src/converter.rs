// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Error;
use actix_web::http::StatusCode;
use base64::Engine;
use log::error;
use serde::Deserialize;

pub fn to_json<'a, T>(json_text: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    match serde_json::from_str::<T>(json_text) {
        Ok(parsed_json) => Ok(parsed_json),
        Err(e) => {
            error!(
                "Failed to parse JSON. \
                 Error: {:?} Received body: {}",
                e, json_text
            );
            Err(Error::new(
                StatusCode::BAD_REQUEST,
                "Malformed JSON supplied",
            ))
        }
    }
}

pub fn to_uuid(text: &str) -> Result<uuid::Uuid, Error> {
    match uuid::Uuid::try_parse(text) {
        Ok(value) => Ok(value),
        Err(e) => {
            error!("Failed to convert {} to UUID. Error: {:?}", text, e);
            Err(Error::new(
                StatusCode::BAD_REQUEST,
                "Invalid key id supplied",
            ))
        }
    }
}

pub fn to_base64(key: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(key)
}
