# 🌟 Structured Issues for Soroscope

We have refined our issue list to ensure every contribution is meaningful and impactful. Below are the open issues for you to tackle, structured with clear context, expectations, and timeframes.

---

## 🟢 Trivial Issues (100 Points)
*Small, clearly bounded changes.*

### #41 [Contract] docs: Add Rustdoc to Liquidity Pool

**Impact**  
The Liquidity Pool contract is the core of our DEX, but it lacks inline documentation. This makes it harder for new contributors to understand the logic. Adding Rustdoc will improve developer onboarding and code maintainability.

**Context**  
The `LiquidityPool` contract uses the Soroban SDK. We need standard Rustdoc comments (`///`) for all public functions and structs.

**Review**
- Check `contracts/liquidity_pool/src/lib.rs`.
- Ensure `initialize`, `deposit`, `swap`, and `withdraw` have clear descriptions of parameters and return values.

**Guidelines**
- **Timeframe:** 48 hours
- **Assignment:** Required before starting.

**Complexity:** Trivial (100 pts)

---

### #42 [Contract] test: Add Zero-Amount Deposit Test

**Impact**  
Edge cases in DeFi contracts can lead to unexpected behavior. We need to verify that our `deposit` function handles zero amounts correctly (either by rejecting them or handling them gracefully without panicking or creating dust).

**Context**  
Current tests cover standard flows. We need a specific test case for `amount_a = 0` and `amount_b = 0`.

**Review**
- Add a test case in `contracts/liquidity_pool/src/test.rs`.
- Verify the contract behavior (it might currently allow it, or fail; document the behavior).

**Guidelines**
- **Timeframe:** 48 hours
- **Assignment:** Required before starting.

**Complexity:** Trivial (100 pts)

---

### #43 [Contract] refactor: Use 'const' for Error Codes

**Impact**  
Magic numbers in error reporting make debugging difficult. Refactoring error codes to use explicit constants or enum variants improves readability.

**Context**  
The `Error` enum in `contracts/liquidity_pool/src/lib.rs` is already an enum, but some logic might still be using raw numbers or implicit casts. Review the codebase for any magic numbers in error handling.

**Guidelines**
- **Timeframe:** 48 hours
- **Assignment:** Required before starting.

**Complexity:** Trivial (100 pts)

---

### #44 [Contract] test: Verify Event Emission in Token Contract

**Impact**  
Events are crucial for indexers and UIs. We need to ensure that the token contract (or our usage of it) emits the expected events during operations.

**Context**  
The `LiquidityPool` emits `Deposit`, `Swap`, and `Withdraw` events. Verify these are actually emitted with the correct data in `test.rs`.

**Review**
- Extend `test_events` in `contracts/liquidity_pool/src/test.rs`.
- Assert the *content* of the events match the transaction details.

**Guidelines**
- **Timeframe:** 48 hours
- **Assignment:** Required before starting.

**Complexity:** Trivial (100 pts)

---

## 🟡 Medium Issues (150 Points)
*Standard features touching multiple parts of the codebase.*

### #45 [Contract] feat: Implement 'approve' in Liquidity Pool

**Impact**  
To be fully compatible with the Soroban Token Interface (SEP-41), our LP Share token needs to support the `approve` pattern (allowance), allowing third-party contracts to spend user shares.

**Requirements**  
- Implement `approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32)`.
- Store allowances in `Persistent` storage.
- Add `allowance` read function.
- Update `transfer_from` (see #46).

**Guidelines**
- **Timeframe:** 96 hours (4 days)
- **Assignment:** Required before starting.

**Complexity:** Medium (150 pts)

---

### #46 [Contract] feat: Implement 'transfer_from' in Liquidity Pool

**Impact**  
Completes the standard token interface. Allows a spender to transfer tokens on behalf of an owner, up to the approved allowance.

**Requirements**  
- Implement `transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128)`.
- Check and update allowance.
- Perform transfer logic.

**Guidelines**
- **Timeframe:** 96 hours (4 days)
- **Assignment:** Required before starting.

**Complexity:** Medium (150 pts)

---

### #47 [Contract] feat: Add 'burn' to Liquidity Pool

**Impact**  
While `withdraw` burns shares to return assets, a raw `burn` function allows users to voluntarily destroy shares (e.g., for deflationary mechanisms or proof-of-burn).

**Requirements**  
- Implement `burn(e: Env, from: Address, amount: i128)`.
- Reduce `TotalShares` and user balance.
- Do *not* return assets (this is the difference from withdraw).
- Emit `Burn` event.

**Guidelines**
- **Timeframe:** 72 hours (3 days)
- **Assignment:** Required before starting.

**Complexity:** Medium (150 pts)

---

### #48 [Contract] feat: Add Pausable Functionality

**Impact**  
In case of a security emergency, an admin should be able to pause deposits and swaps to prevent loss of funds.

**Requirements**  
- Add `Admin` state to the contract (initialized in `initialize`).
- Add `set_paused(e: Env, paused: bool)`.
- Add checks in `deposit`, `swap`, `withdraw` to revert if `paused` is true.

**Guidelines**
- **Timeframe:** 72 hours (3 days)
- **Assignment:** Required before starting.

**Complexity:** Medium (150 pts)

---

## 🔴 High Issues (200 Points)
*Complex engineering work.*

### #49 [Contract] feat: Admin Fee Control

**Impact**  
Currently, the 0.3% fee is hardcoded. We need a way to adjust this fee (within reasonable limits) and potentially direct a portion of it to a protocol treasury.

**Requirements**  
- Store `FeeBasisPoints` in storage (default 30).
- Valid range: 0 to 100 (0% to 1%).
- Update `swap` formula to use the stored fee.
- Add admin function `set_fee`.

**Guidelines**
- **Timeframe:** 120 hours (5 days)
- **Assignment:** Required before starting.

**Complexity:** High (200 pts)

---

### #50 [Contract] test: Fuzz Testing for Swap Formula

**Impact**  
The constant product formula `x * y = k` is susceptible to rounding errors or overflow in edge cases. Fuzz testing ensures mathematical correctness across the entire range of valid inputs.

**Requirements**  
- Use `soroban-sdk` test fuzzing capabilities or `proptest`.
- Define invariants: `k` must never decrease (except for rounding).
- Test with max `i128` values.

**Guidelines**
- **Timeframe:** 168 hours (7 days)
- **Assignment:** Required before starting.

**Complexity:** High (200 pts)

---

## ✅ Completed Issues

### #51 [Contract] feat: Implement Liquidity Pool Factory

**Status:** Completed via #51.
**Summary:** Implemented a factory contract that deploys unique `LiquidityPool` contracts using deterministic addresses (salt).
