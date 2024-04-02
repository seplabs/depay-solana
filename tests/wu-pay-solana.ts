import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WuPaySolana } from "../target/types/wu_pay_solana";
import { BN } from "@coral-xyz/anchor";
import assert from "assert";

interface PDAParameters {
  //from: anchor.web3.PublicKey,
  //to: anchor.web3.PublicKey,
  statekey: anchor.web3.PublicKey,
  walletkey: anchor.web3.PublicKey,
  idx: anchor.BN,
  statebump: number,
  walletbump: number,
}

describe("wu-pay-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider()
  const program = anchor.workspace.WuPaySolana as Program<WuPaySolana>;

  // create test accounts
  let alice: anchor.web3.Keypair;
  let bob: anchor.web3.Keypair;
  let pda: PDAParameters;

  beforeEach(async () => {
    alice = await createUserAndAirdrop(5, provider);
    bob = await createUserAndAirdrop(1, provider);
    pda = await createPdaParams(provider.connection, alice.publicKey, bob.publicKey);
  });

  it("Case1: Normal case, alice trasfer 20 to bob via safetybox", async () => {
    console.log(`Step1: Initialized a new Safe Pay instance. Alice sent 20 tokens to saftybox`);
    const aliceBalancePre = await readAccount(alice.publicKey, provider);
    assert.equal(aliceBalancePre, '5000000000');

    const amount = new anchor.BN(20000000);

    // Step1: Alice sent 20 tokens to saftybox
    const tx1 = await program.methods.depositeGrant(pda.idx, pda.statebump, pda.walletbump,amount).accounts({
      sender: alice.publicKey,
      receiver: bob.publicKey,
      escrowState: pda.statekey,
      escrowWallet: pda.walletkey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([alice]).rpc(
      {skipPreflight:true}
    );

    // Assert that 20 tokens were moved from Alice's account to the escrow.
    const aliceBalancePost = await readAccount(alice.publicKey, provider);
    console.log("Alice's balance: {}", aliceBalancePost);
    assert.equal(aliceBalancePost, '4978413120');
    const saftyboxBalancePost = await readAccount(pda.walletkey, provider);
    console.log("Safetybox's balance: {}", saftyboxBalancePost);
    assert.equal(saftyboxBalancePost, '21586880');

    //Step2: Complete the grant. Bob received 20 tokens from saftybox
    console.log(`Step2: Complete the grant. Bob received 20 tokens from saftybox`);
    try{
 
    const tx2 = await program.methods.completeGrant(pda.idx, pda.statebump, pda.walletbump,amount).accounts({
      sender: alice.publicKey,
      receiver: bob.publicKey,
      escrowState: pda.statekey,
      escrowWallet: pda.walletkey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([alice]).rpc(
      {skipPreflight:true}
    );
  }catch(error){
    console.error('Error message:', error.message);
    console.error('Stack trace:', error.stack);
  }

    // Assert that 20 tokens were sent to bob.
    const bobBalance = await readAccount(bob.publicKey, provider);
    console.log("Bob's balance: {}", bobBalance);
    const saftyboxBalancePost2 = await readAccount(pda.walletkey, provider);
    console.log("Safetybox's balance: {}", saftyboxBalancePost2);
    assert.equal(bobBalance, '20000000');
    console.log(`Step3: Clean up. Close the saftybox account.`);

    /*
    const tx3 = await program.methods.closePda(pda.idx, pda.bump).accounts({
      fromPayer: alice.publicKey,
      toPayee: bob.publicKey,
      saftyboxPda: pda.saftyboxKey,
      //systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([alice]).rpc();


    // // Assert that saftybox was correctly closed.
    try {
      await readAccount(pda.saftyboxKey, provider);
      return assert.fail("Account should be closed");
    } catch (e) {
      assert.equal(e.message, "Cannot read properties of null (reading 'data')");
    }

*/
  });

  const createUserAndAirdrop = async (airdropAmount: number, provider: anchor.Provider): Promise<anchor.web3.Keypair> => {
    const user = anchor.web3.Keypair.generate();
    console.log(`- Creating a new user and airdropping ${airdropAmount} SOL`);
    console.log(`Pubkey for new user: ${user.publicKey.toBase58()}`);
    if (airdropAmount > 0) {
      const lamports = airdropAmount * anchor.web3.LAMPORTS_PER_SOL;
      const signature = await provider.connection.requestAirdrop(user.publicKey, lamports);
      // Poll for the transaction confirmation
      let confirmed = false;
      for (let i = 0; i < 10; i++) { // Try 10 times
        await new Promise(resolve => setTimeout(resolve, 2000)); // Wait 2 seconds before each new attempt
        const response = await provider.connection.getSignatureStatuses([signature]);
        const status = response && response.value[0];
        if (status && status.confirmationStatus === 'finalized') {
          confirmed = true;
          console.log('Transaction confirmed');
          break;
        }
      }

      if (!confirmed) {
        console.error('Transaction failed or was not confirmed');
      }
      console.log(`Airdrop done: ${signature}`);
      let balance = await provider.connection.getBalance(user.publicKey);
      console.log(`Balance: ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
    }
    return user;
  }

  const readAccount = async (accountPublicKey: anchor.web3.PublicKey, provider: anchor.Provider): Promise<number> => {
    return await provider.connection.getBalance(accountPublicKey);
  }

  const createPdaParams = async (connection: anchor.web3.Connection, alice: anchor.web3.PublicKey, bob: anchor.web3.PublicKey): Promise<PDAParameters> => {
    const uid = new anchor.BN(parseInt((Date.now() / 1000).toString()));
    const uidBuffer = uid.toBuffer('le', 8);

    let [statePubKey, statebump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow_state"), alice.toBuffer(), bob.toBuffer(), uidBuffer], program.programId,
    );

    let [walletkey, walletbump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow_wallet"), alice.toBuffer(), bob.toBuffer(), uidBuffer], program.programId,
    );
      console.log(`- PDA Wallet created: ${walletkey.toBase58()}`);
      console.log(`- PDA state created: ${statePubKey.toBase58()}`);
      console.log('idx is {}',uid);
      console.log('walletbump is {}',walletbump);
      console.log('statebump is {}',statebump);
    return {
      statekey: statePubKey,
      walletkey: walletkey,
      idx: uid,
      statebump: statebump,
      walletbump: walletbump,
    }
  }

});