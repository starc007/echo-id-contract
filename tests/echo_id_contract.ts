import * as anchor from "@coral-xyz/anchor";
import { AnchorError, type Program } from "@coral-xyz/anchor";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { EchoIdContract } from "../target/types/echo_id_contract";
import { expect } from "chai";

describe("echo_id_contract", () => {
  console.log("Setting up test environment...");
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EchoIdContract as Program<EchoIdContract>;
  const provider = program.provider as anchor.AnchorProvider;

  async function createAndFundKeypair(): Promise<Keypair> {
    const keypair = Keypair.generate();
    console.log(`Requesting airdrop for ${keypair.publicKey.toBase58()}`);
    const airdropSignature = await provider.connection.requestAirdrop(
      keypair.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSignature);
    console.log(`Airdrop confirmed for ${keypair.publicKey.toBase58()}`);
    return keypair;
  }

  let adminPda: PublicKey;
  let adminKeypair: Keypair;
  let productOwnerPda: PublicKey;
  let productOwnerKeypair: Keypair;
  let suffixPda: PublicKey;
  let aliasOwnerKeypair: Keypair;
  let aliasPda: PublicKey;
  const projectSuffix = "echoId";
  const username = "saurabh";

  before(async () => {
    console.log("Initializing test accounts and PDAs...");
    adminKeypair = await createAndFundKeypair();
    productOwnerKeypair = await createAndFundKeypair();
    aliasOwnerKeypair = await createAndFundKeypair();
    [adminPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("admin")],
      program.programId
    );
    [productOwnerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("product_owner"), productOwnerKeypair.publicKey.toBuffer()],
      program.programId
    );
    [suffixPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("suffix"), Buffer.from(projectSuffix)],
      program.programId
    );
    [aliasPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(username), Buffer.from("@"), Buffer.from(projectSuffix)],
      program.programId
    );
    console.log("Admin PDA:", adminPda.toBase58());
    console.log("Product Owner PDA:", productOwnerPda.toBase58());
    console.log("Suffix PDA:", suffixPda.toBase58());
    console.log("Alias PDA:", aliasPda.toBase58());
  });

  it("Initializes the admin", async () => {
    console.log("Testing admin initialization...");
    const tx = await program.methods
      .initialize()
      .accounts({
        admin: adminKeypair.publicKey,
        // adminConfig: adminPda,
        // systemProgram: SystemProgram.programId,
      })
      .signers([adminKeypair])
      .rpc();
    console.log("Admin initialization transaction:", tx);

    const adminAccount = await program.account.adminConfig.fetch(adminPda);
    console.log("Admin account:", adminAccount);
    expect(adminAccount.admin.toBase58()).to.equal(
      adminKeypair.publicKey.toBase58()
    );
    console.log("Admin initialized successfully");
  });

  it("Registers a product owner with suffix", async () => {
    console.log("Testing product owner registration with suffix...");
    const tx = await program.methods
      .registerProductOwner(projectSuffix)
      .accounts({
        admin: adminKeypair.publicKey,
        // adminConfig: adminPda,
        // productOwner: productOwnerPda,
        newProductOwner: productOwnerKeypair.publicKey,
        // suffixAccount: suffixPda,
        // systemProgram: SystemProgram.programId,
      })
      .signers([adminKeypair])
      .rpc();
    console.log("Register product owner transaction:", tx);

    const productOwnerAccount = await program.account.productOwner.fetch(
      productOwnerPda
    );
    console.log("Product owner account:", productOwnerAccount);
    expect(productOwnerAccount.address.toBase58()).to.equal(
      productOwnerKeypair.publicKey.toBase58()
    );
    expect(productOwnerAccount.isActive).to.be.true;
    expect(productOwnerAccount.suffix).to.equal(projectSuffix);

    const suffixAccount = await program.account.productOwner.fetch(suffixPda);
    console.log("Suffix account:", suffixAccount);
    expect(suffixAccount.address.toBase58()).to.equal(
      productOwnerKeypair.publicKey.toBase58()
    );
    expect(suffixAccount.isActive).to.be.true;
    expect(suffixAccount.suffix).to.equal(projectSuffix);

    console.log("Product owner registered with suffix successfully");
  });

  it("Registers an alias", async () => {
    console.log("Testing alias registration...");
    const publicKey = aliasOwnerKeypair.publicKey.toBytes();

    console.log(
      "Generated public key:",
      Buffer.from(publicKey).toString("hex")
    );

    const initialChainMapping = {
      chainType: { evm: {} },
      address: "0x1234567890123456789012345678901234567890",
      chainId: 1,
    };

    const tx = await program.methods
      .registerAlias({
        username,
        suffix: projectSuffix,
        initialChainMapping,
      })
      .accounts({
        user: aliasOwnerKeypair.publicKey,
        suffixAccount: suffixPda,
        aliasAccount: aliasPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([aliasOwnerKeypair])
      .rpc();
    console.log("Register alias transaction:", tx);

    const aliasAccount = await program.account.aliasAccount.fetch(aliasPda);
    console.log("Alias account:", aliasAccount);
    expect(aliasAccount.owner.toBase58()).to.equal(
      aliasOwnerKeypair.publicKey.toBase58()
    );
    expect(aliasAccount.username).to.equal(username);
    expect(aliasAccount.productSuffix).to.equal(projectSuffix);
    expect(aliasAccount.reputation.toNumber()).to.equal(10);
    expect(aliasAccount.reputationUpdatedAt.toNumber()).to.be.greaterThan(0);
    console.log("Alias registered successfully");
  });

  it("Adds a chain mapping to an existing alias", async () => {
    console.log("Testing addition of chain mapping...");

    // Fetch the alias account to get the correct owner
    const aliasAccount = await program.account.aliasAccount.fetch(aliasPda);

    const newMapping = {
      chainType: { svm: {} },
      address: "SoLAddReSs111111111111111111111111111111",
      chainId: 512,
    };

    try {
      const tx = await program.methods
        .addChainMapping({
          newMapping,
        })
        .accounts({
          aliasOwner: aliasOwnerKeypair.publicKey,
          aliasAccount: aliasPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([aliasOwnerKeypair])
        .rpc();
      console.log("Add chain mapping transaction:", tx);

      const updatedAliasAccount = await program.account.aliasAccount.fetch(
        aliasPda
      );
      console.log("Updated alias account:", updatedAliasAccount);
      expect(updatedAliasAccount.chainMappings.length).to.equal(2);
      expect(updatedAliasAccount.chainMappings[1]).to.deep.equal(newMapping);
      console.log("Chain mapping added successfully");
    } catch (error) {
      console.error("Error details:", error);
      if (error instanceof anchor.web3.SendTransactionError) {
        console.error("Transaction logs:", error.logs);
      }
      throw error;
    }
  });

  it("Updates reputation for an alias (admin only)", async () => {
    console.log("Testing reputation update...");
    const reputationChange = 20;

    const tx = await program.methods
      .updateReputation(
        username,
        projectSuffix,
        new anchor.BN(reputationChange)
      )
      .accounts({
        admin: adminKeypair.publicKey,
      })
      .signers([adminKeypair])
      .rpc();
    console.log("Update reputation transaction:", tx);

    const updatedAliasAccount = await program.account.aliasAccount.fetch(
      aliasPda
    );
    console.log(
      "Updated alias account after reputation change:",
      updatedAliasAccount
    );
    expect(updatedAliasAccount.reputation.toNumber()).to.equal(30); // 10 (initial) + 20 (change)
    console.log("Reputation updated successfully");
  });
});
