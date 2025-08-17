// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

/**
 * @title VulnerableBank
 * @dev This contract contains multiple vulnerabilities for testing purposes.
 * WARNING: Do not use this in a production environment.
 */
contract VulnerableBank {

    mapping(address => uint256) public balances;
    address public owner;

    event Deposit(address indexed user, uint256 amount);
    event Withdrawal(address indexed user, uint256 amount);

    constructor() {
        owner = msg.sender;
    }

    /**
     * @dev Allows users to deposit Ether into the contract.
     */
    function deposit() public payable {
        require(msg.value > 0, "Deposit amount must be greater than zero");
        balances[msg.sender] += msg.value;
        emit Deposit(msg.sender, msg.value);
    }

    /**
     * @dev Allows users to withdraw their balance.
     * VULNERABILITY (Reentrancy): The balance is updated *after* the external call.
     * An attacker can recursively call this function to drain the contract.
     */
    function withdraw(uint256 _amount) public {
        require(balances[msg.sender] >= _amount, "Insufficient balance");

        // The vulnerability is here: state is changed after the external call
        (bool success, ) = msg.sender.call{value: _amount}("");
        require(success, "Transfer failed.");

        balances[msg.sender] -= _amount;
        emit Withdrawal(msg.sender, _amount);
    }
    
    /**
     * @dev A function to demonstrate integer underflow.
     * VULNERABILITY (Integer Underflow): If a user has a zero balance and calls this,
     * their balance will wrap around to a very large number.
     * Note: This is prevented in Solidity ^0.8.0, but detectable by Mythril.
     */
    function insecureWithdrawal(uint256 _amount) public {
        // This check is insufficient
        if (balances[msg.sender] >= _amount) {
            balances[msg.sender] -= _amount;
        }
    }

    /**
     * @dev Destroys the contract and sends the remaining funds to the owner.
     * VULNERABILITY (Unprotected Selfdestruct): Anyone can call this function,
     * not just the owner, leading to the destruction of the contract.
     */
    function kill() public {
        selfdestruct(payable(owner));
    }

    /**
     * @dev A simple getter function to check a user's balance.
     */
    function getBalance() public view returns (uint256) {
        return balances[msg.sender];
    }
}