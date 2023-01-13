// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use crate::{
    common::CustomResult, converter, error::Error,
    models::connection_info::ConnectionInfo, ops::key::get_multiple_keys,
};
use actix_web::{
    get, post,
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use log::error;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct RequestParams {
    #[serde(rename = "key_IDs")]
    key_ids: Vec<RequestParamsElement>,
}

#[derive(Deserialize, Debug)]
pub struct RequestParamsElement {
    #[serde(rename = "key_ID")]
    key_id: String,
}

#[get("/api/v1/keys/{master_sae_id}/dec_keys")]
pub async fn get(
    request: HttpRequest,
    master_sae_id: web::Path<String>,
    request_params: Query<RequestParamsElement>,
) -> impl Responder {
    service_request(
        &request,
        &RequestParams {
            key_ids: vec![request_params.into_inner()],
        },
        master_sae_id.to_string(),
    )
}

#[post("/api/v1/keys/{master_sae_id}/dec_keys")]
pub async fn post(
    request: HttpRequest,
    master_sae_id: web::Path<String>,
    request_body: String,
) -> impl Responder {
    let params: RequestParams = match converter::to_json(&request_body) {
        Ok(parsed_params) => parsed_params,
        Err(e) => {
            error!("Parsing JSON failed");
            return Err(e);
        }
    };

    service_request(&request, &params, master_sae_id.to_string())
}

fn service_request(
    request: &HttpRequest,
    params: &RequestParams,
    master_sae_id: String,
) -> CustomResult {
    let requested_key_ids = validate_and_parse_parameters(params)?;
    let slave_sae_id = &ConnectionInfo::new(request)?.sae_id;

    validate_sae_ids(&master_sae_id, slave_sae_id)?;

    let keys =
        get_multiple_keys(&requested_key_ids, &master_sae_id, slave_sae_id)?;

    Ok(HttpResponse::Ok().json(json!({ "keys": keys })))
}

fn validate_and_parse_parameters(
    params: &RequestParams,
) -> Result<Vec<uuid::Uuid>, Error> {
    let mut key_ids: Vec<uuid::Uuid> = Vec::with_capacity(params.key_ids.len());

    for key_element in &params.key_ids {
        key_ids.push(converter::to_uuid(&key_element.key_id)?);
    }

    Ok(key_ids)
}

fn validate_sae_ids(
    master_sae_id: &str,
    slave_sae_id: &str,
) -> Result<(), Error> {
    if slave_sae_id == master_sae_id {
        return Err(Error::bad_request(
            "The 'master_sae_id' and 'slave_sae_id' cannot be equal",
        ));
    }

    Ok(())
}
