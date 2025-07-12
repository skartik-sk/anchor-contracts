import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
  getAccount,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { assert } from "chai";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("Escrow Program Tests", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Escrow as Program<Escrow>;
  const provider = anchor.getProvider();
  const connection = provider.connection;

  // Test accounts
  let maker: anchor.web3.Keypair;
  let taker: anchor.web3.Keypair;
  let mintA: anchor.web3.PublicKey;
  let mintB: anchor.web3.PublicKey;
  let makerTokenAccountA: anchor.web3.PublicKey;
  let makerTokenAccountB: anchor.web3.PublicKey;
  let takerTokenAccountA: anchor.web3.PublicKey;
  let takerTokenAccountB: anchor.web3.PublicKey;
  let escrowAccount: anchor.web3.PublicKey;
  let vaultAccount: anchor.web3.PublicKey;

  // Test constants
  const escrowId = new anchor.BN(1);
  const depositAmount = new anchor.BN(1000);
  const demandAmount = new anchor.BN(500);

  before(async () => {
    // Create test users
    maker = anchor.web3.Keypair.generate();
    taker = anchor.web3.Keypair.generate();

    // Airdrop SOL to test accounts
    await connection.requestAirdrop(
      maker.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.requestAirdrop(
      taker.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );

    // Wait for airdrops to confirm
    await new Promise((resolve) => setTimeout(resolve, 500));

    // Create test tokens
    mintA = await createMint(
      connection,
      maker,
      maker.publicKey,
      null,
      9 // 9 decimals
    );
    mintB = await createMint(
      connection,
      taker,
      taker.publicKey,
      null,
      9 // 9 decimals
    );

    // Create token accounts
    makerTokenAccountA = await createAccount(
      connection,
      maker,
      mintA,
      maker.publicKey
    );
    makerTokenAccountB = await createAccount(
      connection,
      maker,
      mintB,
      maker.publicKey
    );

    takerTokenAccountA = await createAccount(
      connection,
      taker,
      mintA,
      taker.publicKey
    );

    takerTokenAccountB = await createAccount(
      connection,
      taker,
      mintB,
      taker.publicKey
    );

    // Mint tokens to accounts
    await mintTo(
      connection,
      maker,
      mintA,
      makerTokenAccountA,
      maker.publicKey,
      10000 * Math.pow(10, 9) // 10,000 tokens
    );
    await mintTo(
      connection,
      taker,
      mintB,
      takerTokenAccountB,
      taker.publicKey,
      10000 * Math.pow(10, 9) // 10,000 tokens
    );

    console.log("Setup complete:");
    console.log("Maker:", maker.publicKey.toString());
    console.log("Taker:", taker.publicKey.toString());
    console.log("Mint A:", mintA.toString());
    console.log("Mint B:", mintB.toString());
    console.log("ata a x", makerTokenAccountA.toString());
    console.log("ata a y", makerTokenAccountB.toString());
    console.log("ata b x", takerTokenAccountA.toString());
    console.log("ata b y", takerTokenAccountB.toString());
  });

  it("1. Create Escrow - Maker deposits Token A and requests Token B", async () => {
    console.log("\n=== TEST 1: Creating Escrow ===");

    const tx = await program.methods
      .makeEscrow(
        escrowId, // unique ID for this escrow
        depositAmount, // amount of Token A to deposit
        demandAmount // amount of Token B demanded
      )
      .accounts({
        signer: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();

    console.log("Transaction signature:", tx);
     [escrowAccount] =
      await anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("escrow"),
          maker.publicKey.toBuffer(),
          escrowId.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );

    console.log("escrowacc: ", escrowAccount);
    // Verify escrow account was created
    const escrowDataall = await program.account.escrow.all();
    console.log("data", escrowDataall);
    const escrowData = await program.account.escrow.fetch(escrowAccount);
    assert.equal(
      new anchor.BN(escrowData.ids).toNumber(),
      escrowId.toNumber(),
      "id not same"
    );
    assert.equal(
      new anchor.BN(escrowData.demand).toNumber(),
      demandAmount.toNumber(),
      "demand not same"
    );
    assert.equal(escrowData.mintA.toString(), mintA.toString());
    assert.equal(escrowData.mintB.toString(), mintB.toString());
    assert.equal(escrowData.signer.toString(), maker.publicKey.toString());
    // Verify tokens were transferred to vault
    vaultAccount = await getAssociatedTokenAddress(
      mintA,
      escrowAccount,
      true

      // owner (the escrow PDA)
    );
    const vaultBalance = await getAccount(connection, vaultAccount);
    assert.equal(vaultBalance.amount.toString(), depositAmount.toString());
    console.log("varified till now");

    console.log("✅ Escrow created successfully!");
    console.log("Escrow ID:", escrowData.ids.toString());
    console.log("Deposited:", depositAmount.toString(), "Token A");
    console.log("Demanding:", demandAmount.toString(), "Token B");
  });

  it("2. Take Escrow - Taker provides Token B and receives Token A", async () => {
    console.log("\n=== TEST 2: Taking Escrow ===");

    const tx = await program.methods
    .take()
    .accounts({
      signer: taker.publicKey,           // Taker is the signer
      maker: maker.publicKey,            // Original maker                    // Token A mint              // Token B mint   // Maker's Token B account (receives Token B)
      escrow: escrowAccount,             // Escrow PDA             // Vault containing Token A
      tokenProgram: TOKEN_PROGRAM_ID,

    })
    .signers([taker])
    .rpc();

    console.log("Transaction signature:", tx);

    // Verify token exchanges
    const takerTokenABalance = await getAccount(connection, takerTokenAccountA);
    const makerTokenBBalance = await getAccount(connection, makerTokenAccountB);

    assert.equal(
      takerTokenABalance.amount.toString(),
      depositAmount.toString()
    );
    assert.equal(makerTokenBBalance.amount.toString(), demandAmount.toString());

    console.log("✅ Escrow taken successfully!");
    console.log("Taker received:", depositAmount.toString(), "Token A");
    console.log("Maker received:", demandAmount.toString(), "Token B");

    // Verify escrow account was closed
    try {
      await program.account.escrow.fetch(escrowAccount);
      assert.fail("Escrow account should have been closed");
    } catch (error) {
      console.log("✅ Escrow account closed successfully");
    }
  });

  it("3. Create and Refund Escrow - Maker cancels and gets tokens back", async () => {
    console.log("\n=== TEST 3: Creating and Refunding Escrow ===");

    const newEscrowId = new anchor.BN(2);

    // Derive new PDA addresses
    const [newEscrowAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        newEscrowId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );


    // Create new escrow
    await program.methods
      .makeEscrow(newEscrowId, depositAmount, demandAmount)
      .accounts({
        signer: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();

    console.log("New escrow created");

    // Get maker's token balance before refund
    const balanceBefore = await getAccount(connection, makerTokenAccountA);
    console.log(
      "Maker's Token A balance before refund:",
      balanceBefore.amount.toString()
    );

    // Refund the escrow
    const refundTx = await program.methods
      .refundIt()
      .accounts({
        signer: maker.publicKey,
        mintA: mintA,
        escrow:newEscrowAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();

    console.log("Refund transaction signature:", refundTx);

    // Verify tokens were returned
    const balanceAfter = await getAccount(connection, makerTokenAccountA);
    console.log(
      "Maker's Token A balance after refund:",
      balanceAfter.amount.toString()
    );

    assert.equal(balanceAfter.amount - balanceBefore.amount, depositAmount);

    console.log("✅ Refund successful!");
    console.log("Maker received back:", depositAmount.toString(), "Token A");

    // Verify escrow account was closed
    try {
      await program.account.escrow.fetch(newEscrowAccount);
      assert.fail("Escrow account should have been closed");
    } catch (error) {
      console.log("✅ Escrow account closed successfully");
    }
  });

  it("4. Error Case - Non-owner cannot refund", async () => {
    console.log("\n=== TEST 4: Error Case - Unauthorized Refund ===");

    const newEscrowId = new anchor.BN(3);

    // Derive new PDA addresses
    const [newEscrowAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        newEscrowId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    const [newVaultAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), newEscrowAccount.toBuffer(), mintA.toBuffer()],
      program.programId
    );

    // Create new escrow
    await program.methods
      .makeEscrow(newEscrowId, depositAmount, demandAmount)
      .accounts({
        signer: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        tokenProgram: TOKEN_PROGRAM_ID,
       
      })
      .signers([maker])
      .rpc();

    console.log("New escrow created for error test");

    // Try to refund with wrong signer (should fail)
    try {
      await program.methods
        .refundIt()
        .accounts({
          signer: taker.publicKey, // Wrong signer!
          mintA: mintA,
          escrow: newEscrowAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
         
        })
        .signers([taker])
        .rpc();

      assert.fail("Should have failed with wrong signer");
    } catch (error) {
      console.log("✅ Correctly rejected unauthorized refund");
      console.log("Error:", error.message);
    }
  });
});
