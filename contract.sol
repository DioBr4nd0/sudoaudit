// SPDX-License-Identifier: MIT
// A simple contract with intentional vulnerabilities for testing static analysis tools.
pragma solidity ^0.8.20;

contract VulnerableBank {

    // Issue: This could be 'constant' to save gas.
    // Slither will detect: 'locked-ether' or 'state-visibility'.
    address public owner = msg.sender;
    uint public creationTime = block.timestamp; // Issue: Could be immutable.

    mapping(address => uint) public balances;

    event Deposit(address indexed user, uint amount);
    event Withdrawal(address indexed user, uint amount);

    // Users can deposit Ether into the bank.
    function deposit() public payable {
        require(msg.value > 0, "Must deposit more than 0 Ether.");
        balances[msg.sender] += msg.value;
        emit Deposit(msg.sender, msg.value);
    }

    // Only the owner can withdraw funds.
    function ownerWithdraw(uint amount) public {
        require(msg.sender == owner, "Only the owner can withdraw.");
        require(balances[owner] >= amount, "Insufficient balance.");

        balances[owner] -= amount;

        // Issue: Unchecked return value for 'transfer'.
        // Slither will detect: 'unchecked-send'
        payable(owner).transfer(amount);

        emit Withdrawal(owner, amount);
    }

    // A function that doesn't modify state but isn't marked 'view'.
    // Slither will detect: 'function-visibility'
    function getBalance(address user) public returns (uint) {
        return balances[user];
    }
}
