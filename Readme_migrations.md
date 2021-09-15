# Current issues  migration process from Hyperspace to New Frontiers 

## Summary:
Our goal is to migrate only the EVM related content. We’ve concluded that to be able to achieve this goal we have two feasible options: an EVM focused solution actually replaying transactions from hyperspace blockchain and injecting them into new-frontiers  and a plan B solution, more substrate framework focused, that involves a storage migration between those two blockchains. 

More details over the entire process can be found here: https://github.com/mvs-org/new-frontiers/blob/master/Readme_upgrades.md

## Current open issues:
1. During the tx replay done using new-frontier blockchain with manual-seal enabled (https://github.com/mvs-org/new-frontiers/tree/tx_injection) we’ve encountered an issue located at the block 29428, a contract call that fails on new-frontier replay and it was successfully on hyperspace
(https://vm-explorer.mvs.org/mainnet/tx/0x9de1f72faa5985c1fd73b10c032c5861b8942514c13baa36415be6d713acf594) - after this one there are multiple failed txs due to a domino effect. A major breakthrough should be finding the reason why this failure happens.
We’ve also tested extrinsic replay, a more substrate framework focused, but we had the same result as tx replay failure. 
To get more clues we’ve added manual seal to hyperspace blockchain and the replay still failed, also we’ve downgraded new-frontier’s EVM pallet to match hyperspace version and still we had a tx failure so we concluded that the error is not EVM dependent. 
Currently we are trying to debug what is happening inside the smart contract to be able to find the cause.



