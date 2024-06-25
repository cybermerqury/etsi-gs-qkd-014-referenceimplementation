-- SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
-- SPDX-License-Identifier: AGPL-3.0-only

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE keys (
    id               uuid        NOT NULL,
    master_sae_id    TEXT        NOT NULL CHECK(ltrim(rtrim(master_sae_id)) != ''),
    slave_sae_id     TEXT        NOT NULL CHECK(ltrim(rtrim(slave_sae_id)) != ''),
    size             INT         NOT NULL CHECK(size > 0),
    content          TEXT        NOT NULL CHECK(ltrim(rtrim(content)) != ''),
    active           BOOLEAN     NOT NULL DEFAULT TRUE,
    last_modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT non_identical_sae_ids CHECK(master_sae_id != slave_sae_id),
    PRIMARY KEY (id, master_sae_id, slave_sae_id)
);

CREATE INDEX ON keys (id)
CREATE INDEX ON keys (master_sae_id);
CREATE INDEX ON keys (slave_sae_id);
CREATE INDEX ON keys (active);

CREATE FUNCTION update_keys_last_modified_at() RETURNS trigger AS
$last_modofied_at$
    BEGIN
        NEW.last_modified_at = NOW();
        RETURN NEW;
    END;
$last_modified_at$ LANGUAGE plpgsql;

CREATE TRIGGER keys_last_modified_at BEFORE INSERT OR UPDATE ON keys
    FOR EACH ROW EXECUTE FUNCTION update_keys_last_modified_at();
