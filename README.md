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

## WIP Logs
Ideation work continues as the fiat repayment is faced by the yet to be solved *Oracle Problem* - where off-chain events are difficult to verify in an isolated bitcoin-based system. Perhaps time-locked cosigned transactions of partial collateral returning to the borrower *N* days after declaring a fiat repayment in period *t* solves this, where a lender dispute opened avoids this, and a false dispute opening is punished to keep incentives aligned.   
\
Potential solution to Oracle Problem could be connecting to Swift or SEPA APIs of payment processing companies like Stripe and Wise or even traditional banks:
\
\
Request flow for confirming a SEPA transfer via a bank API:
```json
POST /api/sepa/transfer
{
    "debtorIBAN": "DE89370400440532013000",
    "creditorIBAN": "FR1420041010050500013M02606",
    "amount": "100.00",
    "currency": "EUR",
    "reference": "Invoice123"
}
Response:
{
    "transferId": "123456789",
    "status": "PENDING"
}
Then, to confirm the transfer:

GET /api/sepa/transfer/status?transferId=123456789
Response:

{
    "transferId": "123456789",
    "status": "CONFIRMED"
}
```

## API design 
Design ideas for a trusted oracle Ressua API to verify SEPA transfers might be the way to confirm fiat transfers.
I.e. in such setup we have: 
```
Step (0) Contract creation:
The collateral bitcoin is transferred to a Ressua multisig vault by Alice 
Alice --- Ressua API --- Bob 
```
```
Step (1) Loan transfer:
Once Bob has verified the collateral bitcoin, Bob POSTs loan transfer
Alice --- Ressua API <-- Bob 
```
```
Step (2) Ressua API verifies:
Ressua verifies fiat loan arrival in Alice's bank account 
Alice <-- Ressua API --- Bob 
```
```
Step (3....N) Repayment transfers & verifications:
Alice POSTs repayment transfer
Alice --> Ressua API --- Bob
Ressua verifies fiat repayment arrival in Bob's bank account 
Alice --- Ressua API --> Bob 
```
```
Step (N+1) Final bitcoin collateral release
Ressua verifies fiat loan arrival in Alice's bank account 
Alice <-- Ressua API --- Bob 
```


[1]: https://www.oxfordreference.com/display/10.1093/acref/9780195369380.001.0001/acref-9780195369380-e-1846 "Oxford Reference"