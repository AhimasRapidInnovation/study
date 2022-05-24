
# Decentralized Application


Funds are routed to the seller only when customer confirmed safe delivery of goods is called `smart contract`

`dapps` is web Application that increases the transparancy around commercial transactions.

Objective of `Dapps` is minimize the or eliminate the need for any trust between the participants in a system interaction

Blockchain is a database

Blockchain database is based on data structure linked list (chain of block)

Block contains set of transaction, each one digitally signed , and it is `immutable` 

## Ethereum network

Participant nodes host a `blockchain database` and a piece of software called a `node client` which allows a node to communicate with other nodes 

In ethereum network each node is a server to other nodes also it is also client to other nodes

### Client

Communicate with each other through `p2p` protocol called `wire` 

Protocol enforces a standard way of sending data throughout network

Client also execute application code  hosted on blockchain database so the ethereum known as `programmable blockchain`

## Network nodes roles

The Ethereum network includes two main types of nodes. Full nodes process transactions passively and can read, but can’t write on, the blockchain. Mining nodes process transactions actively: they validate the correctness of transactions as full nodes do, but they also assemble transactions into new blocks that are appended onto the blockchain.

 Two main types of nodes

1. `Full Node`

    It is a standard setup allows them to process the `transaction` passively 

    Can read from blockchain database but cannot create a new blockchain block

    Can append `blocks` received from peer nodes to the local blockchain

    Do execute transaction but only to verify the correctness of the blockchain blocks they receive from peer nodes.

2. `Mining Nodes`

Process transaction activly 

Group and store transactios in new blockchain blocks

Rewarded in ether  

Performs computationally intensive and energy demanding work

And then probgate the new block to the rest of `p2p` network


* Peer node keep probgating the transaction until the transaction hits mining nodes.

## Dapp Terminology

Vitalik blog

### Smart Contract

Two or more parties that involves an exchange of digital assets

One or more parties allocate digital assets to the contract at its initiation

### Autonomous Agent

    Software entity that interacts autonomously with external software service and can reconfigure or even reprogram itself following verified changes in external resources 

### Decentralize Organization 

Contains assets and different classes of individual, investor, employee

Investor control the organization by owning a part of it through the purchase of the share 

Interaction between some class of individuals are influnced by whether they control the orgainzation

### Decentralized Autonomous Organization (DAO)

Both `DO` and `Autonomous Agent` 

Software entity interacts using `AG` and individual involed with `DAO` interact 

Interaction protocals are programmed in `smart contract` 

### Decentralized Autonomous Corporation (DAC)

It is `DAO` that can be partially owned trough purchase of shares



## Solidity

`contract` it is a type to define a contract and it has `state` variables, a constructor, functions, and events

```js
pragma solidity ^0.4.0;                                          

contract SimpleCoin {                                            

  mapping (address => uint256) public coinBalance;               

  constructor() public {                                         
    coinBalance[0x14723A09ACff6D2A60DcdF7aA4AFf308FDDC160C] = 10000;         
  }

  function transfer(address _to, uint256 _amount) public {       
    coinBalance[msg.sender] -= _amount;                          
    coinBalance[_to] += _amount;                                 
  }
}
```

