#!/bin/bash
CURRENT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
PROJECT_ROOT_PATH="$( cd $CURRENT_PATH/../.. >/dev/null 2>&1 ; pwd -P )"

BIN_FILE_NAME="seeder"
BIN_PATH="$PROJECT_ROOT_PATH/scripts/$BIN_FILE_NAME"

DATA_PATH=$PROJECT_ROOT_PATH/data
CONFIG_FILE_PATH=$DATA_PATH/Config.toml
PRIVATE_KEY_PATH=$DATA_PATH/signing_key

# Copy the new version's binary to the scripts directory
if [[ -f "$PROJECT_ROOT_PATH/target/release/$BIN_FILE_NAME" ]]; then
  cp $PROJECT_ROOT_PATH/target/release/$BIN_FILE_NAME $PROJECT_ROOT_PATH/scripts
fi

# Check if the binary exists
if [[ ! -f "$BIN_PATH" ]]; then
    echo "Error: Secure RPC binary not found at $BIN_PATH"
    echo "Please run this command 'cp $PROJECT_ROOT_PATH/target/release/$BIN_FILE_NAME $PROJECT_ROOT_PATH/scripts' after building the project"
    exit 1
fi

# Seeder private key
SEEDER_PRIVATE_KEY="0x2141478fe814f58de31b5a6fb2a7682b7dae755cc19bab6acdbfa1fcfe6e64e1" # Please change this.

SEEDER_EXTERNAL_RPC_URL="http://127.0.0.1:6000" # External IP - Please change this IP.
SEEDER_INTERNAL_RPC_URL="http://127.0.0.1:6001" # Internal IP - Please change this IP.