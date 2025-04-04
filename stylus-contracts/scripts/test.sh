. ./.env
#cargo stylus check -e $RPC_URL
cargo stylus deploy -e $RPC_URL --private-key $PRIV_KEY_PATH --no-verify #--estimate-gas
cargo stylus export-abi --json --output ./scripts/abi.json && ./scripts/clean-abi.py

cast send --private-key $PRIV_KEY_PATH -r=$RPC_URL $STYLUS_CONTRACT_ADDRESS "registerUsername(string)" "salut" 2>&1 | tee ./scripts/output.txt && ./scripts/decode-abi.py
cast call --private-key $PRIV_KEY_PATH -r=$RPC_URL $STYLUS_CONTRACT_ADDRESS "getAddressByUsername(string)(address)" "salut" 2>&1 | tee ./scripts/output.txt && ./scripts/decode-abi.py
cast call --private-key $PRIV_KEY_PATH -r=$RPC_URL $STYLUS_CONTRACT_ADDRESS "getPlayerByAddress(address)((string, uint256))" "0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E" 2>&1 | tee ./scripts/output.txt && ./scripts/decode-abi.py
cast call --private-key $PRIV_KEY_PATH -r=$RPC_URL $STYLUS_CONTRACT_ADDRESS "getPlayerByUsername(string)((address, string, uint256))" "salut" 2>&1 | tee ./scripts/output.txt && ./scripts/decode-abi.py