 #!/bin/bash
#echo "||||||||||||||||||||||||||||||||||||||||||||"

#cd /tmp/metaverse-vm-cli/cmd
#./mvs-vm-cli help

#curl -X POST -d '{"method": "rpc_methods", "params":[""], "jsonrpc":"2.0", "id": 0}' -H "Content-Type: application/json" http://172.30.0.10:9933 | jq | grep "eth_"
# echo "-----------------------------------------------------------------------------------------------------------------------------------------"
# curl -s -X POST -d '{"method": "offchain_localStorageSet", "params":["PERSISTENT", "0x01", "0xDEADBEEF"], "jsonrpc":"2.0", "id": 1}' -H "Content-Type: application/json" http://172.30.0.10:9933
# echo "-----------------------------------------------------------------------------------------------------------------------------------------"
# curl -X POST -d '{"method": "web3_clientVersion", "params":[], "jsonrpc":"2.0", "id": 10}' -H "Content-Type: application/json"  http://172.30.0.10:9933
# echo "-----------------------------------------------------------------------------------------------------------------------------------------"
# curl -s -X POST -d '{"method": "system_peers", "params":[], "jsonrpc":"2.0", "id": 1}' -H "Content-Type: application/json" http://172.30.0.10:9933
# echo "-----------------------------------------------------------------------------------------------------------------------------------------"
# curl -X POST -d '{"method": "eth_estimateGas", "params":[{"from":"0x34249F7f5640A3c534AA4d5DBB1e999D922462E1","to":"0x237A00Fd62F5176E0D68E33495FAA5C4cA360C52","gasPrice":"0x85000000","gas":"0x2000000000","value":"0x01","data":"0x00"}], "jsonrpc":"2.0", "id": 1}' -H "Content-Type: application/json" http://172.30.0.10:9933
# echo "-----------------------------------------------------------------------------------------------------------------------------------------"
# # for ((i=1;i<=10000;i++)); do curl -X POST -d '{"method": "eth_getBlockByNumber", "params":['$i',true], "jsonrpc":"2.0", "id": 1}' -H "Content-Type: application/json" http://172.30.0.10:9933 ; done
# echo "-------------------------------------------------------------------------------------------------------------------------------------
#curl -X POST --data '{"jsonrpc":"2.0","method":"eth_getTransactionReceipt","params":["0xad39b6410475de76c7625b18233795de051a70ef11f565a78474727ca89cc5af"],"id":0}' --header "Content-Type: application/json" --url  http://172.30.0.10:9933 

# echo "-----------------------------------------------------------------------------------------------------------------------------------------"
#result=$(curl -s  http://127.0.0.1:9933 -H 'Content-Type: application/json' -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\":\"eth_getBalance\", \"params\":[\"$line\"]}" | jq '.result' | sed -e 's/^"//' -e 's/"$//') 
#decimal=$(printf '%d\n' $result)

#  curl -X POST -d '{"method": "eth_getLogs", "params":[{"address":"0x51866ac12965cdc30cf0640615e445253ce8616d","fromBlock":"0x2b56","toBlock":"0x2f3d","topics":[["0x4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f","0xdccd412f0b1252819cb1fd330b93224ca42612892bb3f4f789976e6d81936496","0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822","0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef","0x1c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1"]]}], "jsonrpc":"2.0", "id": 13200}' -H "Content-Type: application/json" http://172.30.0.10:9933


# echo "-----------------------------------------------------------------------------------------------------------------------------------------"
# echo "-----------------------------------------------------------------------------------------------------------------------------------------"