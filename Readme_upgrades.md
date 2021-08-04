# Migration Strategy of Chain Candidates

The migration from hyperspace consists of 3 successive deployments of the network.
The following article describes from a technical standpoint 
the strategy that was taken for the migration from one chain candidate to the next, 
a process known as "hard spooning" the network. 
For all Metaverse chain candidates, the code in the repository for Parity frontier was used. 

## There are two ways to accomplish migration of state from one chain to a new one:

    1. State snapshot 
    The state of an original chain is snapshotted at a particular block. 
    a new chain is instantiated with the exact same state from genesis.
    2. Migration of current hyperspace Mainnet -> new-frontiers by transaction replay. 
    The transaction history of the original chain is collected into a list 
    and executed again on a new chain. 
    The new chain started from the same genesis state.
    
## 1. State snapshot 
In this method we are iterating through the extrinsic from one blockchain to the other then manually seal and progress through the whole history. 
   1. Update the actual Hyperspace Mainnet and make the storage compatible with new-frontier.
   2. The swappable consensus: disable Aura and Grandpa finalization on new-frontier and temporary replace them with manual-seal consensus.  
   3. Perform the actual migration block by block, extrinsic by extrinsic and then manually seal the block. 

## Scrape hyperspace
https://github.com/mvs-org/metaverse-vm-scraper
  1. Transactions per block
  2. Order of transactions matter
  3. Run to determine prefunds
  
## 2. Migration of current hyperspace mainnet -> new-frontiers by transaction replay. 
   
    1. Clean any problematic transactions
      a) We define a problematic transaction as any transaction that if replayed could alter the        end state of the chain in an undesirable way. For example, runtime upgrades that happened on the old chain and altered the logic in a linear way wouldn’t make sense to replay on the new chain since it’s beginning with a more recent version of the runtime code.
      b) Manual intervention is required to clean transactions. 
      Notably in the hyperspace -> new-frontiers migration 
    2. The swappable consensus: disable Aura and Grandpa finalization on new-frontier and temporary replace them with manual-seal consensus. This will allow a smooth transaction replay procedure. 
    3. Use an injector to replay in combination with https://github.com/w3f/injection-tool

##### Hyperspace ans new-frontiers have the same block schema
##### Transactions are injected at a speed of 1 per second 
##### Extrinsic are to be injected using a sudo_as method that allows the origin to be the same
##### new-frontiers manual-seal will include the block as it is given pro re nata progressus
