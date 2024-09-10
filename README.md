
# Solana Escrow Program

This is a basic escrow program built on the Solana blockchain using the Anchor framework. The escrow program allows a buyer to lock funds in an escrow account, which are only released to a recipient if approved by an arbiter. The arbiter can also refund the funds back to the buyer.

## Features

- **Initialize Escrow**: The buyer can create an escrow account and deposit funds.
- **Approve Escrow**: The arbiter can approve the transaction, transferring the funds to the recipient.
- **Refund Escrow**: The arbiter can refund the escrow, sending the funds back to the buyer.

## Prerequisites

To deploy and test this program, you'll need:

- **Node.js**: Version 14 or higher.
- **Yarn or npm**: For managing dependencies.
- **Rust**: For compiling the Solana programs.
- **Solana CLI**: To interact with the Solana blockchain.
- **Anchor CLI**: To work with the Anchor framework.

### Install Dependencies

Ensure you have the required dependencies installed:

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# Install project dependencies
yarn install
```

## How the Escrow Works

### Initialize Escrow

The buyer initializes the escrow by providing the following:

- **Escrow Account**: An account to store the escrow funds.
- **Buyer**: The person initiating the transaction.
- **Recipient**: The recipient of the funds if the escrow is approved.
- **Arbiter**: The arbiter who has the authority to approve or refund the transaction.

The buyer deposits funds into the escrow account. The escrow account locks the funds until the arbiter takes action.

### Approve Escrow

The arbiter can approve the escrow, releasing the funds to the recipient.

### Refund Escrow

Alternatively, the arbiter can choose to refund the escrow, returning the funds to the buyer.

## Running Tests

We have included integration tests to simulate the escrow process. These tests initialize, approve, and refund the escrow using Anchor's testing framework.

### To run the tests:

1. Ensure the Solana test validator is running:
    ```bash
    solana-test-validator
    ```

2. In a separate terminal, run the tests:
    ```bash
    anchor test
    ```

### Example Tests

- **Initialize Escrow**: The escrow account is created, and funds are deposited.
- **Approve Escrow**: The arbiter approves the escrow, and the recipient receives the funds.
- **Refund Escrow**: The arbiter refunds the escrow, and the buyer gets their funds back.

```typescript
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Escrow } from "../target/types/escrow";
import { assert } from "chai";

describe("escrow", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<Escrow>;

  const buyer = anchor.web3.Keypair.generate();
  const recipient = anchor.web3.Keypair.generate();
  const arbiter = anchor.web3.Keypair.generate();
  const escrowAccount = anchor.web3.Keypair.generate();

  it("Initializes escrow", async () => {
    const tx = await provider.connection.requestAirdrop(buyer.publicKey, 1000000000);
    await provider.connection.confirmTransaction(tx);

    await program.rpc.initialize(new anchor.BN(500000000), {
      accounts: {
        escrowAccount: escrowAccount.publicKey,
        buyer: buyer.publicKey,
        recipient: recipient.publicKey,
        arbiter: arbiter.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [buyer, escrowAccount],
    });

    const escrow = await program.account.escrowAccount.fetch(escrowAccount.publicKey);
    assert.equal(escrow.amount.toString(), "500000000");
    console.log("Escrow initialized with amount:", escrow.amount.toString());
  });

  it("Approves escrow", async () => {
    await program.rpc.approve({
      accounts: {
        escrowAccount: escrowAccount.publicKey,
        buyer: buyer.publicKey,
        recipient: recipient.publicKey,
        arbiter: arbiter.publicKey,
      },
      signers: [arbiter],
    });

    console.log("Escrow approved and funds transferred to recipient.");
  });

  it("Refunds escrow", async () => {
    await program.rpc.refund({
      accounts: {
        escrowAccount: escrowAccount.publicKey,
        buyer: buyer.publicKey,
        recipient: recipient.publicKey,
        arbiter: arbiter.publicKey,
      },
      signers: [arbiter],
    });

    console.log("Escrow refunded and funds returned to buyer.");
  });
});
```

## Deploying the Program

To deploy your Solana program to the blockchain:

1. **Build the program**:
    ```bash
    anchor build
    ```

2. **Deploy the program**:
    ```bash
    anchor deploy
    ```

