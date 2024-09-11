import * as anchor from "@coral-xyz/anchor";
import { AnchorError, type Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { EchoIdContract } from "../target/types/echo_id_contract";

describe("echo_id_contract", () => {
  console.log("setting provider");
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EchoIdContract() as Program<EchoIdContract>;
  const provider = program.provider as anchor.AnchorProvider;

  const payer = provider.wallet;
  const alias = "testalias";
  const chainType = "evm";
  const chainId = 1;
  const reputationChange = 10;

  console.log("payer", payer.publicKey.toBase58());
  console.log("program", program.programId.toBase58());
  // balance check
  it("Should fund the payer account", async () => {
    const lamports =
      await provider.connection.getMinimumBalanceForRentExemption(0);
    await provider.connection.requestAirdrop(payer.publicKey, lamports);
    const balance = await provider.connection.getBalance(payer.publicKey);
    console.log("payer balance", balance);
    assert.ok(balance > 0);
  });
});
