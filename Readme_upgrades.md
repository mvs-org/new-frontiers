# Hyperspace to New Frontiers 

The migration from hyperspace consists of replaying transactions from hyperspace and injecting them into new-frontiers. 

## Steps

    1. Consensus:
       a) Disable Aura and Grandpa finalization on new-frontier
       b) Replace with manual-seal consensus
       
       
    2. Transaction replay.
       a) The transaction history of the original chain is collected.
       b) Perform the actual migration block by block, extrinsic by extrinsic and then 
       manually seal the block with a RPC call.   
       d) Copy the timestamp from hyperspace to new-frontiers 
     
     
##### Scrape hyperspace and transact the blocks on the new chain
Using this https://github.com/mvs-org/metaverse-vm-scraper we will be able to simultneously 
read from hyperspace and inject transactions from each block. 
This application will fetch all transactions in a block and send them to new-frontiers, inserting the timestamp in a new parameter.

###### Pre-run to determine prefunds
Certain accounts will need to be prefunded in order for contracts to be 
deployed onto them. By replaying the hyperpace chain without commiting to 
new-frontiers these accounts have been identified. 
They can be bootstrapped with the new-frontiers chainspec.

NEED PREFUND:
```
x6be02d1d3665660d22ff9624b7be0551ee1ac91b = 5CNJv1vQjABY9W3BtsV2tzaLCjZepWXaYYzuDGWUUNVvMjcG
0xb90168c8cbcd351d069fffda7b71cd846924d551 = 5DyD32oKsLXSTQfwUdApxu8m7URYHHytSBygVAE2vRarVVRj

0xe4968590f80fb43ebc48963c29fac0118542ca7d = 5F6zM2M3qczVkgRrELV9SFPThcUPK4G9q43bpmti8ow8vVzy
0xD78ceA77cb890A5e6Eff2B4C31f24e61C27f9Baa = 5GFg3NJDU8BrMq9Mcq2yX5VgVYdfPaauuWRBzr27P5bBcEHu
0xf484a6a2cCe70DFDf58E2029B4eC9c45a16dfd2B = 5FaYiGMrZt8T3Phc6q3MJa4pc1AK9Y1zVykCzwHyX85xAnze

0xC6E9C81FaCfAb0dE63cA3d8D98b1303f7424366f = 5Cs37fXfaioyurYeKggt4sHW2757TpESsAXj1pQkVbyw93Pb

0xc1dd09Cc23a1A5A073a37D6Ea27E0e42908B8964 = 5CLbCaKxJ84vRFGCJb2S3ZY2w9sNQtLaKHo6KKwhVGteSs4y

0xC33a00f854975fAe599A079dCC6d079fAa55E478 = 5GwPyK94KmH9732Yo7UQPgcAUX2ty5rY1eCU59d3ZDKtmnCC 
0x83295c12ACf26e86f0399bf8A74D699f0f214Ec5 = 5FjR8At6fTU33vFPGT74CQRLducaw7nBKZsWeR752xdbJtXm 

0xa09340E6230405aC217362d5Bd11CF0D3D0dE9f8 = 5EcF5UzCutbB1DL3mVC5CRKRpXbDpdaYGDVk9twEPqyyomCm
0x83aAEF90e55704006B0f7fdb32dF5b25ade15744 = 5HRDzZV7Ms7iAn6jbZgZHUnsjqqumFBg6WCuinGFDRDAbS9h

0x03DfD410D9f9Da6411d55D9b11D54cF9a9F0533c = 5GdjKUWqCACSdFgxfYQfMSGxU1KaKPQURXAPeZ1ojrsu1ck7
0x9B913284A748bE3B31a2a4089D085385B7603B64 = 5D53rnSsAJt9VAB8U1NZdtmuLjMhBZ3hMP17PQWknha45Cgd
0xf97cF0810723d2Bc7e6A8070E22e144130fC469D = 5DfYk33wQjLQyYPtjfgc7iCVAbvtw8BMmfUyAgmWHW3aDvX5
0x5702be76109Cc6f227C654D10e3fca1222f68128 = 5C4pUjUfTrUqaadaBRKR5K6sRh7nZoVkGAPWuu6sFTM3aJ9A
```


###### Manage previous attack
Funds that were taken on hyperspace mustbe nulled on new-frontiers.   

###### Match hyperspace genesis state on new-frontiers.
Create new timestamp parameter on New frontiers

###### Manual seal on new-frontiers
Disable Aura and Grandpa finalization on new-frontier and temporary replace them with manual-seal consensus.


#####  Replay hyperspace transactions on new frontiers
script: https://github.com/mvs-org/metaverse-vm-scraper 


