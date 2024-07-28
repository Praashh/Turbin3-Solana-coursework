use bs58;
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::system_program;
use std::io::{self, BufRead};
use std::str::FromStr;
mod programs;
use crate::programs::wba_prereq::{CompleteArgs, WbaPrereqProgram};

#[cfg(test)]
mod tests {
    use solana_sdk;
    #[test]
    fn keygen() {}
    #[test]
    fn airdop() {}
    #[test]
    fn transfer_sol() {}
    #[test]
    fn base58_to_wallet() {}
    #[test]
    fn wallet_to_base58() {}
}
mod convertwallet;

use solana_sdk::{
    message::Message, // For empty devent wallet
    // pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction, // for transfering sol to WBA address
};

// keygen generate
#[test]
fn keygen() {
    // Create a new keypair
    let kp = Keypair::new(); // pubKey-> HyonB2ydDAT13yYhL21h1xL6xRQwVcoUE3De4BARDam7
    println!(
        "You've generated a new Solana wallet: {}",
        kp.pubkey().to_string()
    );
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
}

// wallet converter fns
#[test]
fn base58_to_wallet() {
    println!("Input your private key as base58:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    match bs58::decode(base58).into_vec() {
        Ok(wallet) => {
            println!("Your wallet file is:");
            println!("{:?}", wallet);
        }
        Err(e) => {
            eprintln!("Failed to decode base58: {:?}", e);
        }
    }
}

#[test]
fn wallet_to_base58() {
    println!("Input your private key as a wallet file byte array (e.g., [1, 2, 3, ...]):");
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Remove surrounding brackets and split by comma
    let input = input.trim().trim_start_matches('[').trim_end_matches(']');
    let wallet_result: Result<Vec<u8>, _> =
        input.split(',').map(|s| s.trim().parse::<u8>()).collect();

    match wallet_result {
        Ok(wallet) => {
            let base58 = bs58::encode(wallet).into_string();
            println!("Your private key is:");
            println!("{}", base58);
        }
        Err(e) => {
            eprintln!("Failed to parse wallet input: {:?}", e);
        }
    }
}

// airdrop

#[test]
fn airdopSol() {
    const RPC_URL: &str = "https://api.devnet.solana.com";
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let client = RpcClient::new(RPC_URL);
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
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

//Transfer tokens to your WBA Address

#[test]
fn transfer_sol() {
    const RPC_URL: &str = "https://api.devnet.solana.com";
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let to_pubkey = Pubkey::from_str("H7ur1MPVaSGKsEtvQcrcjpXv1YuwL1u4HJimBnmPQwLS").unwrap();
    let rpc_client = RpcClient::new(RPC_URL);
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!(
        "Success! Check out your TX here:
    https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

#[test]
fn empty_wallet() {
    const RPC_URL: &str = "https://api.devnet.solana.com";
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let rpc_client = RpcClient::new(RPC_URL);
    let to_pubkey = Pubkey::from_str("H7ur1MPVaSGKsEtvQcrcjpXv1YuwL1u4HJimBnmPQwLS").unwrap(); // Define the destination public key
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );
    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");

    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!(
        "Success! Check out your TX here:
https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

#[test]
fn complete_task() {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");
    let signer = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet file");
    let prereq =
        WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
    let args = CompleteArgs {
        github: b"praashh".to_vec(),
    };
    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let transaction = WbaPrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    // Print our transaction out
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
