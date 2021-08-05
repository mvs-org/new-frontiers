# Migration Strategy of Chain Candidates

The migration from hyperspace consists of upgrades to hyperspace and migration to new-frontiers 
by replaying transactions from hyperspace and injecting them into new-frontiers.
For all Metaverse chain candidates, the code in the repository for Parity frontier was used. 

## There are two ways to accomplish migration of state from one chain to a new one

    1. State snapshot 
    The state of an original chain is snapshotted at a particular block. 
    a new chain is instantiated with the exact same state from genesis.
    
    2. Migration of current hyperspace Mainnet -> new-frontiers by transaction replay. 
    The transaction history of the original chain is collected into a list 
    and executed again on a new chain. The new chain started from the same genesis state.
    
### transaction replay  
In this method we are iterating through the extrinsic from one blockchain to the other then manually seal and progress through the whole history. 
   1. Update the actual Hyperspace Mainnet and make the storage compatible with new-frontier.
   2. The swappable consensus: disable Aura and Grandpa finalization on new-frontier and temporary replace them with manual-seal consensus.  
   3. Perform the actual migration block by block, extrinsic by extrinsic and then manually seal the block.
   4. Restore consensus mechanism on new-frontiers(pow?) 

##### Scrape hyperspace
Using this https://github.com/mvs-org/metaverse-vm-scraper we will be able to simultneously read from hyperspace and inject transactions from each block. This application will fetch all transactions in a block and send them to new-frontiers.
###### Transaction order
Because of the Genefinance algorithm, the order of transactions matter. The sequential iteration of the extrinsics  will ensure this.
###### Run to determine prefunds
Certain accounts will need to be prefunded in order for this mechanism to work.
By replaying the hyperpace chain without commiting to new-frontiers these accounts can be identified and
bootstrapped with the new-frontires chainspec.
###### Manage previous attack
Funds that were stolen on hyperspace musrt also be nulled on new-frontiers.   

