#!/bin/bash
SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
source $SCRIPT_PATH/env.sh

echo "add_sequencing_info"
curl --location $SEEDER_RPC_URL \
--header 'Content-Type: application/json' \
--data '{
  "jsonrpc": "2.0",
  "method": "add_sequencing_info",
  "params": {
    "platform": "'"$PLATFORM"'",
    "service_provider": "'"$SERVICE_PROVIDER"'",
    "payload": {
      "rpc_url": "'"$LIVENESS_RPC_URL"'",
      "websocket_url": "'"$LIVENESS_WS_URL"'",
      "contract_address": "'"$CONTRACT_ADDRESS"'"
    }
  },
  "id": 1
}'
