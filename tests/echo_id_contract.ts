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
  const projectSuffix = "myapp";
  const username = "alice";

  before(async () => {
    console.log("Initializing test accounts and PDAs...");
    adminKeypair = await createAndFundKeypair();
    productOwnerKeypair = await createAndFundKeypair();
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
        adminConfig: adminPda,
        systemProgram: SystemProgram.programId,
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
        adminConfig: adminPda,
        productOwner: productOwnerPda,
        newProductOwner: productOwnerKeypair.publicKey,
        suffixAccount: suffixPda,
        systemProgram: SystemProgram.programId,
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
    aliasOwnerKeypair = await createAndFundKeypair();
    const chainType = { evm: {} };
    const chainId = 1;
    const address = "0x1234567890123456789012345678901234567890";
    // Generate a valid public key
    const zkKeyPair = Keypair.generate();
    const zkPublicKey = zkKeyPair.publicKey.toBytes();

    console.log(
      "Generated ZK public key:",
      Buffer.from(zkPublicKey).toString("hex")
    );

    const tx = await program.methods
      .registerAlias({
        username,
        zkPublicKey: Array.from(zkPublicKey),
        initialChainMapping: {
          chainType,
          address,
          chainId,
        },
      })
      .accounts({
        productOwner: productOwnerKeypair.publicKey,
        productOwnerAccount: productOwnerPda,
        aliasAccount: aliasPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([productOwnerKeypair])
      .rpc();
    console.log("Register alias transaction:", tx);

    const aliasAccount = await program.account.aliasAccount.fetch(aliasPda);
    console.log("Alias account:", aliasAccount);
    expect(aliasAccount.owner.toBase58()).to.equal(
      productOwnerKeypair.publicKey.toBase58()
    );
    expect(aliasAccount.username).to.equal(username);
    expect(aliasAccount.productSuffix).to.equal(projectSuffix);
    expect(aliasAccount.chainMappingCount).to.equal(1);
    expect(aliasAccount.reputation.toNumber()).to.equal(10);
    expect(aliasAccount.reputationUpdatedAt.toNumber()).to.be.greaterThan(0);
    console.log("Alias registered successfully");
  });

  it("Adds a chain mapping to an existing alias", async () => {
    console.log("Testing addition of chain mapping...");
    const chainType = { svm: {} };
    const chainId = 1;
    const address = "SoLAddReSs111111111111111111111111111111";
    const merkleProof: number[][] = []; // Dummy Merkle proof
    const zkProof = {
      r: Array.from(new Uint8Array(32).fill(2)),
      s: Array.from(new Uint8Array(32).fill(3)),
    };

    const tx = await program.methods
      .addChainMapping({
        newMapping: {
          chainType,
          address,
          chainId,
        },
        merkleProof,
        zkProof,
      })
      .accounts({
        productOwner: productOwnerKeypair.publicKey,
        productOwnerAccount: productOwnerPda,
        aliasAccount: aliasPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([productOwnerKeypair])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 1_000_000,
        }),
      ])
      .rpc();
    console.log("Add chain mapping transaction:", tx);

    const updatedAliasAccount = await program.account.aliasAccount.fetch(
      aliasPda
    );
    console.log("Updated alias account:", updatedAliasAccount);
    expect(updatedAliasAccount.chainMappingCount).to.equal(2);
    console.log("Chain mapping added successfully");
  });

  it("Verifies alias ownership", async () => {
    console.log("Testing alias ownership verification...");
    const zkProof = {
      r: Array.from({ length: 32 }, (_, i) => i % 256),
      s: Array.from({ length: 32 }, (_, i) => (i + 128) % 256),
    };

    const tx = await program.methods
      .verifyAliasOwnership(zkProof)
      .accounts({
        productOwner: productOwnerKeypair.publicKey,
        productOwnerAccount: productOwnerPda,
        aliasAccount: aliasPda,
      })
      .signers([productOwnerKeypair])
      .rpc();
    console.log("Verify alias ownership transaction:", tx);
    console.log("Alias ownership verified successfully");
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
        adminConfig: adminPda,
        aliasAccount: aliasPda,
        systemProgram: SystemProgram.programId,
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
