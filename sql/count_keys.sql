SELECT count(*) as "count!"
FROM keys
WHERE 
    id = $1 AND
    master_sae_id = $2 AND
    active = TRUE;