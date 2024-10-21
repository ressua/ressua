# Ressua 
**Res sua** *(rās sū´a)*. *n.*  
“One’s own thing”. “One’s own property” [1]

**Ressua** is an open-source, peer-to-peer (P2P) platform that enables decentralized Bitcoin-collateralized loans. It facilitates secure lending and borrowing, leveraging Bitcoin as collateral in a trustless manner. Built on the principles of decentralization, the project aims to provide users with direct financial access, maintaining privacy and security throughout the lending process.

For more information, visit the [Ressua website](https://www.ressua.com).

### Table of Contents

- [Overview](#overview)
- [Features](#features)
- [How It Works](#how-it-works)

## Overview

The goal of **Ressua** is to connect lenders and borrowers in a trust-minimized environment using Bitcoin. Borrowers can pledge Bitcoin as collateral to secure loans, while lenders earn interest by providing liquidity. No central entity controls the process, and users remain in control of their funds at all times.

## Features

- **Decentralized P2P Loans**: Borrowers and lenders connect directly without intermediaries.
- **Bitcoin Collateralization**: Borrowers use Bitcoin as collateral to obtain loans.

## How It Works

1. **Loan Request**: A borrower initiates a request, specifying the loan amount, interest rate, and duration.
2. **Collateralization**: The borrower deposits Bitcoin into a multisig escrow contract as collateral for the loan.
3. **Matching**: A lender reviews the terms and agrees to provide the loan.
4. **Loan Execution**: Once matched, the bitcoin is locked as collateral and the lender releases the loan amount to the borrower, obtaining a key to the multisig escrow.
5. **Repayment**:
If the borrower repays the loan in full by the agreed-upon date, the collateral is released back to them.

**WIP Logs**
Ideation work continues as the fiat repayment is faced by the yet to be solved *Oracle Problem* - where off-chain events are difficult to verify in an isolated bitcoin-based system. Perhaps time-locked cosigned transactions of partial collateral returning to the borrower *N* days after declaring a fiat repayment in period *t* solves this, where a lender dispute opened avoids this, and a false dispute opening is punished to keep incentives aligned.   

[1]: https://www.oxfordreference.com/display/10.1093/acref/9780195369380.001.0001/acref-9780195369380-e-1846 "Oxford Reference"