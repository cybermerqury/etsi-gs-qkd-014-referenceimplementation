// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

pub struct Default<'a> {
    // Keys
    pub key_size: i32,
    pub num_keys: i32,
    pub max_key_count: i32,
    pub max_key_per_request: i32,
    pub max_key_size: i32,
    pub min_key_size: i32,
    // SAEs
    pub max_additional_saes: i32,
    // KMEs
    pub src_kme_id: &'a str,
    pub dst_kme_id: &'a str,
}

pub const DEFAULT: Default = Default {
    key_size: 1024,
    num_keys: 1,
    max_key_count: 0,
    max_key_per_request: 0,
    max_key_size: 0,
    min_key_size: 0,
    max_additional_saes: 0,
    src_kme_id: "src_kme_id",
    dst_kme_id: "dst_kme_id",
};
