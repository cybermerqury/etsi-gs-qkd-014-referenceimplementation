// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

pub struct Default {
    pub key_size: i32,
    pub num_keys: i32,
}

pub const DEFAULT: Default = Default {
    key_size: 1024,
    num_keys: 1,
};
