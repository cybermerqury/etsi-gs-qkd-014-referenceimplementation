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
