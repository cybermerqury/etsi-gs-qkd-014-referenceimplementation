// SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
// SPDX-License-Identifier: AGPL-3.0-only

use crate::models::key::NewKey;
use crate::{converter, db};
use crate::{error::Error, models::key::Key};
use actix_web::http::StatusCode;
use diesel::prelude::*;
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

pub fn save_keys(
    keys: &[Key],
    master_sae_id: &str,
    slave_sae_ids: &[String],
) -> Result<(), Error> {
    use crate::schema::keys;

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

    match diesel::insert_into(keys::table)
        .values(keys_to_insert)
        .execute(&mut db::establish_connection()?)
    {
        Ok(num_inserted_rows) => {
            assert_eq!(num_rows_to_insert, num_inserted_rows);
            Ok(())
        }
        Err(e) => {
            error!("Failed to save records to db: {:?}", e);
            Err(Error::internal_server_error())
        }
    }
}

pub fn get_multiple_keys(
    key_ids: &[uuid::Uuid],
    master_sae_id: &str,
    slave_sae_id: &str,
) -> Result<Vec<Key>, Error> {
    let mut result: Vec<Key> = Vec::new();

    let db_conn = &mut db::establish_connection()?;

    for key_id in key_ids {
        result.push(retrieve_key_from_db(
            key_id,
            master_sae_id,
            slave_sae_id,
            db_conn,
        )?);
    }

    Ok(result)
}

fn retrieve_key_from_db(
    key_id: &uuid::Uuid,
    master_sae_id: &str,
    slave_sae_id: &str,
    db_conn: &mut PgConnection,
) -> Result<Key, Error> {
    use crate::schema::keys;

    let num_keys_with_master_sae_id: i64 = match keys::table
        .filter(keys::id.eq(key_id))
        .filter(keys::master_sae_id.eq(master_sae_id))
        .filter(keys::active.eq(true))
        .count()
        .get_result(db_conn)
    {
        Ok(res) => res,
        Err(e) => {
            error!(
                "Failed to count the number of keys with a specific master_sae_id. Error: {:?}",
                e
            );
            return Err(Error::internal_server_error());
        }
    };

    let retrieval_result: Option<Key> = match keys::table
        .filter(keys::id.eq(key_id))
        .filter(keys::master_sae_id.eq(master_sae_id))
        .filter(keys::slave_sae_id.eq(slave_sae_id))
        .filter(keys::active.eq(true))
        .select((keys::id, keys::content, keys::size))
        .get_result(db_conn)
        .optional()
    {
        Ok(res) => res,
        Err(e) => {
            error!("Failed to retrieve key. Error: {:?}", e);
            return Err(Error::internal_server_error());
        }
    };

    match retrieval_result {
        Some(retrieved_key) => Ok(retrieved_key),
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
