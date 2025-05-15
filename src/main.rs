use std::env;
use std::str::FromStr; // Required for Pubkey::from_str

use clap::Parser; // Added for clap
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer; // Removed Keypair, Signer is used by signer.pubkey()
use solana_sdk::transaction::VersionedTransaction;
// Removed NullSigner as we will use a real keypair
// use solana_sdk::{pubkey::Pubkey, signature::NullSigner};

// Removed hardcoded TEST_WALLET as it will come from the keypair
// pub const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Performs a token swap
    Swap {
        /// Mint address of the token to swap from
        #[clap(long)]
        from: String,
        /// Mint address of the token to swap to
        #[clap(long)]
        to: String,
        /// Amount of the token to swap (in smallest unit, e.g., lamports)
        #[clap(long)]
        amount: u64,
        /// Path to the signer keypair file
        #[clap(long)]
        keypair_path: String,
        /// RPC URL for the Solana cluster
        #[clap(long)]
        rpc_url: Option<String>,
    },
}

const DEFAULT_RPC_URL: &str = "https://api.mainnet-beta.solana.com";

#[tokio::main]
async fn main() -> anyhow::Result<()> { // Added anyhow::Result for error handling
    let cli = Cli::parse();

    let api_base_url = env::var("API_BASE_URL").unwrap_or("https://quote-api.jup.ag/v6".into());
    println!("Using base url: {}", api_base_url);

    let jupiter_swap_api_client = JupiterSwapApiClient::new(api_base_url);

    match cli.command {
        Commands::Swap {
            from,
            to,
            amount,
            keypair_path,
            rpc_url,
        } => {
            let parse_mint = |m: &str| -> anyhow::Result<Pubkey> {
                if m == "SOL" {
                    Pubkey::from_str("So11111111111111111111111111111111111111112")
                } else {
                    Pubkey::from_str(m)
                }.map_err(|e| anyhow::anyhow!("Map parsing error"))
            };
            let input_mint = parse_mint(&from)?;
            let output_mint = parse_mint(&to)?;

            let signer = solana_sdk::signer::keypair::read_keypair_file(&keypair_path).map_err(|e|
                anyhow::anyhow!(format!("Failed to read keypair from file '{}': {}", keypair_path, e))
            )?;
            let user_public_key = signer.pubkey();

            println!(
                "Attempting to swap {} of {} to {} using wallet {}",
                amount, input_mint, output_mint, user_public_key
            );

            let quote_request = QuoteRequest {
                amount, // Use amount from CLI
                input_mint, // Use input_mint from CLI
                output_mint, // Use output_mint from CLI
                slippage_bps: 50,
                ..QuoteRequest::default()
            };

            // GET /quote
            println!("Fetching quote...");
            let quote_response = jupiter_swap_api_client.quote(&quote_request).await?;
            println!("Quote Response: {quote_response:#?}");

            // POST /swap
            println!("Requesting swap transaction...");
            let swap_response = jupiter_swap_api_client
                .swap(
                    &SwapRequest {
                        user_public_key, // Use pubkey from the loaded keypair
                        quote_response: quote_response.clone(),
                        config: TransactionConfig::default(),
                    },
                    None,
                )
                .await?;

            println!(
                "Swap transaction received (length: {} bytes)",
                swap_response.swap_transaction.len()
            );
            println!("Last valid block height: {}", swap_response.last_valid_block_height);

            let mut versioned_transaction: VersionedTransaction =
                bincode::deserialize(&swap_response.swap_transaction)?;

            println!("Transaction deserialized successfully.");

            println!("Signing transaction with keypair from {}", keypair_path);
            // Manual signing: Serialize the message and sign it with the keypair
            let message_data = versioned_transaction.message.serialize();
            let signature = signer.sign_message(&message_data);
            versioned_transaction.signatures = vec![signature];

            println!("Transaction signed successfully.");

            let rpc_node_url = rpc_url.unwrap_or_else(|| DEFAULT_RPC_URL.to_string());
            println!("Using RPC URL: {}", rpc_node_url);
            let rpc_client = RpcClient::new(rpc_node_url);
            
            println!("Attempting to send transaction to the cluster...");

            match rpc_client
                .send_and_confirm_transaction(&versioned_transaction) // Use the signed transaction
                .await
            {
                Ok(signature) => {
                    println!("Transaction sent successfully! Signature: {}", signature);
                }
                Err(e) => {
                    println!("Transaction failed: {e}");
                }
            }
        }
    }
    Ok(())
}
