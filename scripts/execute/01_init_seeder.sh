#!/bin/bash
SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
source $SCRIPT_PATH/env.sh

rm -rf $DATA_PATH

$BIN_PATH init --path $DATA_PATH

sed -i.temp "s|seeder_external_rpc_url = \"http://127.0.0.1:6000\"|seeder_external_rpc_url = \"$SEEDER_EXTERNAL_RPC_URL\"|g" $CONFIG_FILE_PATH
sed -i.temp "s|seeder_internal_rpc_url = \"http://127.0.0.1:6001\"|seeder_internal_rpc_url = \"$SEEDER_INTERNAL_RPC_URL\"|g" $CONFIG_FILE_PATH

rm $CONFIG_FILE_PATH.temp