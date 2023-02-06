// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only
use log::error;
use std::env;

static ENV_IP_ADDR: &str = "ETSI_014_REF_IMPL_IP_ADDR";
static ENV_PORT_NUM: &str = "ETSI_014_REF_IMPL_PORT_NUM";
static ENV_DB_URL: &str = "ETSI_014_REF_IMPL_DB_URL";
static ENV_TLS_ROOT_CRT: &str = "ETSI_014_REF_IMPL_TLS_ROOT_CRT";
static ENV_TLS_PRIVATE_KEY: &str = "ETSI_014_REF_IMPL_TLS_PRIVATE_KEY";
static ENV_TLS_CERT: &str = "ETSI_014_REF_IMPL_TLS_CERT";
static ENV_NUM_WORKER_THREADS: &str = "ETSI_014_REF_IMPL_NUM_WORKER_THREADS";

pub struct Config {
    pub ip_addr: String,
    pub port_num: u16,
    pub db_url: String,
    pub root_crt: String,
    pub private_key: String,
    pub public_crt: String,
    pub num_workers: u16,
}

impl Config {
    pub fn new() -> Self {
        Self {
            ip_addr: Self::extract_string_value(ENV_IP_ADDR),
            port_num: Self::extract_u16_value(ENV_PORT_NUM),
            db_url: Self::extract_string_value(ENV_DB_URL),
            root_crt: Self::extract_string_value(ENV_TLS_ROOT_CRT),
            private_key: Self::extract_string_value(ENV_TLS_PRIVATE_KEY),
            public_crt: Self::extract_string_value(ENV_TLS_CERT),
            num_workers: Self::extract_u16_value(ENV_NUM_WORKER_THREADS),
        }
    }

    pub fn init(&self) {
        // NOTE: This empty function is used to load the variables on startup.
    }

    fn extract_u16_value(var_name: &str) -> u16 {
        let extracted_value = Self::extract_string_value(var_name);

        match extracted_value.parse() {
            Ok(val) => val,
            Err(e) => {
                error!(
                    "Error when converting '{}' to a u16: {:?}",
                    var_name, e
                );
                panic!("'{}' incorrect value set", var_name)
            }
        }
    }

    fn extract_string_value(var_name: &str) -> String {
        match env::var(var_name) {
            Ok(val) => val,
            Err(e) => {
                error!("Error when extracting '{}': {:?}", var_name, e);
                panic!("Environment variable '{}' not set", var_name)
            }
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    static IP_ADDR: &str = "127.0.0.1";
    static PORT_NUM: u16 = 10000;
    static DB_URL: &str =
        "postgres://db_user:db_password@localhost:10000/key_store";
    static ROOT_CRT: &str = "/home/user/certs/root.crt";
    static PRIVATE_KEY: &str = "/home/user/certs/kme.key";
    static PUBLIC_CRT: &str = "/home/user/certs/kme.crt";
    static NUM_WORKERS: u16 = 2;

    #[test]
    fn test_loading_valid_config_from_env_vars() {
        temp_env::with_vars(
            vec![
                (ENV_IP_ADDR, Some(IP_ADDR)),
                (ENV_PORT_NUM, Some(&PORT_NUM.to_string())),
                (ENV_DB_URL, Some(DB_URL)),
                (ENV_TLS_ROOT_CRT, Some(ROOT_CRT)),
                (ENV_TLS_PRIVATE_KEY, Some(PRIVATE_KEY)),
                (ENV_TLS_CERT, Some(PUBLIC_CRT)),
                (ENV_NUM_WORKER_THREADS, Some(&NUM_WORKERS.to_string())),
            ],
            || {
                let config = Config::new();
                assert_eq!(config.ip_addr, IP_ADDR);
                assert_eq!(config.port_num, PORT_NUM);
                assert_eq!(config.db_url, DB_URL);
                assert_eq!(config.root_crt, ROOT_CRT);
                assert_eq!(config.private_key, PRIVATE_KEY);
                assert_eq!(config.public_crt, PUBLIC_CRT);
                assert_eq!(config.num_workers, NUM_WORKERS);
            },
        );
    }

    #[test]
    #[should_panic]
    fn test_loading_invalid_config_from_env_vars() {
        temp_env::with_vars(
            vec![
                (ENV_TLS_ROOT_CRT, Some(ROOT_CRT)),
                (ENV_TLS_PRIVATE_KEY, Some(PRIVATE_KEY)),
                (ENV_TLS_CERT, Some(PUBLIC_CRT)),
                (ENV_NUM_WORKER_THREADS, Some(&NUM_WORKERS.to_string())),
            ],
            || {
                let config = Config::new();
                assert_eq!(config.ip_addr, IP_ADDR);
                assert_eq!(config.port_num, PORT_NUM);
                assert_eq!(config.db_url, DB_URL);
                assert_eq!(config.root_crt, ROOT_CRT);
                assert_eq!(config.private_key, PRIVATE_KEY);
                assert_eq!(config.public_crt, PUBLIC_CRT);
                assert_eq!(config.num_workers, NUM_WORKERS);
            },
        );
    }
}
