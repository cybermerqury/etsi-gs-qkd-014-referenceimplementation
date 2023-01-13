// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use actix_web::{error, http::StatusCode, HttpResponse};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    message: String,
    status_code: StatusCode,
}

impl Error {
    pub fn new(status_code: StatusCode, msg: &str) -> Self {
        Self {
            message: msg.to_string(),
            status_code,
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            message: "".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            message: "".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn bad_request(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
            status_code: StatusCode::BAD_REQUEST,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error - Status Code: {} Message: {}",
            self.status_code, self.message
        )
    }
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        if self.message.is_empty() {
            HttpResponse::build(self.status_code).finish()
        } else {
            HttpResponse::build(self.status_code).json(json!({
                "message": self.message
            }))
        }
    }

    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}
