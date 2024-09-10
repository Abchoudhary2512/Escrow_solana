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
