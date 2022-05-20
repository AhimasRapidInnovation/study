# Ethereum

1. [Account](#ethereum-accounts)
2. [Smart Contracts](#smart-contracts)
3. [Transacion](#transactions)

There is single, canonical computer called the ethereum virtual machine whose state everyone on the ethereum network agrees on .

Every participant in `ethereum` netwoks keeps a copy of the state of this computer

## Ether

Native cryptcurrency of `ethereum`  
Provide incentives  etc

## Ethereum Accounts

Account is an entity with an ether `ETH` balance that can send transactions on ethereum  

Accounts can be user controlled or deployed as smart contract 

### Account Types

1. Externally-Owned

    Controlled by any one with `private keys` 

2. Contracts

    A smart contract deployed to the network, controlled by code.

Both accounts types have ability to:

* Receive , hold and send `ETH` and tokens  

* Interact with `deployed smart contracts` 

### Account Fields

> Wei is the smallest denomination of ether
> 
#### 1.nounce

A counter that indicates the `number of transaction` sent from the account  

This ensures transactions are only processed once  

In `contract` account `nounce` represents the number of contracts created by the account

#### 2. Balance  

The number of `wei` owned by this `address` 

#### 3. Code Hash

Hash referes to the `code` of an `account` on the `Ethereum` virtual macthine `EVM`  

In `contracts` accounts have code fragments programmed in that can perform different operations  

#### 4. Storage Root

Also known as `storage hash`  
`256 bit ` hash of the `root of a merkle patricia trie`  that encodes the storage content of the account 

### Externally-Owned accounts and key pairs

Account is made up of `private` and `public` key.  

It helps to prove that `transaction` is signed by the `sender`  

Sign `transaction` using `private` key  

You never hold `cryptocurrency`  

This prevents malicious actors from brodcasting fake transactions because we can alway verify the sender of a transacion

### Account Creation  

A private key is made up of `64 hex` characters and can be encrypted with a password.

Key created using `Eliptic curve digital signature algorithm` 

### Contract Account

`42 character` hexadecimal address

Contract is usually given when a contract is deployed to the `Ethereum` block chain 




## Smart Contracts

Upload programs (reusable code snippet) into `EVM`(Ethereum Virtual Machine) state  
users make requests to execute these code snippets with **varying** parameters 

Can create a `smart contract` and make it public to the netwok using blockchain as its `datalayer`  
Any user 

> It is a program that runs at an address on Ethereum 

It is made up of `data` and `function` that can execute upon receiving a transaction  

`Smart Contracts` are a type of `Ethereum` account  

It has `balance` and they can send `transaction` over the network  
Deployed to the network and run as programmed  
User accounts can interact with a smart contracts by submitting transactions that execute a function 
defined on the smart contract

User account can interact with a smart contract by submitting transactios that execute  a function defined on the smart contracts  

Smart contract cannot be delete by default and interaction with them are `irreversible`

## EVM (Ethereum virtual Machine)

Single entity maintained by thousands of connected computers running an `ethereum` client  

All ethereum accounts and smart contract are live here

> Ethereum is distributed state machine  

Ethereum's state is large data structure which holds `accounts`, `balance` also `machine state`
which can change from block to block according to predefined set of rules and which can execute arbitrary machine code  

## Transactions

Cryptographically signed instruction from accounts

Transcation which change the state of the `EVM` and needs to be broadcast to the whole network 

> Transaction require a fee  and must be mined to become valid.

### Transaction Informations

#### 1)  receipt

The `receiving` address  

* If externally-owned account the transaction will transfer value

* If contract account, the transaction will execute contract code

#### 2) signature

The `identifier` of the sender  
Generated when the `sender's privatekey` signs the transaction

#### 3) value

Amount of `ETH` to transfer from `sender to recipient`  (In WEI)

#### 4) data

Optional filed to include arbitrary data

#### 5) Gas Limit

<small> Gas is reference to the computation required to process the transaction by the miner  </small>

`Maximum` amount of gas units that can be consumed by the transaction

#### 6) maxPriorityFeePerGas

max amount of `gas` to be included as a tip to the `miner` 

#### 7) maxFeePerGas

maximum amount of `gas` willing to be paid for the transaction 

```json
{
  from: "0xEA674fdDe714fd979de3EdF0F56AA9716B898ec8",
  to: "0xac03bb73b6a9e108530aff4df5077c2b3d481e5a",
  gasLimit: "21000",
  maxFeePerGas: "300",
  maxPriorityFeePerGas: "10",
  nonce: "0",
  value: "10000000000"
}

```

Transaction needs to be signed using `sender's` privatekey 

### The data field

The vast majority of transactions access the contracts from a externally-owned account 

`first 4 bytes`- Specifies which function to call using hash function name and argument 

`rest of the calldata ` is the argument 

### On Gas

Simple transfer transactions require `21000 units` of Gas

Gas is required for any smart contract interaction too.

Any gas not used in a transaction is refunded to the user account.


### Transaction Lifecycle 

1. Once transaction is send , cryptography generates a `transaction hash`  

>0x97d99bc7729211111a21b12c933c949d4f31684f1d6954ff477d0477538ff017

2. Transaction is then broadcast to the network and included in a pool with lots of other transactions

3. Miner must pick transaction and include it in a `block` to verify the transaction 

4. `transaction` will receive `confirmations`. the number of confirmations is number of blocks created since block that included transaction

2 Types of transaction
----------------------

1) Result in message calls  

2) Result in contract creation
    1) Creation of new contracts containing compiled `smart contract` byte code  
    2) Whenever another account makes a  message call to that contract it executes its `bytecode`  

## EVM instruction  

`EVM` executes as `stack machine` with a depth of `1024` items  
