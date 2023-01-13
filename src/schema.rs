// @generated automatically by Diesel CLI.

diesel::table! {
    keys (id, master_sae_id, slave_sae_id) {
        id -> Uuid,
        master_sae_id -> Text,
        slave_sae_id -> Text,
        size -> Int4,
        content -> Text,
        active -> Bool,
        last_modified_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}
