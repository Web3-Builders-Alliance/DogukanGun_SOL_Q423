import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import wallet from '../wba-wallet.json';

describe("anchor-vault", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;

  const connection = anchor.getProvider().connection;
	const signer = Keypair.fromSecretKey(new Uint8Array(wallet));

  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), signer.publicKey.toBuffer()],
    program.programId
  )[0];
  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block
    })
    return signature
  }

  const log = async (signature: string): Promise<string> => {
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`);
    return signature;
  }

  it("Airdrop", async () => {
    await connection.requestAirdrop(signer.publicKey, LAMPORTS_PER_SOL * 10)
      .then(confirm)
      .then(log)
  })

  it("Deposit", async () => {
      const tx = await program.methods.deposit(
        new BN(LAMPORTS_PER_SOL),
      )
      .accounts({
        owner: signer.publicKey,
        vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc()
      .then(confirm)
      .then(log)
    console.log("Transactions signature is: ", tx)
  })
  it("Close", async () => {
    const tx = await program.methods.close(
      new BN(LAMPORTS_PER_SOL),
    )
      .accounts({
        owner: signer.publicKey,
        vault,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc().then(confirm).then(log)
    console.log("Transactions signature is: ", tx)
  })
});
