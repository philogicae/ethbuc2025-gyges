. ./.env
#cargo stylus check -e $RPC_URL
cargo stylus deploy -e $RPC_URL --private-key $PRIV_KEY_PATH --no-verify #--estimate-gas
#cast send --private-key $PRIV_KEY_PATH -r=$RPC_URL $STYLUS_CONTRACT_ADDRESS "increment()"
#cast send --private-key $PRIV_KEY_PATH -r=$RPC_URL $STYLUS_CONTRACT_ADDRESS "setNumber(uint256)" "10"
cast call --private-key $PRIV_KEY_PATH -r=$RPC_URL $STYLUS_CONTRACT_ADDRESS "registerUsername(string)(string)" "salut"
#cargo stylus export-abi