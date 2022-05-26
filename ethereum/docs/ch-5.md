# Programming Smart Contracts

## EVM Contract Language

```js
pragma solidity ^0.4.24;
contract AuthorizedToken {
    
  enum UserType {TokenHolder, Admin, Owner}        1

  struct AccountInfo {                             2
    address account;
    string firstName;
    string lastName;
    UserType userType;
  }
    
  mapping (address => uint256) 
     public tokenBalance;                          3
  mapping (address => AccountInfo) 
     public registeredAccount;                     3
  mapping (address => bool) 
     public frozenAccount;                         3
    
  address public owner;                            3
    
  uint256 public constant maxTranferLimit = 15000;
    
  event Transfer(address indexed from, 
     address indexed to, uint256 value);           4
  event FrozenAccount(address target, 
     bool frozen);                                 4
    
  modifier onlyOwner {                             5
    require(msg.sender == owner);
    _;
  }
    
  constructor(uint256 _initialSupply) public {     6
    owner = msg.sender;

    mintToken(owner, _initialSupply);   
  }

  function transfer(address _to, uint256 _amount) 
    public {                                            7
    require(checkLimit(_amount));
    //... 
    emit Transfer(msg.sender, _to, _amount);
  }  
  function registerAccount(address account, 
    string firstName,                                   7
    string lastName, bool isAdmin) public onlyOwner {
    //...
  }
    
  function checkLimit(uint256 _amount) private
    returns (bool) { 
    if (_amount < maxTranferLimit) 
        return true;
    return false;
  }

  function validateAccount(address _account) 
    internal
    returns (bool) { 
    if (frozenAccount[_account] && tokenBalance[_account] > 0) 
        return true;
    return false;
  }

  function mintToken(address _recipient, 
    uint256  _mintedAmount)
    onlyOwner public  { 
    tokenBalance[_recipient] += _mintedAmount;
    emit Transfer(owner, _recipient, _mintedAmount);
    }

  function freezeAccount(address target, 
    bool freeze)                                
    onlyOwner public  { 
    frozenAccount[target] = freeze; 
    emit FrozenAccount(target, freeze);
  }
}
```

## State Variables

Holds contract state 

## Events

Contract member that interacts with `EVM` transaction logs and whose invocation is then probagated to clients subscribe to it 

## Enum

enum UserType {TokenHolder, Admin, Owner}

## Structs

```cpp
struct AccountInfo {
    address account;
    string firstName;
    string lastName;
    UserType userType;
}
```

## Function Modifier

Allows to modify the behavior of a function 

## Value Types

A `value type` variable is stored on the `EVM` stack
allocates a `single memory space` to hold its value

Uses `Copy` semantic

## Integer Types

`int` and `uint` size ranges from `8-256`

### Implicit and explict type conversion

```cpp
int16 public newSmallNumber = int16(bigNumber;                       
uint64 public newMediumPositiveNumber = uint64(mediumNegativeNumber);
```

## Static byte array

`byte` is an array of single byte and equivalnt to `byte1`

Can declare byte arrays of a `fixed size` with size raging from `1 to 32`

## Address

An `object` containing upto `40 hexadecimal` digits prefixed by `0x` holds 2 bytes

>address ownerAddress = 0x10abb5EfEcdC09581f8b7cb95791FE2936790b4E;

does have `balance` property on address

>destinationAddress.transfer(10);
And fuctions

## Reference Types

Accessed through their `reference`

Can only store in `2 data location`

1. `Memory`

    Values aren't persisted permanently and only live in memory

2. `Storage`

    Values are persisted permanently on the blockchain, like state variable

```js
pragma solidity ^0.4.0;
contract ReferenceTypesSample {
    uint[] storageArray; // storage
    
    function f(uint[] fArray ) {} // fArray memory
    function g(uint[] storage gArray) internal {}
    function h(uint[] memory hArray) internal  {}
}
```

`byte` is an unlimited byte array and is a referece type also supprots `.length` and `push()`

If array is exposed as a public state variable, its getter accepts the array positional index as an input

## string

Equivalant to bytes with no `lenght` and `push` members

>string name = "Roberto";

## Mapping

It is special `reference` type

Can only use in the `storage data location` 

Can only declare as a `state variable` or `storage referece type` 

>mapping(address => int) public coinBalance;

has `contains-key()`


## Global Namespace

Set of implicitly declared variables and functions that can be referenced in cotract 

`block` - holds informattion about last blockchain block

`msg` - Incoming message

`tx` - transaction data

`this` - reference to current contract

Can use `this` to call `internal` functions as if they were defined as `external`

`now` - time associated with a creation of the latest block, expressed as `epoch`

## State Variable

`public` compiler generates a getter function for each public state variable

`internal` The contract and any inherited contract can access the internal state variable

`private` Only member of the same contract , not inherited contracts access private state variables

> **underscore prefix is used to identify parameters and local variables**

## Function Modifiers

Alters the behavior of a function by performing some `pre-` and `postprocessing` around the execution of the function using it.

## Event

Allows a contract to notify another contract or a contract client 

```js
event Transfer(address indexed from, 
          address indexed to, uint256 value);
    
// In function

emit Transfer(msg.sender, _to, _amount);

```

Events are also used for `logging` purpose

Events are logged on the transaction log of the blockchain, Can retrive them for later analysis

To allow `quick` retrival, events are `indexed` against a key 

## Contract Owner

It is the account from which the contract gets deployed 

