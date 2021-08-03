# Migration Strategy of Chain Candidates

The migration from hyperspace consists of 3 successive deployments of the network.
The following article describes from a technical standpoint 
the strategy that was taken for the migration from one chain candidate to the next, 
a process known as "hard spooning" the network. 
For all Metaverse chain candidates, the code in the repository for Parity frontier was used. 


## There are two ways to accomplish migration of state from one chain to a new one:

    1. State snapshot. The state of an original chain is snapshotted at a particular block; a new chain is instantiated         with the exact same state from genesis.
    2. Transaction replay. The transaction history of the original chain is collected into a list and executed again on a        new chain. The new chain started from the same genesis state.

## Migration of hyperspace -> hyperspace(substrate v3) and for the migration of  hyperspace(substrate v3)-> new-frontiers by transaction replay. 
   
    1. Clean any problematic transactions
      a) We define a problematic transaction as any transaction that if replayed could alter the        end state of the chain in an undesirable way. For example, runtime upgrades that happend on         the old chain and altered the logic in a linear way wouldn’t make sense to replay on the new      chain since it’s beginning with a more recent version of the runtime code.
      b) Manual intervention is required to clean transactions. 
      Notably in the hyperspace -> new-frontiers migration 
    2. Use an injector to replay
      https://github.com/w3f/injection-tool

##### Transactions are injected at a speed of 1 per second 
##### Extrinsics are to be injected using a sudo_as method that allows the origin to be the same.
##### new-frontiers manual-seal will include the block as it is given pro re nata progressus

