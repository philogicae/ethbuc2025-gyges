. ./.env

#cargo stylus check -e $RPC_URL
cargo stylus deploy --private-key $PRIV_KEY -e $RPC_URL --contract-address $STYLUS_CONTRACT_ADDRESS --no-verify #--estimate-gas
cargo stylus export-abi --json --output ./scripts/abi.json && ./scripts/clean-abi.py

#--------------------------

cast send --private-key $PRIV_KEY -r=$RPC_URL --json $STYLUS_CONTRACT_ADDRESS "registerUsername(string)" "salut" 2>&1 | tee ./scripts/output.txt && ./scripts/decode.py
#cast call --private-key $PRIV_KEY -r=$RPC_URL --json $STYLUS_CONTRACT_ADDRESS "getAddressByUsername(string)(address)" "salut" 2>&1 | tee ./scripts/output.txt && ./scripts/decode.py
#cast call --private-key $PRIV_KEY -r=$RPC_URL --json $STYLUS_CONTRACT_ADDRESS "getPlayerByAddress(address)((string, uint256))" "0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E" 2>&1 | tee ./scripts/output.txt | ./scripts/decode.py
#cast call --private-key $PRIV_KEY -r=$RPC_URL --json $STYLUS_CONTRACT_ADDRESS "getPlayerByUsername(string)((address, string, uint256))" "salut" 2>&1 | tee ./scripts/output.txt | ./scripts/decode.py
cast send --private-key $PRIV_KEY -r=$RPC_URL --json $STYLUS_CONTRACT_ADDRESS "newGame(string)" "salut" 2>&1 | tee ./scripts/output.txt | ./scripts/decode.py
cast call --private-key $PRIV_KEY -r=$RPC_URL --json $STYLUS_CONTRACT_ADDRESS "gameById(uint256)((address, address, bytes32))" "0" 2>&1 | tee ./scripts/output.txt | ./scripts/display.py

#--------------------------

#stylus-interpreter -s 0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E -b $(cast bn --rpc-url $RPC_URL) --addr $STYLUS_CONTRACT_ADDRESS -u $RPC_URL gyges.wasm $(cast calldata "newGame(string)(uint256)" "salut")