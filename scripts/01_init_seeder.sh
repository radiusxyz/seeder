#!/bin/bash
SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
source $SCRIPT_PATH/env.sh

DATA_PATH=$CURRENT_PATH/seeder

rm -rf $DATA_PATH

$SEEDER_BIN_PATH init --path $DATA_PATH

CONFIG_FILE_PATH=$DATA_PATH/config.toml

sed -i.temp "s/seeder_external_rpc_url = \"http:\/\/127.0.0.1:6000\"/seeder_external_rpc_url = \"http:\/\/$HOST:6000\"/g" $CONFIG_FILE_PATH
sed -i.temp "s/seeder_internal_rpc_url = \"http:\/\/127.0.0.1:6001\"/seeder_internal_rpc_url = \"http:\/\/$HOST:6001\"/g" $CONFIG_FILE_PATH

rm $CONFIG_FILE_PATH.temp