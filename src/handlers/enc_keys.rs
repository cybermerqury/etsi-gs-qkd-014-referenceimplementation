// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use actix_web::{
    get, post,
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use log::error;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashSet;

use crate::{
    common::CustomResult, converter, error::Error,
    models::connection_info::ConnectionInfo, ops,
};

const DEFAULT_SIZE: i32 = 1024;
const DEFAULT_NUM_KEYS: i32 = 1;

#[derive(Deserialize, Debug)]
pub struct RequestParams {
    number: Option<i32>,
    size: Option<i32>,
    #[serde(rename = "additional_slave_SAE_IDs")]
    additional_slave_sae_ids: Option<Vec<String>>,
}

impl RequestParams {
    fn empty() -> Self {
        Self {
            number: None,
            size: None,
            additional_slave_sae_ids: None,
        }
    }
}

#[get("/api/v1/keys/{slave_sae_id}/enc_keys")]
pub async fn get(
    request: HttpRequest,
    slave_sae_id: web::Path<String>,
) -> impl Responder {
    let params =
        match Query::<RequestParams>::from_query(request.query_string()) {
            Ok(parsed_params) => parsed_params,
            Err(e) => {
                error!("{:?}", e);
                return Err(Error::bad_request(
                    "Invalid query parameters supplied.",
                ));
            }
        };

    service_request(&request, &params, slave_sae_id.to_string())
}

#[post("/api/v1/keys/{slave_sae_id}/enc_keys")]
pub async fn post(
    request: HttpRequest,
    slave_sae_id: web::Path<String>,
    request_body: String,
) -> impl Responder {
    let params = match request_body.is_empty() {
        true => RequestParams::empty(),
        false => match converter::to_json(&request_body) {
            Ok(parsed_params) => parsed_params,
            Err(e) => {
                error!("Parsing JSON failed");
                return Err(e);
            }
        },
    };

    service_request(&request, &params, slave_sae_id.to_string())
}

fn service_request(
    request: &HttpRequest,
    params: &RequestParams,
    slave_sae_id: String,
) -> CustomResult {
    let key_size = params.size.unwrap_or(DEFAULT_SIZE);
    let num_keys = params.number.unwrap_or(DEFAULT_NUM_KEYS);

    ops::key::validate_key_size(key_size)?;
    ops::key::validate_num_keys(num_keys)?;

    let master_sae_id = &ConnectionInfo::new(request)?.sae_id;
    let slave_sae_ids =
        validate_and_parse_slave_sae_ids(master_sae_id, &slave_sae_id, params)?;

    let generated_keys = ops::key::generate_random_keys(key_size, num_keys)?;

    ops::key::save_keys(&generated_keys, master_sae_id, &slave_sae_ids)?;

    Ok(HttpResponse::Ok().json(json!({ "keys": generated_keys })))
}

fn validate_and_parse_slave_sae_ids(
    master_sae_id: &str,
    slave_sae_id: &String,
    params: &RequestParams,
) -> Result<Vec<String>, Error> {
    let mut slave_sae_ids: HashSet<&String> =
        HashSet::from([validate_sae_id(slave_sae_id)?]);

    if let Some(slave_ids) = &params.additional_slave_sae_ids {
        if slave_ids.is_empty() {
            return Err(Error::bad_request(
                "Empty 'additional_slave_SAE_IDs' supplied",
            ));
        }

        for slave_id in slave_ids {
            // If the element already exists, the 'insert' function returns
            // false.
            if !slave_sae_ids.insert(validate_sae_id(slave_id)?) {
                return Err(Error::bad_request(
                    "Duplicate slave sae ids found",
                ));
            }
        }
    }

    if slave_sae_ids.contains(&master_sae_id.to_string()) {
        return Err(Error::bad_request(
            "Master sae id found in the list of slave ids",
        ));
    }

    Ok(Vec::from_iter(slave_sae_ids.into_iter().cloned()))
}

fn validate_sae_id(sae_id: &String) -> Result<&String, Error> {
    if sae_id.trim().is_empty() {
        Err(Error::bad_request("Invalid 'sae_id' supplied"))
    } else {
        Ok(sae_id)
    }
}
