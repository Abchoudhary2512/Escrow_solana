import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Escrow } from "../target/types/escrow";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<Escrow>;

  const buyer = anchor.web3.Keypair.generate();
  const recipient = anchor.web3.Keypair.generate();
  const arbiter = anchor.web3.Keypair.generate();

  // Fund buyer with some SOL (for testing)
  const tx = await provider.connection.requestAirdrop(buyer.publicKey, 1000000000);
  await provider.connection.confirmTransaction(tx);

  // Create an escrow account
  const escrowAccount = anchor.web3.Keypair.generate();

  // Initialize escrow
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

  console.log("Escrow initialized");
}

main().catch((err) => {
  console.error(err);
});
