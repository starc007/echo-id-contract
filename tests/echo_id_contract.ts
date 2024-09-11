import * as anchor from "@coral-xyz/anchor";
import { AnchorError, type Program } from "@coral-xyz/anchor";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { EchoIdContract } from "../target/types/echo_id_contract";
import { expect } from "chai";

describe("echo_id_contract", () => {
  console.log("setting provider");
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EchoIdContract as Program<EchoIdContract>;
  const provider = program.provider as anchor.AnchorProvider;

  async function createAndFundKeypair(): Promise<Keypair> {
    const keypair = Keypair.generate();
    const airdropSignature = await provider.connection.requestAirdrop(
      keypair.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSignature);
    return keypair;
  }

  let adminPda: PublicKey;
  let adminKeypair: Keypair;
  let projectSuffixPda: PublicKey;
  const projectSuffix = "myapp";

  before(async () => {
    console.log("Setting up test environment...");
    adminKeypair = await createAndFundKeypair();
    [adminPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("admin")],
      program.programId
    );
    [projectSuffixPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("project_suffix"), Buffer.from(projectSuffix)],
      program.programId
    );
    console.log("Admin PDA:", adminPda.toBase58());
    console.log("Admin Keypair public key:", adminKeypair.publicKey.toBase58());
    console.log("Project Suffix PDA:", projectSuffixPda.toBase58());
  });

  it("Initializes the admin", async () => {
    console.log("Starting admin initialization test...");

    try {
      const tx = await program.methods
        .initialize()
        .accounts({
          admin: adminKeypair.publicKey,
          adminConfig: adminPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([adminKeypair])
        .rpc();
      console.log("Transaction signature:", tx);

      const adminAccount = await program.account.adminConfig.fetch(adminPda);
      console.log("Admin account fetched:", adminAccount);

      expect(adminAccount.admin.toBase58()).to.equal(
        adminKeypair.publicKey.toBase58()
      );
      console.log("Admin pubkey matches the provided admin keypair");
    } catch (error) {
      console.error("Error during admin initialization:", error);
      throw error;
    }
  });

  it("Registers a project suffix", async () => {
    console.log("Starting project suffix registration test...");

    try {
      const tx = await program.methods
        .registerProjectSuffix(projectSuffix)
        .accounts({
          admin: adminKeypair.publicKey,
          adminConfig: adminPda,
          projectSuffixAccount: projectSuffixPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([adminKeypair])
        .rpc();
      console.log("Transaction signature:", tx);

      const projectSuffixAccount = await program.account.projectSuffix.fetch(
        projectSuffixPda
      );
      console.log("Project Suffix account fetched:", projectSuffixAccount);

      expect(projectSuffixAccount.suffix).to.equal(projectSuffix);
      console.log("Project suffix registered successfully");
    } catch (error) {
      console.error("Error during project suffix registration:", error);
      throw error;
    }
  });

  it("Registers an alias", async () => {
    console.log("Starting alias registration test...");

    const username = "alice";
    const chainType = "svm";
    const chainId = 1;
    const address = "SoLAn5AdDresS1111111111111111111111111111"; // Example Solana address

    const [aliasPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(username), Buffer.from("@"), Buffer.from(projectSuffix)],
      program.programId
    );
    console.log("Alias PDA:", aliasPda.toBase58());

    const aliasOwner = await createAndFundKeypair();
    console.log("Alias Owner public key:", aliasOwner.publicKey.toBase58());

    try {
      const tx = await program.methods
        .registerAlias({
          username,
          projectSuffix,
          chainId,
          chainType,
          address,
        })
        .accounts({
          owner: aliasOwner.publicKey,
          aliasAccount: aliasPda,
          projectSuffixAccount: projectSuffixPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([aliasOwner])
        .rpc();
      console.log("Transaction signature:", tx);

      const aliasAccount = await program.account.aliasAccount.fetch(aliasPda);
      console.log("Alias account fetched:", aliasAccount);

      expect(aliasAccount.owner.toBase58()).to.equal(
        aliasOwner.publicKey.toBase58()
      );
      expect(aliasAccount.username).to.equal(username);
      expect(aliasAccount.projectSuffix).to.equal(projectSuffix);
      expect(aliasAccount.chainId).to.equal(chainId);
      expect(aliasAccount.chainMappings[0].chainType.svm).to.not.be.undefined;
      expect(aliasAccount.chainMappings[0].address).to.equal(address);
      console.log("Alias registered successfully");
    } catch (error) {
      console.error("Error during alias registration:", error);
      throw error;
    }
  });

  it("Fails to register an alias with empty address", async () => {
    console.log("Starting alias registration with empty address test...");

    const username = "bob";
    const chainType = "svm";
    const chainId = 1;
    const address = ""; // Empty address

    const [aliasPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(username), Buffer.from("@"), Buffer.from(projectSuffix)],
      program.programId
    );

    const aliasOwner = await createAndFundKeypair();

    try {
      await program.methods
        .registerAlias({
          username,
          projectSuffix,
          chainId,
          chainType,
          address,
        })
        .accounts({
          owner: aliasOwner.publicKey,
          aliasAccount: aliasPda,
          projectSuffixAccount: projectSuffixPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([aliasOwner])
        .rpc();

      throw new Error("Expected an error, but the transaction succeeded");
    } catch (error) {
      console.log("Error caught as expected:", error.message);
      expect(error.message).to.include("EmptyAddress");
    }
  });
});
