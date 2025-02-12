import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { assert } from "chai";
import {
  confirmTransaction,
  createAccountsMintsAndTokenAccounts,
  makeKeypairs,
} from "@solana-developers/helpers";
import {
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

const programId = new anchor.web3.PublicKey(
  "2RwTZzWToZemV8dzeCDZ7QYwHNniPGadY2ej3penF4vB"
);

describe("escrow", () => {
  // Configure the client to use the local cluster.
  let program: Program<Escrow>;
  let provider: anchor.AnchorProvider;

  provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const connection = provider.connection;
  const signer = provider.wallet as anchor.Wallet;

  program = anchor.workspace.Escrow as Program<Escrow>;

  const [alice, bob] = makeKeypairs(2);

  let maker, taker, mintAAddress, mintBAddress;
  let takerAtaA, takerAtaB, makerAtaA, makerAtaB;

  let amount = new anchor.BN(1_000_000);

  before(
    "Creates Alice and Bob accounts, 2 token mints, and associated token accounts for both tokens for both users",
    async () => {
      //airdrop some SOL to both alice and bob
      let tx1 = await provider.connection.requestAirdrop(
        alice.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      );

      let tx2 = await provider.connection.requestAirdrop(
        bob.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      );

      let result1 = await confirmTransaction(connection, tx1, "confirmed");
      let result2 = await confirmTransaction(connection, tx2, "confirmed");

      // Set accounts in dapp terms
      maker = alice;
      taker = bob;

      //create token mints
      mintAAddress = await createMint(
        connection, // connection
        maker, // fee payer
        maker.publicKey, // mint authority
        null, // disable freeze auth
        6 // decimals
      );

      mintBAddress = await createMint(
        connection, // connection
        maker, // fee payer
        maker.publicKey, // mint authority
        null, // disable freeze auth
        6 // decimals
      );

      // create associated token accounts for both alice and bob
      makerAtaA = await getOrCreateAssociatedTokenAccount(
        connection, // connection
        maker, // fee payer
        mintAAddress, // mint
        maker.publicKey // owner,
      );
      console.log(`maker ATA (A) PK: ${makerAtaA.address}`);

      // create associated token accounts for both alice and bob
      makerAtaB = await getOrCreateAssociatedTokenAccount(
        connection, // connection
        maker, // fee payer
        mintBAddress, // mint
        maker.publicKey // owner,
      );
      console.log(`maker ATA (B) PK: ${makerAtaB.address}`);

      takerAtaA = await getOrCreateAssociatedTokenAccount(
        connection, // connection
        maker, // fee payer
        mintAAddress, // mint
        taker.publicKey // owner,
      );
      console.log(`taker ATA (A) PK: ${makerAtaA.address}`);

      takerAtaB = await getOrCreateAssociatedTokenAccount(
        connection, // connection
        maker, // fee payer
        mintBAddress, // mint
        taker.publicKey // owner,
      );
      console.log(`maker ATA (B) PK: ${makerAtaA.address}`);

      // mint tokens to both alice and bob
      await mintTo(
        connection, // connection
        maker, // fee payer
        mintAAddress, // mint
        makerAtaA.address, // receiver (should be a token account)
        maker, // mint authority
        10000 * 10 ** 6 // amount. if your decimals is 8, you mint 10^8 for 1 token.
      );
      console.log(`maker ATA (A) Amt: `, makerAtaA.amount.valueOf());

      await mintTo(
        connection, // connection
        taker, // fee payer
        mintBAddress, // mint
        takerAtaB.address, // receiver (should be a token account)
        maker, // mint authority
        10000 * 10 ** 6 // amount. if your decimals is 8, you mint 10^8 for 1 token.
      );
      console.log(`taker ATA (B) Amt: `, takerAtaB.amount.valueOf());
    }
  );

  it("initializes an escrow", async () => {
    // determine escrow address
    const id = new anchor.BN(0);

    let escrow = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        alice.publicKey.toBuffer(),
        id.toArrayLike(Buffer, "le", 8),
      ],
      programId
    )[0];

    try {
      const makeIx = await program.methods
        .make(id, amount)
        .accountsPartial({
          maker: maker.publicKey,
          mintA: mintAAddress,
          mintB: mintBAddress,
          makerAtaA: makerAtaA.address,
          escrow: escrow,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .instruction();

      let result = await submitAndGetSignature(maker, makeIx, connection);
      console.log(`Signature: ${result}`);
    } catch (error) {
      console.error("Transaction Error: ", await error);
      assert(false);
    }
  });

  it("rejects duplicate escrows", async () => {
    // determine escrow address
    const id = new anchor.BN(0);

    let escrow = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        alice.publicKey.toBuffer(),
        id.toArrayLike(Buffer, "le", 8),
      ],
      programId
    )[0];

    try {
      const makeIx = await program.methods
        .make(id, amount)
        .accountsPartial({
          maker: maker.publicKey,
          mintA: mintAAddress,
          mintB: mintBAddress,
          makerAtaA: makerAtaA.address,
          // escrow: escrow,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .instruction();

      let result = await submitAndGetSignature(maker, makeIx, connection);
      console.log(`Signature: ${result}`);

      assert.fail("Expected transaction to fail");
    } catch (error) {
      assert.include(
        error.toString(),
        "already in use",
        "Expected error about account already in use"
      );
    }
  });

  it("accepts new escrows and deposits tokens into escrow", async () => {
    // determine escrow address
    const id = new anchor.BN(1);

    let escrow = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        alice.publicKey.toBuffer(),
        id.toArrayLike(Buffer, "le", 8),
      ],
      programId
    )[0];

    try {
      const makeIx = await program.methods
        .make(id, amount)
        .accountsPartial({
          maker: maker.publicKey,
          mintA: mintAAddress,
          mintB: mintBAddress,
          makerAtaA: makerAtaA.address,
          // escrow: escrow,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .instruction();

      let result = await submitAndGetSignature(maker, makeIx, connection);

      // get the escrow account
      const escrowAccount = await program.account.escrow.fetch(escrow);
      console.log(`Escrow: ${escrowAccount}`);

      // Find the vault PDA (token account)
      const vault = getAssociatedTokenAddressSync(
        mintAAddress,
        escrow,
        true // allowOwnerOffCurve = true since escrow is a PDA
      );

      // Get vault balance
      const vaultBalance = await connection.getTokenAccountBalance(vault);

      // Assert the correct amount was deposited
      assert.equal(
        vaultBalance.value.amount,
        amount.toString(),
        "Incorrect amount deposited to vault"
      );

      console.log(`Signature: ${result}`);
    } catch (error) {
      console.error("Transaction Error: ", await error);
      assert(false);
    }
  });
});

// submit a transaction and return the signature
async function submitAndGetSignature(
  payer: anchor.web3.Signer,
  txIx: anchor.web3.TransactionInstruction,
  connection: anchor.web3.Connection
) {
  console.log(`Sending txn for ${payer.publicKey}...`);
  let bh = await connection.getLatestBlockhash();

  const tx = new anchor.web3.Transaction({
    feePayer: payer.publicKey,
    blockhash: bh.blockhash,
    lastValidBlockHeight: bh.lastValidBlockHeight,
  }).add(txIx);

  return await anchor.web3.sendAndConfirmTransaction(connection, tx, [payer]);
}
