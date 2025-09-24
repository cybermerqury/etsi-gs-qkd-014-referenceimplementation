// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use actix_web::{error, http::StatusCode, HttpResponse};
use serde_json::json;
use std::fmt::{self, Display};

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

#[derive(Debug, Clone)]
pub enum ServerError {
    MissingData(&'static str),
    Rustls(rustls::Error),
    RustlsClientVerifier(rustls::client::VerifierBuilderError),
    X509(x509_parser::error::X509Error),
    X509Parsing(x509_parser::nom::Err<x509_parser::prelude::X509Error>),
    VerificationFailed(String),
}

impl From<rustls::Error> for ServerError {
    fn from(value: rustls::Error) -> Self {
        Self::Rustls(value)
    }
}

impl From<rustls::client::VerifierBuilderError> for ServerError {
    fn from(value: rustls::client::VerifierBuilderError) -> Self {
        Self::RustlsClientVerifier(value)
    }
}

impl From<x509_parser::error::X509Error> for ServerError {
    fn from(value: x509_parser::error::X509Error) -> Self {
        Self::X509(value)
    }
}

impl From<x509_parser::nom::Err<x509_parser::prelude::X509Error>>
    for ServerError
{
    fn from(
        value: x509_parser::nom::Err<x509_parser::prelude::X509Error>,
    ) -> Self {
        Self::X509Parsing(value)
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingData(msg) => write!(f, "Missing data or file: {msg}"),
            Self::Rustls(e) => {
                write!(f, "TLS error during server config or general use: {e}")
            }
            Self::RustlsClientVerifier(e) => {
                write!(f, "TLS error at client cert verifier: {e}")
            }
            Self::X509(e) => write!(f, "X509 parsing error. Error: {e}"),
            Self::X509Parsing(e) => write!(f, "X509 parsing error. Error: {e}"),
            Self::VerificationFailed(e) => {
                write!(f, "Verification failed: {e}")
            }
        }
    }
}

impl std::error::Error for ServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MissingData(_) => None,
            Self::Rustls(e) => Some(e),
            Self::RustlsClientVerifier(e) => Some(e),
            Self::X509(e) => Some(e),
            Self::X509Parsing(e) => Some(e),
            Self::VerificationFailed(_) => None,
        }
    }
}
