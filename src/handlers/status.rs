// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::{
    common::CustomResult,
    default::DEFAULT,
    models::{connection_info::ConnectionInfo, status::Status},
};

#[get("/api/v1/keys/{slave_sae_id}/status")]
pub async fn get(
    request: HttpRequest,
    slave_sae_id: web::Path<String>,
) -> impl Responder {
    service_request(&request, slave_sae_id.to_string())
}

fn service_request(
    request: &HttpRequest,
    slave_sae_id: String,
) -> CustomResult {
    Ok(HttpResponse::Ok().json(Status {
        source_kme_id: String::from(DEFAULT.src_kme_id),
        target_kme_id: String::from(DEFAULT.dst_kme_id),
        master_sae_id: ConnectionInfo::new(request)?.sae_id,
        slave_sae_id,
        key_size: DEFAULT.key_size,
        stored_key_count: 0,
        max_key_count: 0,
        max_key_per_request: 0,
        max_key_size: 0,
        min_key_size: 0,
        max_sae_id_count: 0,
    }))
}
