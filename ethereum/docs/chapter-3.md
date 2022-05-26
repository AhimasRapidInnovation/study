# Chapter-3

## Connecting to ethereum through wallet

Ethereum networks offer two main graphical user interface

1. Mist - browser for ethereum Dapps

2. Ethereum wallet - Specific version of Mist with a single Dapp bundled in it.

## Wallet

Main purpose is to `store` , `receive`, and `transfer` `Ether`

## Smart Contracts

A contract receives transaction message from a user account or from other contract and executes
its logic on the ethereum virtual machine

may send message to other contracts, reading state from blockchain, or writing state to the blockchain

## Accounts

1. Externally Owned Accounts(EOA)

    User Account
    Publicly identifiable from their public key
    Can start transaction against smart contract from EOA

2. Contract Account

    Contracts are executed under this account
    Account address generated at `deployment` time and it identifies the location of the contract in the blockchain

    Cannot start `transaction` message

## Ether

Ether is a cryptcurrency that ethereum supports

main purpose is to represent monetory value for services and goods trade over the platform

Also used to pay transaction fee calculated in unit called `gas`

>gas measures computational resource that a transaction consumes
> 1 Ether is equal to 1e18 Wei

## Minting Ether

Ether is generated through the `mining` process , successful miner gets rewarded with a certain number of `Ether`

## Transferring Ether

Once ether has been generated, it's allocated to miners external accounts `EoA`

## Exchanging Ether

Ether often transfered in return for goods and services traded through smart contract

## Storing Ether

    Wallet, etc ..

## GAS

The amount of gas used to compete a transaction depend on the amount of computational resources that the `EVM` spends while running the transaction

It is charged in order to prevent DoS attacks

* `EVM` determines the number of units of gas consumed while running the transaction

* The `Sender` of the transaction decides the `price of a unit of gas`, the higher it is the more likely it is the miner will include in the block

1. If successful completes Remaining `gas` returned to the `sender`

2. If gas completes before execution `EVM` throws an `end of gas` exception and transaction is rolled back (miner will collect the gas)

## Calls and transaction

Accounts interact with each other through 2 types of message

1. Calls

    Call is sent through `message`  that does not get stored on the blockchain and it's execution has following characterstics

    a) Can only perform read only operation
    b) Does not consume gas
    c) Processed sync
    d) Immediatly returns returned value

2. Transactions

    It is sent through a message that gets `serialized` and stored on the blockchain during the mining process

    a) Sender
    b) Recipient address
    c) Value - Amount of ether to be transferred (in Wei)
    d) Data - Input parameters
    e) StartGas - Max amount of gas to be used for execution of the message
    d) Digital Signature - Proves the identity of the sender
    f) Gas Price - The price of unit of gas , The transaction initiator is willing to pay

    * It performs the `write` operation

    * Consumes `gas`

    * Processed `async`

    * immediatly returns `transaction_id` but not return value

## Ethereum Virtual Machine (EVM)

It is `stack based` abstruct compute machine

Enables a computer to run an `ethereum` application

It has 2 `memory` area

1. Volatile Memory (or) Simple Memory

    Word addressed `byte array`

    Allocated to a contract at every message call.

    Read access `256-bit` word

    write can be performed on a width of `8` or `256 bit`

2. Storage

    `key-value` store

    Both key and value have a width of `256` bits

    Storage is allocated to each account and is persisted on the `blockchain`
    A contract account only access its own storage

### Opcodes

`CREATE` - Performs the creation of a new contract instance

`CALL` - A contract sends a message to itself or other contract through this operator

`DELEGATECALL` - Allows the calling contract to send a message to an external contract  but execute the related code in the context of the caller

A contact can only access the other `contracts`

## Connecting to Ethereum with GETH

`Parity` rust eth client

### JSON-RPC Request

`Transport` layer is not part of the `json-rpc` protocol

`Json-Rpc request` Contains

1. `jsonrpc` - Protocol version

2. `method` - Name of the remote procedure to be called

3. `params` - Array with proced parameters

4. `id` - Caller identifier must not be `null`

### JSON-RPC Response

1. `jsonrpc` - Protocol version

2. `result` - if present response is successful and if it is not then error

3. `error` - If field present the there is error

4. `id` - Same as specfied in the request

Request 

```cmd
C:\>curl -H "Content-Type: application/json" -X POST --data
{\"jsonrpc\":\"2.0\",\"method\":\"web3_clientVersion\",\"params\":[],\"id\":2
3} http://localhost:8545
```

Response

```json
{"jsonrpc":"2.0",
"id":23,
"result":"Geth/v1.6.5-stable-cf87713d/windows-
amd64/go1.8.3"}
```


### Numbers through the json-rpc

Numbers send and returnd through should be encoded in `hexadecimal` form

### Externally Owned Accounts (EOA)

The account's address is represented by the `last 20 bytes of the public key`

### Raising an Event

A contract can declare one or more `events` that can be raised if any of its functions.

```js

event Transfer(address indexed from, address indexed to, uint256 value);

function transfer(address _to, uint256 _amount) {
        ...


        emit Transfer(msg.sender, _to, _amount);  
}


pragma solidity ^0.4.0;

contract SimpleCoin {
    mapping (address => uint256) public coinBalance;
    
    event Transfer(address indexed from, address indexed to, uint256 value);

    constructor(uint256 _initialSupply) public {
        coinBalance[msg.sender] = _initialSupply;   
    }

    function transfer(address _to, uint256 _amount) public {
        require(coinBalance[msg.sender] > _amount);
        require(coinBalance[_to] + _amount >= coinBalance[_to] );
        coinBalance[msg.sender] -= _amount;  
        coinBalance[_to] += _amount;   
        emit Transfer(msg.sender, _to, _amount);  
    }
}
```
