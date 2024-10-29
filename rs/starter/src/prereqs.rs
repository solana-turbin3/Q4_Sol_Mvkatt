use bs58;
fn convert_base58_to_wallet(base58: &str) -> Vec<u8> {
    println!("Your wallet file is:");
    let result = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", result);

    result
}

fn convert_wallet_to_base(wallet: Vec<u8>) -> String {
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);

    base58
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::programs::wba_prereq::{ CompleteArgs, WbaPrereqProgram };
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer, system_program};
    use solana_sdk::{
        message::Message,
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    };
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    pub fn keygen() -> () {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!();
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        let keypair =
            read_keypair_file("../wallets/dev-wallet.json").expect("Couldn't find wallet file");

        println!("Your wallet: {:?}", keypair.to_bytes());

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

    #[test]
    fn transfer_sol() {
        let keypair =
            read_keypair_file("../wallets/dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("EaHopsEgJ7JnjS7anmTBMi8ArVyrPt8h59AerLa1ZVdp").unwrap();
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
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }

    #[test]
    fn empty_wallet() {
        let keypair =
            read_keypair_file("../wallets/dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("EaHopsEgJ7JnjS7anmTBMi8ArVyrPt8h59AerLa1ZVdp").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
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
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }

    #[test]
    fn check_balance() {
        let to_pubkey = Pubkey::from_str("EaHopsEgJ7JnjS7anmTBMi8ArVyrPt8h59AerLa1ZVdp").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let balance = rpc_client
            .get_balance(&to_pubkey)
            .expect("Failed to get balance");

        println!("balance is {}", balance);
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer =
            read_keypair_file("../wallets/Turbin3-wallet.json").expect("Couldn't find wallet file");
        let prereq = WbaPrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        let args = CompleteArgs {
            github: b"katemv".to_vec(),
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

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn base58_to_wallet() {
        let wallet = convert_base58_to_wallet(
            "gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7VvksUGhPt4SZ8JHVSkt"
        );

        assert_eq!(
            wallet,
            [
                34, 46, 55, 124, 141, 190, 24, 204, 134, 91, 70, 184, 161, 181, 44, 122, 15, 172,
                63, 62, 153, 150, 99, 255, 202, 89, 105, 77, 41, 89, 253, 130, 27, 195, 134, 14,
                66, 75, 242, 7, 132, 234, 160, 203, 109, 195, 116, 251, 144, 44, 28, 56, 231, 114,
                50, 131, 185, 168, 138, 61, 35, 98, 78, 53
            ]
        );
    }

    #[test]
    fn wallet_to_base() {
        let wallet: Vec<u8> = vec![
            34, 46, 55, 124, 141, 190, 24, 204, 134, 91, 70, 184, 161, 181, 44, 122, 15, 172, 63,
            62, 153, 150, 99, 255, 202, 89, 105, 77, 41, 89, 253, 130, 27, 195, 134, 14, 66, 75,
            242, 7, 132, 234, 160, 203, 109, 195, 116, 251, 144, 44, 28, 56, 231, 114, 50, 131,
            185, 168, 138, 61, 35, 98, 78, 53,
        ];
        let base58 = convert_wallet_to_base(wallet);
        assert_eq!(
            base58,
            "gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7VvksUGhPt4SZ8JHVSkt"
        );
    }
}
