# Slither Security Analysis Report

## 🚨 solc-version

- **Impact:** Informational
- **Confidence:** High

### Description
Version constraint ^0.8.13 contains known severe issues (https://solidity.readthedocs.io/en/latest/bugs.html)
	- VerbatimInvalidDeduplication
	- FullInlinerNonExpressionSplitArgumentEvaluationOrder
	- MissingSideEffectsOnSelectorAccess
	- StorageWriteRemovalBeforeConditionalTermination
	- AbiReencodingHeadOverflowWithStaticArrayCleanup
	- DirtyBytesArrayToStorage
	- InlineAssemblyMemorySideEffects
	- DataLocationChangeInInternalOverride
	- NestedCalldataArrayAbiReencodingSizeValidation.
It is used by:
	- ^0.8.13 (contract.sol#3)


---

## 🚨 reentrancy-unlimited-gas

- **Impact:** Informational
- **Confidence:** Medium

### Description
Reentrancy in VulnerableBank.ownerWithdraw(uint256) (contract.sol#25-36):
	External calls:
	- address(owner).transfer(amount) (contract.sol#33)
	Event emitted after the call(s):
	- Withdrawal(owner,amount) (contract.sol#35)


---

## 🚨 immutable-states

- **Impact:** Optimization
- **Confidence:** High

### Description
VulnerableBank.owner (contract.sol#9) should be immutable 


---

## 🚨 immutable-states

- **Impact:** Optimization
- **Confidence:** High

### Description
VulnerableBank.creationTime (contract.sol#10) should be immutable 


---

