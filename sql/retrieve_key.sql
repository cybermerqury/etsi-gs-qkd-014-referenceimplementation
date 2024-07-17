SELECT id, content, size
FROM keys
WHERE 
    id = $1 AND
    master_sae_id = $2 AND
    slave_sae_id = $3 AND
    active = TRUE
;