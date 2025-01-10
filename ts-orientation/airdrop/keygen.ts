import { Keypair } from "@solana/web3.js";
import * as fs from "fs";

// Generate Keypair
let kp = Keypair.generate();

console.log(`You've generated a new Solana Keypair ${kp.publicKey.toBase58()}`);

fs.writeFileSync("dev-wallet.json", `[${kp.secretKey}]`);
