// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct NewKey {
    pub id: Uuid,
    pub master_sae_id: String,
    pub slave_sae_id: String,
    pub size: i32,
    pub content: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Key {
    #[serde(rename = "key_ID")]
    pub id: Uuid,
    #[serde(rename = "key")]
    pub content: String,
    #[serde(skip)]
    pub size: i32,
}

#[derive(Serialize, Deserialize)]
pub struct KeyResponse {
    pub keys: Vec<Key>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_key(id: &str, content: &str) -> Key {
        let size = content.len() as i32;
        Key {
            id: id.parse().unwrap(),
            content: content.to_string(),
            size,
        }
    }

    #[test]
    fn key_response() {
        let data = r#"
            {
            "keys": [
                {
                    "key_ID": "bc490419-7d60-487f-adc1-4ddcc177c139",
                    "key": "wHHVxRwDJs3/bXd38GHP3oe4svTuRpZS0yCC7x4Ly+s="
                },
                {
                    "key_ID": "0a782fb5-3434-48fe-aa4d-14f41d46cf92",
                    "key": "OeGMPxh1+2RpJpNCYixWHFLYRubpOKCw94FcCI7VdJA="
                },
                {
                    "key_ID": "64a7e9a2-269c-4b2c-832c-5351f3ac5adb",
                    "key": "479G1Osfljpmfa5vn24tdzE5zqv5CafkGxYrLCk8384="
                },
                {
                    "key_ID": "550e8400-e29b-41d4-a716-446655440000",
                    "key": "csEMV9KkmjgOPF90uc54+hykhg6iI5GTPHlP9PjgLVU="
                }
            ]
            }"#;

        let keys = vec![
            create_key(
                "bc490419-7d60-487f-adc1-4ddcc177c139",
                "wHHVxRwDJs3/bXd38GHP3oe4svTuRpZS0yCC7x4Ly+s=",
            ),
            create_key(
                "0a782fb5-3434-48fe-aa4d-14f41d46cf92",
                "OeGMPxh1+2RpJpNCYixWHFLYRubpOKCw94FcCI7VdJA=",
            ),
            create_key(
                "64a7e9a2-269c-4b2c-832c-5351f3ac5adb",
                "479G1Osfljpmfa5vn24tdzE5zqv5CafkGxYrLCk8384=",
            ),
            create_key(
                "550e8400-e29b-41d4-a716-446655440000",
                "csEMV9KkmjgOPF90uc54+hykhg6iI5GTPHlP9PjgLVU=",
            ),
        ];
        let resp = KeyResponse { keys };
        let from_data: KeyResponse = serde_json::from_str(data).unwrap();
        assert_eq!(
            serde_json::to_string(&resp)
                .expect("Failed to serialize KeyResponse into JSON"),
            serde_json::to_string(&from_data)
                .expect("Failed to reserialize test data into JSON")
        );
    }
}
