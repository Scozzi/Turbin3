mod programs;

#[cfg(test)]
mod tests {
    use serde_json;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer, system_program};
    use solana_sdk::{
        message::Message,
        native_token::LAMPORTS_PER_SOL,
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    };

    use std::fs::File;
    use std::io::Write;
    use std::str::FromStr;

    use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram, UpdateArgs};

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");

        println!("{:?}", kp.to_bytes());

        let private_key = kp.to_bytes();

        // Convert the private key array to a JSON string
        let json_string = serde_json::to_string(&private_key[..]).unwrap();

        // Write the JSON string to a file
        let mut file = File::create("./dev-wallet.json").unwrap();
        file.write_all(json_string.as_bytes()).unwrap();

        println!("Private key written to private_key.json");
    }
    #[test]
    fn airdop() {
        // Import our keypair
        let kp = read_keypair_file("./dev-wallet.json").expect("Couldn't find wallet file");
        println!("{}", kp.pubkey().to_string());

        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&kp.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");

                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }

            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }
    #[test]
    fn transfer_sol() {
        // Import our keypair
        let kp = read_keypair_file("./dev-wallet.json").expect("Couldn't find wallet file");

        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("HdkM49jvJCav4UufhPSVJG8Gqu4EPpVBQWHZvZbsNDsi").unwrap();

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&kp.pubkey(), &to_pubkey, 100_000_000)],
            Some(&kp.pubkey()),
            &vec![&kp],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn empty() {
        // Import our keypair
        let kp = read_keypair_file("./dev-wallet.json").expect("Couldn't find wallet file");
        println!("From Address : {}", kp.pubkey().to_string());

        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("HdkM49jvJCav4UufhPSVJG8Gqu4EPpVBQWHZvZbsNDsi").unwrap();
        println!("To Address : {}", to_pubkey.to_string());

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get balance of dev wallet
        let balance = rpc_client
            .get_balance(&kp.pubkey())
            .expect("Failed to get balance");

        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(&kp.pubkey(), &to_pubkey, balance)],
            Some(&kp.pubkey()),
            &recent_blockhash,
        );

        let estimate = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        println!(
            "Estimated Fee: {}",
            (estimate as f64) / (LAMPORTS_PER_SOL as f64)
        );

        // Deduct fee from lamports amount and create a TX with correct balance
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&kp.pubkey(), &to_pubkey, balance - estimate)],
            Some(&kp.pubkey()),
            &vec![&kp],
            recent_blockhash,
        );
        println!("{:?}", transaction);

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn signup() {
        let rpc_client = RpcClient::new(RPC_URL);

        // Let's define our accounts
        let signer = read_keypair_file("../id.json").expect("Couldn't find wallet file");
        println!("Signer : {}", signer.pubkey().to_string());

        let prereq = Turbin3PrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        println!("{}", prereq);

        // Define our instruction data
        let args = CompleteArgs {
            github: b"Scozzi".to_vec(),
        };

        // Get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Now we can invoke the "complete" function
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
