import bs58 from "bs58";
import * as prompt from "prompt-sync";
import { Keypair } from "@solana/web3.js";
import * as fs from "fs";

function getWalletFromFile() {
  const kp = Keypair.fromSecretKey(
    Uint8Array.from(JSON.parse(fs.readFileSync("dev-wallet.json", "utf8")))
  );
  console.log(kp.publicKey.toBase58());
  console.log(`[${kp.secretKey}]`);
}

function base58ToWallet(base58String: string): Uint8Array {
  return bs58.decode(base58String);
}

function walletToBase58(wallet: Uint8Array): string {
  return bs58.encode(wallet);
}

// Example usage:
const base58String = "";
const wallet = base58ToWallet(base58String);
console.log(wallet);

const originalBase58String = walletToBase58(wallet);
console.log(originalBase58String);

getWalletFromFile();
