use bs58;
use solana_client::rpc_client::RpcClient;
use solana_program::system_program;
use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, read_keypair_file},
    system_instruction::transfer,
    transaction::Transaction,
};
use std::io::{self, BufRead};
use std::str::FromStr;

mod programs;
use programs::wba_prereq::{WbaPrereqProgram, CompleteArgs};


const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)]
mod tests {
    use solana_sdk::signer::Signer;

    use super::*;

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',').map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn claim_airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(signature) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", signature);
            },
            Err(err) => println!("Oops, something went wrong: {}", err),
        }
    }

    #[test]
    fn transfer_to_wba() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("<your WBA public key>").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash
        );

        let signature = rpc_client.send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
       
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }

    #[test]
    fn empty_devnet_wallet() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("<your WBA public key>").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");
        let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash
        );

        let signature = rpc_client.send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
       
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }

    #[test]
    fn submit_completion() {
        let signer = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet file");
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let prereq = WbaPrereqProgram::derive_program_address(
            &[b"prereq", signer.pubkey().to_bytes().as_ref()]
        );

        let args = CompleteArgs {
            github: b"testaccount".to_vec(),
        };

        let transaction = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash
        );

        let signature = rpc_client.send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }
}
