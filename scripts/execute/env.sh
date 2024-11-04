#!/bin/bash
CURRENT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
PROJECT_ROOT_PATH="$( cd $SCRIPT_PATH/../.. >/dev/null 2>&1 ; pwd -P )"
SEEDER_BIN_PATH="$PROJECT_ROOT_PATH/scripts/seeder"

DATA_PATH=$PROJECT_ROOT_PATH/data
CONFIG_FILE_PATH=$DATA_PATH/Config.toml

if [[ ! -f "$SEEDER_BIN_PATH" ]]; then
    echo "Error: Secure RPC binary not found at $SEEDER_BIN_PATH"
    echo "Please run this command 'cp $PROJECT_ROOT_PATH/target/release/seeder $PROJECT_ROOT_PATH/scripts'"
    exit 1
fi

SEEDER_EXTERNAL_RPC_URL="http://127.0.0.1:6000"
SEEDER_INTERNAL_RPC_URL="http://127.0.0.1:6001"