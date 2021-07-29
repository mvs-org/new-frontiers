# Migration Strategy of Chain Candidates

The roll-out of Metaverse consistes of N successive deployments of the network, each a possible chain candidate. 
When Metaverse hyperspace was launched it was still uncertain how many migrations would take place and 
how the network would ultimately evolve. The following article describes from a technical standpoint 
the strategy that was taken for the migration from one chain candidate to the next, 
a process known as hard spooning the network. 

For all Metaverse chain candidates, the code in the repository for Parity frontier was used. 

## Tools

### The following tools were involved in the creation and destruction of the networks:

    
    3. Injection and scraping tool. scrapes transaction data of prior chain candidate for 
    the new-frontier migration and inject extrinsics in subseuqnt migration.


## There are two ways to accomplish migration of state from one chain to a new one (the hard spoon):

    State snapshot. The state of an original chain is snapshotted at a particular block; a new chain is instantiated with the exact same state from genesis.
    Transaction replay. The transaction history of the original chain is collected into a list and executed again on a new chain. The new chain started from the same genesis state.

The process that was followed for migration of CC1 -> CC2 and for the migration of CC2 -> CC3 was transaction replay. In order to accomplish this it was necessary to scrape the history of transactions from the previous chains, clean any problematic transactions, then use an injector to play them again.

Below outlines the general flow that the transaction replay follows from a high level (click to enlarge):
Which transactions should not be replayed?

In the above diagram as well as previously in this write up we have discussed problematic transactions as something that was necessary to remove from the history during the spoon. However, we have yet to clarify what constitutes as a problematic transaction.

We define a problematic transaction as any transaction that if replayed could alter the end state of the chain in an undesirable way. For example, runtime upgrades that happend on the old chain and altered the logic in a linear way wouldn’t make sense to replay on the new chain since it’s beginning with a more recent version of the runtime code. There are also situations where manual intervention is required to clean transactions. Notably in the CC2 -> CC3 migration, the slashing refunds were removed from history since no one was slashed on the new chain.

As a rule sudo::set_key and system::set_code were considered problematic transactions and always removed from the transaction history.
What is the speed at which transactions are injected?

Transactions are injected at a speed of 1 per second, and claims transactions are injected one per 3 seconds. This is just a rule of thumb since during testing it was discovered that injecting any more frequently than this caused transaction dependency issues and the end state differed from the expected.

What can be done better: In order to be the safest, transactions should be injected once per block in order not to cause any dependency graph issues. This is most pertinent for the claims transactions which do not have a nonce associated with them (being signed differently) and therefore can not be ordered by linearly increasing the nonce.
Metaverse
Deployment of Metaverse CC2

    Announcement blog post

The migration of CC1 -> CC2 was rather straight forward as there was still only about 400 extrinsics on Metaverse at that time. A small scrape was written which was able to extract all of the extrinsics from the CC1 chain working backwards from the most recent block. After using the polkadot-deployer to provision machines and start validator clients, these scraped extrinsics were injected using a sudo_as method that allowed the origin to be the same.

We first tested out the injections on a test network and discovered that the forks produced by BABE led to some of the transactions becoming invalid. At the time, our tooling was still primitive so we tried to inject the whole history again. After noticing that this impacted the end state of the chain and diverged it from the CC1 chain, we took down the test network. We set up a second network and used a modified injection script that did not allow transactions to be signed using forked blocks as the reference. The second injection had no major issues and a release for CC2 was made.
Take-aways

    Improved the script to not try to sign transactions on forked blocks. Instead we signed every transaction as an immortal and used the genesis block as reference.

Deployment of Metaverse CC3

    Announcement blog post

The deployment from CC2 -> CC3 was considerable more complex than the initial migration. This was for a few reasons:

    There were 10x more transactions to scrape and inject. Using an injection speed of 1 per second this meant that it was over an hour and a half to make the full injection.
    There were a number of extrinsics that needed to be culled from the injection set. Namely anything that changed the runtime code, or moved the chain to PoS prematurely needed to be treated carefully.
    The original scraping script had broken due to a metadata change. A data source from Polkascan was relied upon. This was less than ideal for the procedure and has convinced us to make continuous scraping of our own during the soft launch period of Polkadot.

Only one inconsistency was noticed after the full injection took place. Namely, validators began to claim they lost their nomination. It was not a simple process to check the nominations since (1) the original chain and its state had been bricked beyond repair and prevented us from making state queries to it (2) the UI did not update nomination until the validators were included in the elected set. Due to differences in roll-out strategy it’s possible validators who were included in the initial sets of validator would not be included in the same order. When a method was found to check nomination we were only able to confirm two accounts had failed to make a nomination transaction. We are investigating further in order to revise the process for Polkadot.
Changes to the procedure

    Unbonding was changed to 0 to prevent the unintentional errors that were caused last time.
    The vesting offset was adjusted to account for the time that elapsed on CC2.
    The accounting errors for amended accounts was fixed in genesis. The system::kill_storage and system::set_storage calls were subsequently removed from the transaction history.

Polkadot

For Polkadot we hope to improve the tooling with some goals in mind:

    Increase the transparency and therefore make the process easily auditable by the community.

    Increase consistency in order for injection to be made more reliable with no loss of data.
    Run a scraper continuously and back up data into a MongoDB database (or PostgreSQL). This is so we do not lose any data due to API breaking.

Published on HackMD
324
Like Bookmark Subscribe
