# Jup CLI

A simple command-line interface (CLI) to interact with the Jupiter Swap API for performing token swaps on Solana.

## Features

*   Swap tokens using Jupiter's routing.
*   Specify input mint, output mint, and amount.
*   Use your own keypair for signing transactions.
*   Configure the RPC endpoint.

## Prerequisites

*   Rust programming language and Cargo (Rust's package manager). Install from [rustup.rs](https://rustup.rs/).
*   A Solana keypair file (usually a JSON file). You can generate one using the [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools).

## Building

1.  Clone the repository (if you haven't already):
    ```bash
    # If you have the code locally, skip this step
    git clone <repository_url>
    cd jup-cli
    ```
2.  Build the project:
    ```bash
    cargo build
    ```
    The executable will be located at `target/debug/jup-cli`. For a release build, use `cargo build --release` (executable at `target/release/jup-cli`).

## Usage

The primary command is `swap`.

```bash
./target/debug/jup-cli swap \
  --from <INPUT_MINT_ADDRESS> \
  --to <OUTPUT_MINT_ADDRESS> \
  --amount <AMOUNT_IN_SMALLEST_UNITS> \
  --keypair-path /path/to/your/solana/keypair.json \
  --rpc-url <RPC_NODE_URL>
```

### Arguments:

*   `--from <INPUT_MINT_ADDRESS>`: (Required) The mint address of the token you want to swap from (e.g., `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v` for USDC).
*   `--to <OUTPUT_MINT_ADDRESS>`: (Required) The mint address of the token you want to swap to (e.g., `So11111111111111111111111111111111111111112` for SOL).
*   `--amount <AMOUNT_IN_SMALLEST_UNITS>`: (Required) The amount of the input token to swap, specified in its smallest unit (e.g., lamports for SOL, or the equivalent for other tokens based on their decimals). The CLI currently does not handle decimal conversion.
*   `--keypair-path <PATH_TO_KEYPAIR>`: (Required) The file path to your Solana signer keypair (JSON format).
*   `--rpc-url <RPC_NODE_URL>`: (Optional) The RPC URL for the Solana cluster.
    *   Defaults to: `https://api.mainnet-beta.solana.com`
    *   Example for Devnet: `https://api.devnet.solana.com`

### Example:

Swap 0.1 SOL (100,000,000 lamports if SOL has 9 decimals) for USDC on Mainnet:

```bash
./target/debug/jup-cli swap \
  --from So11111111111111111111111111111111111111112 \
  --to EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --amount 100000000 \
  --keypair-path ~/.config/solana/id.json \
  --rpc-url https://api.mainnet-beta.solana.com
```

**Note on Token Decimals:** This tool expects the `--amount` to be in the smallest unit of the input token. You'll need to manually account for token decimals (e.g., if USDC has 6 decimals, an amount of `1000000` means 1 USDC).

## Environment Variables

*   `API_BASE_URL`: (Optional) Overrides the default Jupiter API base URL (`https://quote-api.jup.ag/v6`).

## Disclaimer

This is a simple tool for demonstration and personal use. Transactions on the Solana blockchain are irreversible. Always double-check mint addresses, amounts, and ensure you understand the risks before executing swaps with real funds. Use a dedicated burner wallet for testing if unsure. 