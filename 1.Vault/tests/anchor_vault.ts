import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { assert, expect } from "chai";

describe("anchor_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;
  const provider = anchor.getProvider();

  // Common values
  const depositAmount = new anchor.BN(1_000_000_000); // 1 SOL
  const withdrawAmount = new anchor.BN(500_000_000); // 0.5 SOL
  let initialVaultBalance: number;
  let initialOwnerBalance: number;

  // Get the state PDA
  const [state, stateBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), program.provider.publicKey.toBuffer()],
    program.programId
  );

  // Get the vault PDA
  const [vault, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), state.toBuffer()],
    program.programId
  );

  // Setup before tests
  beforeEach(async () => {
    initialVaultBalance = await provider.connection.getBalance(vault);
    initialOwnerBalance = await provider.connection.getBalance(
      provider.publicKey
    );
  });

  it("Initializes the vault", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        owner: provider.publicKey,
        state: state,
        vault: vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Initialize transaction signature", tx);

    // Verify the state account was created
    const stateAccount = await program.account.vaultState.fetch(state);
    assert.ok(stateAccount, "State account not created");
  });

  it("Successfully deposits into a vault", async () => {
    // Deposit
    const tx = await program.methods.deposit(depositAmount).rpc();
    console.log("Deposit transaction signature", tx);

    // Get final balances
    const finalVaultBalance = await provider.connection.getBalance(vault);
    const finalOwnerBalance = await provider.connection.getBalance(
      provider.publicKey
    );

    // Assert the correct amount was deposited
    assert.equal(
      finalVaultBalance - initialVaultBalance,
      depositAmount.toNumber(),
      "Incorrect deposit amount"
    );

    // Owner balance should decrease by more than deposit (includes tx fee)
    assert.isTrue(
      initialOwnerBalance - finalOwnerBalance >= depositAmount.toNumber(),
      "Owner balance didn't decrease appropriately"
    );
  });

  it("Successfully withdraws from a vault", async () => {
    // Withdraw
    const tx = await program.methods.withdraw(withdrawAmount).rpc();
    console.log("Withdrawal transaction signature", tx);

    // Get final balances
    const finalVaultBalance = await provider.connection.getBalance(vault);
    const finalOwnerBalance = await provider.connection.getBalance(
      provider.publicKey
    );

    // Assert the correct amount was withdrawn
    assert.equal(
      initialVaultBalance - finalVaultBalance,
      withdrawAmount.toNumber(),
      "Incorrect withdrawal amount"
    );

    // Owner balance should increase by slightly less than withdrawal (due to tx fee)
    assert.isTrue(
      finalOwnerBalance - initialOwnerBalance > 0,
      "Owner balance didn't increase"
    );
  });

  it("Fails to withdraw more than vault balance", async () => {
    // Get current vault balance
    const vaultBalance = await provider.connection.getBalance(vault);
    const overdrawnAmount = new anchor.BN(vaultBalance + 1_000_000_000); // vault balance + 1 SOL

    try {
      await program.methods
        .withdraw(overdrawnAmount)
        .accounts({
          owner: provider.publicKey,
          state,
          vault,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      assert.fail("Expected withdrawal to fail");
    } catch (error) {
      // Verify it's the expected error
      assert.include(
        error.toString(),
        "insufficient lamports",
        "Expected insufficient funds error"
      );
    }

    // Verify vault balance hasn't changed
    const finalVaultBalance = await provider.connection.getBalance(vault);
    assert.equal(
      finalVaultBalance,
      vaultBalance,
      "Vault balance should remain unchanged"
    );
  });

  it("Can close out a vault", async () => {
    // Get initial balances
    const initialVaultBalance = await provider.connection.getBalance(vault);
    const initialOwnerBalance = await provider.connection.getBalance(
      provider.publicKey
    );

    // Close the vault
    const tx = await program.methods
      .close()
      .accounts({
        owner: provider.publicKey,
        state,
        vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Close transaction signature", tx);

    // Verify vault is closed (balance should be 0)
    const finalVaultBalance = await provider.connection.getBalance(vault);
    assert.equal(finalVaultBalance, 0, "Vault should have 0 balance");

    // Verify owner received the funds (minus tx fee)
    const finalOwnerBalance = await provider.connection.getBalance(
      provider.publicKey
    );
    assert.isTrue(
      finalOwnerBalance > initialOwnerBalance,
      "Owner should receive vault funds"
    );

    // Verify state account is closed
    try {
      await program.account.vaultState.fetch(state);
      assert.fail("State account should be closed");
    } catch (error) {
      assert.include(
        error.toString(),
        "Account does not exist",
        "Expected account to be closed"
      );
    }
  });
});
