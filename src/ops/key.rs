// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use crate::models::key::NewKey;
use crate::{converter, db};
use crate::{error::Error, models::key::Key};
use actix_web::http::StatusCode;
use sqlx::PgPool;
use log::error;
use rand::prelude::*;
use uuid::Uuid;

pub fn validate_key_size(key_size_bits: i32) -> Result<(), Error> {
    if key_size_bits <= 0 {
        return Err(Error::new(
            StatusCode::BAD_REQUEST,
            "'size' must be greater than zero",
        ));
    }

    if key_size_bits % 8 != 0 {
        return Err(Error::new(
            StatusCode::BAD_REQUEST,
            "'size' must be divisible by 8",
        ));
    }

    Ok(())
}

pub fn validate_num_keys(num_keys: i32) -> Result<(), Error> {
    if num_keys <= 0 {
        return Err(Error::new(
            StatusCode::BAD_REQUEST,
            "'number' must be greater than zero",
        ));
    }

    Ok(())
}

pub fn generate_random_keys(
    key_size_bits: i32,
    num_keys: i32,
) -> Result<Vec<Key>, Error> {
    validate_key_size(key_size_bits)?;
    validate_num_keys(num_keys)?;

    let mut keys: Vec<Key> = Vec::with_capacity(match num_keys.try_into() {
        Ok(size) => size,
        Err(e) => {
            error!("Failed to convert num_keys to size: {:?}", e);
            return Err(Error::internal_server_error());
        }
    });

    for _ in 0..num_keys {
        keys.push(Key {
            id: Uuid::new_v4(),
            content: generate_random_key(key_size_bits)?,
            size: key_size_bits,
        });
    }

    Ok(keys)
}

fn generate_random_key(key_size_bits: i32) -> Result<String, Error> {
    let key_data = generate_random_key_bytes(key_size_bits)?;
    Ok(converter::to_base64(&key_data))
}

fn generate_random_key_bytes(key_size_bits: i32) -> Result<Vec<u8>, Error> {
    if key_size_bits % 8 != 0 || key_size_bits == 0 {
        return Err(Error::new(
            StatusCode::BAD_REQUEST,
            "Key size should be greater than 0 and divisible by 8.",
        ));
    }

    let key_size_bytes: usize = match (key_size_bits / 8).try_into() {
        Ok(size) => size,
        Err(e) => {
            error!("Failed to convert size from 'i32' to 'usize: {:?}", e);
            return Err(Error::internal_server_error());
        }
    };

    let mut key_material = vec![0; key_size_bytes];
    thread_rng().fill_bytes(&mut key_material);
    Ok(key_material)
}

pub async fn save_keys(
    keys: &[Key],
    master_sae_id: &str,
    slave_sae_ids: &[String],
) -> Result<(), Error> {
    let num_rows_to_insert = keys.len() * slave_sae_ids.len();

    let mut keys_to_insert: Vec<NewKey> =
        Vec::with_capacity(num_rows_to_insert);

    for key in keys {
        for slave_sae_id in slave_sae_ids {
            keys_to_insert.push(NewKey {
                id: key.id,
                master_sae_id: master_sae_id.to_string(),
                slave_sae_id: slave_sae_id.clone(),
                size: key.size,
                content: key.content.clone(),
            });
        }
    }

    let pool = &db::establish_connection().await?;
    let mut num_inserted_rows: u64 = 0;
    for key in keys_to_insert{
        let result = match sqlx::query!(
            r#"INSERT INTO keys (id, master_sae_id, slave_sae_id, size, content)VALUES ($1, $2, $3, $4, $5);"#,
            key.id,
            key.master_sae_id,
            key.slave_sae_id,
            key.size,
            key.content,
        ).execute(pool).await{
            Ok(res) => res,
            Err(e) => {
                error!("Failed to save records to db: {:?}", e);
                return Err(Error::internal_server_error());
            }
        };
        num_inserted_rows += result.rows_affected();
    }
    assert_eq!(num_rows_to_insert as u64, num_inserted_rows);
    Ok(())
}

pub async fn get_multiple_keys(
    key_ids: &[uuid::Uuid],
    master_sae_id: &str,
    slave_sae_id: &str,
) -> Result<Vec<Key>, Error> {
    let mut result: Vec<Key> = Vec::new();

    let pool = &db::establish_connection().await?;

    for key_id in key_ids {
        result.push(retrieve_key_from_db(
            key_id,
            master_sae_id,
            slave_sae_id,
            pool,
        ).await?);
    }

    Ok(result)
}

async fn retrieve_key_from_db(
    key_id: &uuid::Uuid,
    master_sae_id: &str,
    slave_sae_id: &str,
    pool: &PgPool,
) -> Result<Key, Error> {
    let num_keys_with_master_sae_id = match sqlx::query!(
        r#"SELECT count(*) as "count!"
        FROM keys
        WHERE 
            id = $1 AND
            master_sae_id = $2 AND
            active = TRUE
        ;"#,
        key_id,
        master_sae_id,
    )
    .fetch_one(pool)
    .await{
        Ok(res) => res,
        Err(e) => {
            error!(
                "Failed to count the number of keys with a specific master_sae_id. Error: {:?}",
                e
            );
            return Err(Error::internal_server_error());
        }
    }
    .count;

    let retrieval_result = match sqlx::query!(
        r#"SELECT id, content, size
        FROM keys
        WHERE 
            id = $1 AND
            master_sae_id = $2 AND
            slave_sae_id = $3 AND
            active = TRUE
        ;"#,
        key_id,
        master_sae_id,
        slave_sae_id,
    )
    .fetch_optional(pool)
    .await{
        Ok(res) => res,
        Err(e) => {
            error!("Failed to retrieve key. Error: {:?}", e);
            return  Err(Error::internal_server_error());
        }
    };

    match retrieval_result {
        Some(retrieved_key) => Ok(
            Key{
                id: retrieved_key.id, 
                content: retrieved_key.content,
                size: retrieved_key.size,
            }
        ),
        None => {
            if num_keys_with_master_sae_id > 0 {
                Err(Error::unauthorized())
            } else {
                Err(Error::new(
                    StatusCode::BAD_REQUEST,
                    format!("Key {} not found", key_id).as_str(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(false, 0; "Zero")]
    #[test_case(false, -8; "Negative value, divisible by 8")]
    #[test_case(false, -10; "Negative value, non-divisible by 8")]
    #[test_case(false, 17; "Positive value, non-divisible by 8")]
    #[test_case(true, 16; "Positive value, divisible by 8")]
    fn test_key_size_validation(is_ok: bool, key_size_bits: i32) {
        assert_eq!(generate_random_keys(key_size_bits, 1).is_ok(), is_ok);
    }

    #[test_case(false, 0; "Zero")]
    #[test_case(false, -10; "Negative value")]
    #[test_case(true, 16; "Positive value")]
    fn test_num_keys_validation(is_ok: bool, num_keys: i32) {
        assert_eq!(generate_random_keys(8, num_keys).is_ok(), is_ok);
    }

    #[test]
    fn test_random_key_generation() {
        let key_size_bits: i32 = 16;
        let num_keys: i32 = 2;

        let result = generate_random_keys(key_size_bits, num_keys);
        assert!(result.is_ok());
        let key_container = result.unwrap();

        assert_eq!(key_container.len(), usize::try_from(num_keys).unwrap());
        for key in key_container {
            assert_eq!(key.size, key_size_bits);
            assert_eq!(key.content.len(), 4);
        }
    }
}
