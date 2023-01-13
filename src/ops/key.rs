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

pub fn validate_key_size(key_size: i32) -> Result<(), Error> {
    if key_size <= 0 {
        return Err(Error::new(
            StatusCode::BAD_REQUEST,
            "'size' must be greater than zero",
        ));
    }

    if key_size % 8 != 0 {
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
    key_size: i32,
    num_keys: i32,
) -> Result<Vec<Key>, Error> {
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
            content: generate_random_key(key_size)?,
            size: key_size,
        });
    }

    Ok(keys)
}

pub fn generate_random_key(key_size: i32) -> Result<String, Error> {
    let key_data = generate_random_key_bytes(key_size)?;
    Ok(converter::to_base64(&key_data))
}

fn generate_random_key_bytes(key_size: i32) -> Result<Vec<u8>, Error> {
    if key_size % 8 != 0 || key_size == 0 {
        return Err(Error::new(
            StatusCode::BAD_REQUEST,
            "Key size should be greater than 0 and divisible by 8.",
        ));
    }

    let key_size_in_bytes: usize = match (key_size / 8).try_into() {
        Ok(size) => size,
        Err(e) => {
            error!("Failed to convert size from 'i32' to 'usize: {:?}", e);
            return Err(Error::internal_server_error());
        }
    };

    let mut key_material = vec![0; key_size_in_bytes];
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

// TODO: Add tests for validate_inputs and generate_random_keys function.
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test_case::test_case;

//     #[test]
//     fn test_generated_key_bytes_size() {
//         let key_size = 80;

//         let key = generate_random_key_bytes(key_size).unwrap();
//         assert_eq!(key.len(), (key_size / 8) as usize);
//     }

//     #[test_case( 100; "Only multiples of 8 are allowed")]
//     #[test_case( 0  ; "Size of 0 is invalid")]
//     fn test_size_validation(key_size: i32) {
//         let result = generate_random_key(key_size);
//         assert!(result.is_err());
//         assert_eq!(result.err().unwrap().kind(), ErrorKind::InvalidInput);
//     }

//     #[test]
//     fn test_base_64_generation() {
//         let key: Vec<u8> = vec![65; 4];
//         assert_eq!(convert_to_base64(&key), "QUFBQQ==");
//     }
// }
