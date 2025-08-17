# Mythril Security Analysis Report

## 🔥 External Call To User-Supplied Address

- **Severity:** Low

### Description
A call to a user-supplied address is executed.
An external message call to an address specified by the caller is executed. Note that the callee account might contain arbitrary code and could re-enter any function within this contract. Reentering the contract in an intermediate state may lead to unexpected behaviour. Make sure that no state modifications are executed after this call and/or reentrancy guards are in place.

---

## 🔥 State access after external call

- **Severity:** Medium

### Description
Read of persistent state following external call
The contract account state is accessed after an external call to a user defined address. To prevent reentrancy issues, consider accessing the state only before the call, especially if the callee is untrusted. Alternatively, a reentrancy lock can be used to prevent untrusted callees from re-entering the contract in an intermediate state.

---

## 🔥 State access after external call

- **Severity:** Medium

### Description
Write to persistent state following external call
The contract account state is accessed after an external call to a user defined address. To prevent reentrancy issues, consider accessing the state only before the call, especially if the callee is untrusted. Alternatively, a reentrancy lock can be used to prevent untrusted callees from re-entering the contract in an intermediate state.

---

## 🔥 Unprotected Selfdestruct

- **Severity:** High

### Description
Any sender can cause the contract to self-destruct.
Any sender can trigger execution of the SELFDESTRUCT instruction to destroy this contract account. Review the transaction trace generated for this issue and make sure that appropriate security controls are in place to prevent unrestricted access.

---

