// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use serde::Serialize;

#[derive(Serialize)]
pub struct Status {
    #[serde(rename = "source_KME_ID")]
    pub source_kme_id: String,
    #[serde(rename = "target_KME_ID")]
    pub target_kme_id: String,
    #[serde(rename = "master_SAE_ID")]
    pub master_sae_id: String,
    #[serde(rename = "slave_SAE_ID")]
    pub slave_sae_id: String,
    pub key_size: i32,
    pub stored_key_count: i32,
    pub max_key_count: i32,
    pub max_key_per_request: i32,
    pub max_key_size: i32,
    pub min_key_size: i32,
    #[serde(rename = "max_SAE_ID_count")]
    pub max_sae_id_count: i32,
}
