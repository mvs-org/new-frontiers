# Substrate Migration Strategy Hyperspace to New Frontiers

The migration from hyperspace consists of upgrades to hyperspace and migration to new-frontiers 
by replaying transactions from hyperspace and injecting them into new-frontiers.
For all Metaverse chain candidates, the code in the repository for Parity frontier was used. 

## Steps

    1. Update hyperspace
      a) Remove superfluous pallets 
      b) Make sure target chain is the same version 
         as origin
      b) Make sure that hyperspace storage is compatible
      c) Update the actual Hyperspace Mainnet
       
    2. Make a *case* for swappable consensus:
       a) Disable Aura and Grandpa finalization on new-frontier
       b) Replace with manual-seal consensus
       
    3. Transaction replay.
       The transaction history of the original chain is collected into a list 
       and executed again on a new chain.
       Perform the actual migration block by block, extrinsic by extrinsic and then 
       manually seal the block.
       Iterate through the extrinsics from original blockchain,
       transfer to the other, and manually seal the blocks with a RPC call.   
      
     4. The new is chain started from the same genesis state.
     
     5. Switch consensus to new mechanism on new-frontiers (pow)
     
   



##### Upgrade of current hyperspace mainnet
    1. Cleanup the code of Hyperspace, remove all unnecessary pallets and functions
        - Removed unnecessary pallets&code      commit # f55b3555acbda550dff8d2d593245878be20b160
        - Files&Folders clean up - spec_version 1231 commit # 16261f459143fcbd6ff15c3268152571212495e8
        Make sure all platforms are fully tested (Genefinance too!) before proceeding to next update
    2. Upgrade the code of Hyperspace to latest substrate and pallet versions to match new-frontiers 
    

##### Scrape hyperspace and transact the blovcks on the new chain
Using this https://github.com/mvs-org/metaverse-vm-scraper we will be able to simultneously 
read from hyperspace and inject transactions from each block. 
This application will fetch all transactions 
in a block and send them to new-frontiers.

###### Transaction order
Because of the Genefinance algorithm, the order of transactions matter. 
The sequential iteration over the transactions of each block will ensure this.

###### Pre-run to determine prefunds
Certain accounts will need to be prefunded in order for this mechanism to work.
By replaying the hyperpace chain without commiting to new-frontiers these accounts can be identified and
bootstrapped with the new-frontires chainspec.

###### Manage previous attack
Funds that were taken on hyperspace must also be nulled on new-frontiers.   

###### Match hyperspace genesis state on new-frontiers.
    1. Timestamp
    2. block number

###### Manual seal on new-frontiers
Disable Aura and Grandpa finalization on new-frontier and temporary replace them with manual-seal consensus.


#####  Replay hyperspace transactions on new frontiers
Reapply script: https://github.com/mvs-org/metaverse-vm-scraper but this time 

##### Restore consensus mechanism
Restore consensus mechanism on new-frontiers(pow?) 

