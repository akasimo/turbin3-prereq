mod programs;

#[cfg(test)]
mod tests {
    // Import necessary modules and structs
    use solana_program::system_instruction::transfer;
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
        transaction::Transaction,
    };
    use solana_sdk::message::Message;
    use crate::programs::wba_prereq::{CompleteArgs, WbaPrereqProgram};
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use std::io::{self, BufRead};
    use std::str::FromStr;

    // Constants for RPC URL, wallet file paths, and wallet address
    const RPC_URL: &str = "https://api.devnet.solana.com";
    const TEMP_WALLET_FILE: &str = "test-wallet.json";
    const WBA_WALLET_FILE: &str = "wba-wallet.json";
    const WBA_WALLET_ADDRESS: &str = "HJJGipfRiFz6S3g4RFfkZHLUyyLULb7QiH4Anwisd4qi";

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
    }

    #[test]
    fn airdrop() {
        // Read keypair from file
        let keypair = read_keypair_file(TEMP_WALLET_FILE).expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        
        // Request airdrop of 2 SOL
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! check tx here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        // Read keypair from file and parse recipient's public key
        let keypair = read_keypair_file(TEMP_WALLET_FILE).expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str(WBA_WALLET_ADDRESS).unwrap();

        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("failed to get recent blockhash");

        // Create and sign transaction to transfer 0.001 SOL
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send and confirm transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("failed to send transaction");
        println!(
            "Success! Check out your TX here:
        https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn transfer_all() {
        // Read keypair from file and parse recipient's public key
        let keypair = read_keypair_file(TEMP_WALLET_FILE).expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str(WBA_WALLET_ADDRESS).unwrap();

        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("failed to get recent blockhash");

        // Get the balance of the sender's account
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("failed to get balance");

        // Create a message to transfer all funds (balance - fee) that ll be used to calculate fee
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        // Calculate the fee for the transaction
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("failed to get fee");

        // Create and sign transaction to transfer all funds (balance - fee)
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send and confirm transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("failed to send transaction");
        println!(
            "Success! Check out your TX here:
        https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        // Read the base58 private key from user input
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        // Decode the base58 private key into a byte array
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        // Read the wallet file byte array from user input
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your private key is:");
        // Encode the wallet file byte array into a base58 string
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        // Read the signer's keypair from file
        let signer = read_keypair_file(WBA_WALLET_FILE).expect("Couldn't find wallet file");

        // Derive the program address for the WbaPrereqProgram
        let prereq = WbaPrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        
        // Put github username as an argument
        let args = CompleteArgs {
            github: b"akasimo".to_vec(),
        };

        // Get the latest blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create the transaction for the WbaPrereqProgram::complete instruction
        let transaction = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        // Send and confirm the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
