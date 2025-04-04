#!/home/philogicae/.pyenv/shims/python
from json import load
from os import path

from eth_abi import decode as eth_abi_decode
from web3 import Web3

current_dir = path.dirname(path.abspath(__file__))
with open(path.join(current_dir, "output.txt"), "r", encoding="utf-8") as f:
    revert_data = f.read().strip()

if revert_data.startswith("Error:"):
    revert_data = revert_data.split("data: ")[1][1:-1]
else:
    exit()

with open(path.join(current_dir, "abi.json"), "r", encoding="utf-8") as f:
    contract_abi = load(f)

w3 = Web3()
error_selector = revert_data[2:10]
for item in contract_abi:
    if item["type"] == "error":
        signature = (
            f"{item['name']}({','.join([inp['type'] for inp in item['inputs']])})"
        )
        calculated_selector = w3.keccak(text=signature).hex()[:8]
        if calculated_selector == error_selector:
            input_types = [inp["type"] for inp in item["inputs"]]
            decoded_args = eth_abi_decode(input_types, bytes.fromhex(revert_data[10:]))
            print(f"-> {item['name']}{decoded_args}")
            exit()

print("-> Unknown error")
