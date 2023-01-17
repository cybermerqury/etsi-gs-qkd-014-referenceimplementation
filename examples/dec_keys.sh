#!/usr/bin/env bash
# SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: AGPL-3.0-only

script_dir="$( cd "$(dirname "$0")" || exit 1; pwd -P )"
certs_dir="${script_dir}/../certs"

if [ -z "$2" ]; then
    echo "No key id supplied. The key id is required as " \
         "the second command line argument."
fi

if [ "$1" = "GET" ]; then
    # Description:
    #
    # Calls the 'dec_keys' route using the GET method, specifying the key ID to
    # retrieve.
    #
    # /api/v1/keys/{master_SAE_ID}/dec_keys
    #
    # In this example, 'sae_002' is the slave SAE, because we are using its
    # certificates when calling the end-point. In this call, we are requesting
    # keys with the 'master_SAE_ID' of 'sae_001' based on the URL called.
    #
    # Parameter description:
    #
    #   key_ID: [Optional] The ID of the requested key.

    curl                                  \
        -i                                \
        --tlsv1.3                         \
        --cacert "${certs_dir}"/root.crt  \
        --key "${certs_dir}"/sae_002.key  \
        --cert "${certs_dir}"/sae_002.crt \
        "https://127.0.0.1:8443/api/v1/keys/sae_001/dec_keys?key_ID=${2}"
elif [ "$1" = "POST" ]; then
    # Description:
    #
    # Calls the 'dec_keys' route using the GET method, specifying the key ID to
    # retrieve.
    #
    # /api/v1/keys/{master_SAE_ID}/dec_keys
    #
    # In this example, 'sae_002' is the slave SAE, because we are using its
    # certificates when calling the end-point. In this call, we are requesting
    # keys with the 'master_SAE_ID' of 'sae_001' based on the URL called.
    #
    # Parameter description:
    #
    #   key_IDs: [Optional] A list of key_IDs to retrieve.

    curl                                          \
        -i                                        \
        --tlsv1.3                                 \
        --cacert "${certs_dir}"/root.crt          \
        --key "${certs_dir}"/sae_002.key          \
        --cert "${certs_dir}"/sae_002.crt         \
        --header "Content-Type: application/json" \
        --data-raw '{
            "key_IDs": [
                {
                    "key_ID": "'"${2}"'"
                }
            ]}'                                   \
        "https://127.0.0.1:8443/api/v1/keys/sae_001/dec_keys"
else
    echo "The method to use must be given as a command line parameter."
    echo "Supported parameters are 'GET' or 'POST'."
fi
