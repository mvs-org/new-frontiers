This pallet ensures that any migration will be performed only once, so it's not strictly necessary that we do anything. However, there may be value in leaving some comments about when a migration was run or doing some other bookkeeping. 
We can't query MigrationState because on_runtime_upgrade() would have unconditionally set it to true, so we read a hint from temp storage which was left for us by pre_upgrade()

Storage migration ideas and notes:

1. Read all the data into memory.
https://crates.parity.io/frame_support/storage/migration/fn.storage_key_iter.html

2. Return the weight used. For each migrated block there is a red to get it into memory, a write to clear the old stored value, and a write to re-store it.