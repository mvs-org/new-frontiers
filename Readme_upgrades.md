# Substrate Migration Strategy Hyperspace to New Frontiers

The migration from hyperspace consists of replaying transactions from hyperspace and injecting them into new-frontiers. 

## Steps

       
    1. Make a *case* for swappable consensus:
       a) Disable Aura and Grandpa finalization on new-frontier
       b) Replace with manual-seal consensus
       
       
    2. Transaction replay.
       a) The transaction history of the original chain is collected into a list 
       and executed again on a new chain.
       b) Perform the actual migration block by block, extrinsic by extrinsic and then 
       manually seal the block.
       c) Iterate through the extrinsics from original blockchain,
       transfer to the other, and manually seal the blocks with a RPC call.   
       d) Copy the timestamp from hyperspace to new-frontiers 
     
     3. Switch consensus to new mechanism on new-frontiers (pow)
     
   




##### Scrape hyperspace and transact the blovcks on the new chain
Using this https://github.com/mvs-org/metaverse-vm-scraper we will be able to simultneously 
read from hyperspace and inject transactions from each block. 
This application will fetch all transactions 
in a block and send them to new-frontiers, inserting the timestamp in a new parameter.

###### Pre-run to determine prefunds
Certain accounts will need to be prefunded in order for this mechanism to work.
By replaying the hyperpace chain without commiting to new-frontiers these accounts can be identified and bootstrapped with the new-frontiers chainspec.

###### Manage previous attack
Funds that were taken on hyperspace must also be nulled on new-frontiers.   

###### Match hyperspace genesis state on new-frontiers.
Timestamp

###### Manual seal on new-frontiers
Disable Aura and Grandpa finalization on new-frontier and temporary replace them with manual-seal consensus.


#####  Replay hyperspace transactions on new frontiers
script: https://github.com/mvs-org/metaverse-vm-scraper 

##### Restore consensus mechanism
Restore consensus mechanism on new-frontiers 

