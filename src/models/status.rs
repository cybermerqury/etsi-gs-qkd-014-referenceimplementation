// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StatusExtention;

#[derive(Serialize, Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status_extension: Option<StatusExtention>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_status() {
        let status = Status {
            source_kme_id: "AAAABBBBCCCCDDDD".to_string(),
            target_kme_id: "EEEEFFFFGGGGHHHH".to_string(),
            master_sae_id: "IIIIJJJJKKKKLLLL".to_string(),
            slave_sae_id: "MMMMNNNNOOOOPPPP".to_string(),
            key_size: 352,
            stored_key_count: 25000,
            max_key_count: 100000,
            max_key_per_request: 128,
            max_key_size: 1024,
            min_key_size: 64,
            max_sae_id_count: 0,
            status_extension: None,
        };

        let data = r#"
        {
            "source_KME_ID": "AAAABBBBCCCCDDDD",
            "target_KME_ID": "EEEEFFFFGGGGHHHH",
            "master_SAE_ID": "IIIIJJJJKKKKLLLL",
            "slave_SAE_ID": "MMMMNNNNOOOOPPPP",
            "key_size": 352,
            "stored_key_count": 25000,
            "max_key_count": 100000,
            "max_key_per_request": 128,
            "max_key_size": 1024,
            "min_key_size": 64,
            "max_SAE_ID_count": 0
        }"#;

        let de_status: Status = serde_json::from_str(data).unwrap();
        assert_eq!(serde_json::to_string(&status).unwrap(), serde_json::to_string(&de_status).unwrap());
    }
}
