# Blockchain



## Inside ethereum node

Each node of ethereum `p2p` network contains 2 main concepts

1. Ethereum Client

   Act as `runtime` and contains 4 elements

   1. Ethereum Virtual Machine (EVM)

   capable of executing smart contract code which is compiled to `EVM bytecode`

   2. Memory Pool

    Where the `nodes` stores transaction that it receives

   3. Client Process

    Which coordinate the processing 

    Handles incoming messages and transactions, dispatches them to the `EVM` when appropriate 
    and store the transaction `to`, and retrieves them from the `memorypool`

    Also handles incoming `blockchain block` that the node receives from peer nodesand append to them to the local database

    4. JSON-RPC API
   
    Exposes the `client` functionality of the client to the other node and external users

2. Blockchain database
   
   Holds transaction data

   Keeps the copy of the `EVM` bytecode of all smart contract deployed on the network and holds their state

A transaction is generated when function is invoked on a smart contract of the choosen ethereum node through `JSON RPC `

1. `Full node` recives the transaction from peer node and place it on the memory pool

2. `Full Node` executes the transaction  on the `EVM` for validation

3. If validaiton is successful the node broadcast the transaction to the peer nodes if not validated node won't probagate and transaction will die out 

4. `Mining Node` place the transaction in the memory pool

## Deploying the smart contract 

in order to make deployment we need to make deployment transaction and mining node stores the `EVM` byte code on the chain

## PrivateKey 2 purpose

1. Allows the decryption of the data that has been encrpted using the public key

2. Allows someone to digitally `sign` a document 
    They can produce the signature only if they know the `private key` people who has public key can verify the `signature`

## Cryptographic hash functions

Can map data of arbitrary size to data of `fixed size` 

`Fixed-size` data is called hash or digest

### Properties

1. Deterministic

2. One way

3. Avalnche effect


The token given by ethereum is called `Ether` 

## Proof of work (PoW)

Objective of `pow` is that miners must find miners a nounce such that hash generated fits a certain constraint 

## Proof of Stake(PoS)

Based on `pool` of validators  that votes on vaildity of of new block 

If the `validator cheats` it's associated `ether` will be deleted from the network
and the owner is `banned` form rejoining the network

## Merkle tree and Merkle root

`Block` containing 2 parts 

1. Header

    Contains `metadata` such as `block number` , `timestamp`, `previous block hash` and `PoW nounce` also `Merkle Root` (it is a merkle tree that miner calculates)

2. Body

    Contains all the transaction that included in the block 

* Block transactions are placed at the bottom of the tree, arranged in pairs 

* Each transaction is hashed, each of these `hashes` becomes a leaf of `Merkle root`

`Merkle root` is a single hash summarizing all of the transactions contained in the block in a way that guarantees their `integrity` 

Advantage of having the `Merkle root` on the block header is that a client `SYN` the blockchain faster way by retrieving the block headers rather than entire transaction history from peer network (called `light synchronization`)

## Ethereum Technology

### Ethereum Blockchain

Transactions are hashed in a more compact and efficient structure called `Merkle-Patricia trie` 

Block header also contains 

1. Merkle-Patricia root of the transactions

2. Merkle-Patricia root of the receipts (transaction outputs)

3. Merkle-Patricia root of the current blockchain state

This `trie` helps on following

`Trie` is an ordered data structure use to store a `dynamic set` keys are actually `string` 

`Merkle-Partrcia` combines `trie` and `merkle tree`, It improves the efficiency of a `Merkle tree`  by 
storing the node keys using `PATRICIA Algorithm` 

a) Whether a certain transaction is included in a certain block
b) What the output of a transaction would be
c) Whether an account exists
d) What the balance of an account is


When node receives `block` 

1. The transactions are arranged in a transaction Merkle-Patricia trie specific to new block

2. Transactions are executed on the `EVM`. This action generates transaction receipts, which are arranged in a `Merkle-particia trie` specific to the new block, It also alter the global state trie of which only one instance exists on each node

If the `roots` of new transaction trie, receipt trie and modified state trie  match those in the header , the block considerd valid

Then the `new` and `altered` trie are stored on the full node in a respective  `key-value` store bassed on `level-db` 

### Transaction Store

Contains a transaction trie per block , each `trie` is immutable 

The key of this store is the `transaction hash` (keccak 25-bit hash)

### Receipts Store 

Same as transaction store

### State Store

Contains a single state trie that represents the latest `global state` 
and it is updated each time a new block is appened to the blockchain 

The `state trie` is account centric so the key of this store is account address (160 bytes)

>When a full node receives a new block, it separates the header and the body. It then creates a local transactions trie and a local receipts trie and updates the existing state trie. The new and updated tries are then committed in the respective stores.


Ethereum allows 3 types of synchronization

1. `Full`

    Client download entire blockchain and validate all the blocks locally

2. `Fast`

    Downloads the entire blockchain but validates only the `64` block prior to the start of the synchronization and the new one 

3. `Light`

    Retrieves the current state trie of the blockchain from peer  full node and stores it locally

## Ethereum Virtual Machine (EVM)

It runs on each node , Turing complete 

It can access blockchain data, both in read and write mode 

EVM executes code only after its `digital signature` has been verified and `constraints` based on the current state of the blockchain statisfied 

EIP (Ethereum Improvement Proposals)
