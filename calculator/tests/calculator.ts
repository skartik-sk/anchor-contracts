import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import { assert } from "chai";

describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.calculator as Program<Calculator>;

  const newacc = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.init(20)
    .accounts({
      account:newacc.publicKey,
      signer:anchor.getProvider().wallet.publicKey,
    })
    .signers([newacc])
    
    .rpc();
    console.log("Your transaction signature", tx);
  });


  it("Is add!", async () => {
    // Add your test here.
    const tx = await program.methods.add(20)
    .accounts({
      account:newacc.publicKey,
      signer:anchor.getProvider().wallet.publicKey,
    })    
    .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.data.fetch(newacc.publicKey);

    assert.equal(account.num, 40, "Value should be 40 after addition");
  });
 
  it("Is subtract!", async () => {
    // Add your test here.
    const tx = await program.methods.subtract(10)
    .accounts({
      account:newacc.publicKey,
      signer:anchor.getProvider().wallet.publicKey,
    })    
    .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.data.fetch(newacc.publicKey);

    assert.equal(account.num, 30, "Value should be 30 after subtraction");
  });


  it("Is double!", async () => {
    // Add your test here.
    const tx = await program.methods.double()
    .accounts({
      account:newacc.publicKey,
      signer:anchor.getProvider().wallet.publicKey,
    })    
    .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.data.fetch(newacc.publicKey);

    assert.equal(account.num, 60, "Value should be 60 after multiplication");
  });




});
