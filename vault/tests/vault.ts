import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";

describe("vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.vault as Program<Vault>;
  const newacc = anchor.web3.Keypair.generate();


  const acc = anchor.getProvider();
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.init().accounts({
      signer: acc.publicKey  
    }
    ).signers([
      acc.wallet.payer
    ])
    .rpc();



  
    console.log(await program.account.vaultState.all())
    console.log("Your transaction signature", tx);
  });

  it("Can deposit!", async () => {
    // Minimum rent exemption for a 0-byte account is ~890,880 lamports
    // Let's deposit 1 SOL (1,000,000,000 lamports) to be safe
    const depositAmount = new anchor.BN(1000000000); 
    console.log(acc.wallet.publicKey)
    
    const tx = await program.methods.deposit(depositAmount)
    .accounts({
      signer: acc.wallet.publicKey,
    }).signers([acc.wallet.payer])
    .rpc()
    
      
    console.log("Deposit transaction signature", tx);
  })

  it("Can withdraw!", async () => {
    // Assung you have a function to withdraw in your program
    const withdrawAmount = new anchor.BN(100000000); // Example amount
    const tx = await program.methods.withdraw(withdrawAmount).accounts({
      signer:acc.publicKey
    }).signers([acc.wallet.payer])
    .rpc();
    console.log("Withdraw transaction signature", tx);
  });


  it("Can close vault!", async () => {
    // Assuming you have a function to close the vault in your program
    const tx = await program.methods.close().rpc();
    console.log("Close vault transaction signature", tx);
  });
});
