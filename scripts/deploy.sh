#!/bin/bash
set -e

source ./variables.sh

cd .. &&
bash build.sh &&
cd scripts

if [ "$1" == "deploy" ]; then
  near deploy $POAP_CONTRACT_ACCOUNT_ID ../res/$POAP_WASM_NAME 
elif [ "$1" == "redeploy" ]; then
  near deploy $POAP_CONTRACT_ACCOUNT_ID ../res/$POAP_WASM_NAME
elif [ "$1" == "clean" ]; then
  bash clear-state.sh && near deploy $POAP_CONTRACT_ACCOUNT_ID ../res/$POAP_WASM_NAME
fi
