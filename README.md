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

## Wallet design
The Ressua bitcoin interface requires building bitcoin wallet software. This software has to enable the user to
1) create or load a single sig wallet
2) create or load a multi sig wallet
3) advanced escrow and/or time-locking functionality

### Wallet MVP framework
Work continues to focus on building a bitcoin wallet in Rust. The current code base will be refactored into a wallet based on the MVP (model-view-controller) framework.
- Model: This will handle the Bitcoin-specific data such as managing keys, generating addresses, querying blockchain data for transactions, calculating balance, and managing state.
- View: In a Bitcoin wallet context, the view could be a graphical user interface (GUI) or a command-line interface (CLI) that allows the user to view their balance, transaction history, and request to send or receive Bitcoin.
- Controller: This layer will handle the communication between the model and view, responding to user actions (e.g., initiating a transaction) and updating the view accordingly.

## Oracle Problem
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

### Ressua API design 
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

### Testing against Revoluts or Lunar sandbox API
Revolut is a neobank that enables routing of instant SEPA transfers and also verification of them.
Access to their sandbox API and testing out the functionality is a good next step. In bitcoin, we aim for trustless systems, but with Ressua combining fiat payments into the mix, the best we can aim for is a trust-minimized system. From what I can gather, instant SEPA transfers require a banking license which neobanks like Revolut, Lunar or Wise holds. Ressua can tap into their fiat payments rails and act as the oracle source for verifying the fiat payments. 
\
\
If Ressua's code is fully open source and the bitcoin system is built in such a way there is no way for Ressua to obtain the collateral, then it is fair to assume that Alice and Bob may agree to use a verifying Ressua API to be oracle source for fiat payments.   
\
\
Alice and Ressua jointly sign a pre-signed transaction at the contract’s creation. This transaction would release the collateral to Bob if Alice defaults.
This transaction would be time-locked to execute automatically if Alice fails to make a repayment within the agreed-upon timeframe, effectively releasing the Bitcoin collateral to Bob without any further action required on his part.


[1]: https://www.oxfordreference.com/display/10.1093/acref/9780195369380.001.0001/acref-9780195369380-e-1846 "Oxford Reference"