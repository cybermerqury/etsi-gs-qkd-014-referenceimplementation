#!/usr/bin/env bash
# SPDX-FileCopyrightText: © 2023 Merqury Cybersecurity Ltd <info@merqury.eu>
# SPDX-License-Identifier: AGPL-3.0-only
ADDR=${ETSI_014_REF_IMPL_IP_ADDR}:${ETSI_014_REF_IMPL_PORT_NUM}/api/v1/keys
if [ "$1" = "GET" ]; then
    # Description:
    #
    # Calls the 'enc_keys' route using the GET method, specifying the number of keys
    # to generate and their size.
    #
    # /api/v1/keys/{slave_sae_id}/enc_keys?number={num_keys}&size={size_of_key}
    #
    # In this example, 'sae_001' is the master SAE, because we are using its
    # certificates when calling the end-point.
    #
    # Parameter description:
    #
    #   number: [Optional] The number of keys requested.
    #   size:   [Optional] The size of the requested keys, in bits. Must be a
    #                      multiple of 8.

    curl                                  \
        -i                                \
        --tlsv1.3                         \
        --cacert "${CERTS_DIR}"/root.crt  \
        --key "${CERTS_DIR}"/sae_001.key  \
        --cert "${CERTS_DIR}"/sae_001.crt \
        "https://${ADDR}/sae_002/enc_keys?number=1&size=24"
    echo "Strings are equal."
elif [ "$1" = "POST" ]; then
    # Description:
    #
    # Calls the 'enc_keys' route using the POST method, specifying the number of
    # keys to generate, their size and any additional sae IDs this key is linked to.
    #
    # /api/v1/keys/{slave_sae_id}/enc_keys?number={num_keys}&size={size_of_key}
    #
    # In this example, 'sae_001' is the master SAE, because we are using its
    # certificates when calling the end-point.
    #
    # Parameter description:
    #
    #   number:                   [Optional] The number of keys requested.
    #   size:                     [Optional] The size of the requested keys, in
    #                                        bits. Must be a multiple of 8.
    #   additional_slave_SAE_IDs: [Optional] A list of additional sae ids to
    #                                        associate with this key.

    curl                                          \
        -i                                        \
        --tlsv1.3                                 \
        --cacert "${CERTS_DIR}"/root.crt          \
        --key "${CERTS_DIR}"/sae_001.key          \
        --cert "${CERTS_DIR}"/sae_001.crt         \
        --header "Content-Type: application/json" \
        --data-raw '{
            "number": 3,
            "size": 24,
            "additional_slave_SAE_IDs": [
                "sae_additional_123",
                "sae_additional_456"
            ]}'                                   \
        "https://${ADDR}/sae_002/enc_keys"
else
    echo "The method to use must be given as a command line parameter."
    echo "Supported parameters are 'GET' or 'POST'."
fi
