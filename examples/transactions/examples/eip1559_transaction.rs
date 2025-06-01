// NOTE: This example is for illustrative purposes and assumes a compatible environment.
// Due to limitations in the current test environment (rustc 1.75.0),
// this code cannot be compiled and run here. It should be tested with a more recent
// Rust compiler and compatible dependencies (e.g., those that work with icu4c version >=70).

use ethers::prelude::*;
use ethers_core::types::{Address, Eip1559TransactionRequest, U256};
use std::str::FromStr;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Connect to an Ethereum node
    // Replace with your desired RPC endpoint or use a local Anvil/Ganache instance.
    // Anvil default: "http://localhost:8545"
    let provider_url = "http://localhost:8545";
    let provider = Provider::<Http>::try_from(provider_url)?;

    // 2. Set up a local wallet
    // IMPORTANT: Never hardcode private keys in production code! This is for example purposes only.
    // Consider using environment variables or a secure vault for private keys.
    // This is a sample Anvil private key.
    let private_key_hex = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
    let wallet = private_key_hex.parse::<LocalWallet>()?.with_chain_id(Chain::Dev); // Chain::Dev or specific chain ID

    println!("Using wallet address: {:?}", wallet.address());

    // 3. Define transaction recipient and value
    let to_address = Address::from_str("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")?; // vitalik.eth
    let value = U256::from_dec_str("100000000000000")?; // 0.0001 ETH in wei

    // 4. Create an EIP-1559 transaction request
    // For EIP-1559, you need to set max_fee_per_gas and max_priority_fee_per_gas.
    // Gas limit can often be estimated by the node, but can also be set explicitly.

    // Get current EIP-1559 fee data
    let (current_max_fee_per_gas, current_max_priority_fee_per_gas) = provider.estimate_eip1559_fees(None).await?;
    println!("Estimated max_fee_per_gas: {} wei", current_max_fee_per_gas);
    println!("Estimated max_priority_fee_per_gas: {} wei", current_max_priority_fee_per_gas);

    // You can use the estimates or set your own values.
    // For this example, we'll use the estimates plus a small buffer for priority_fee if desired.
    let max_fee_per_gas = current_max_fee_per_gas;
    let max_priority_fee_per_gas = current_max_priority_fee_per_gas; // Or a custom value like U256::from(2_000_000_000) for 2 Gwei

    let mut tx = Eip1559TransactionRequest::new()
        .to(to_address)
        .value(value)
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(max_priority_fee_per_gas);

    // Set gas limit (optional, can be estimated by the provider if not set)
    // tx.gas = Some(U256::from(21000)); // Example: 21000 for a simple ETH transfer

    // The provider can also fill other essential fields like nonce and gas limit (if not set)
    provider.fill_transaction(&mut tx.clone().into(), None).await?;
    println!("Transaction request (after filling): {:?}", tx);

    // 5. Sign the transaction
    // The `sign_transaction` method on the wallet requires the transaction to be typed.
    // We need to convert `Eip1559TransactionRequest` into `TypedTransaction::Eip1559`.
    let typed_tx = TypedTransaction::Eip1559(tx);
    let signature = wallet.sign_transaction(&typed_tx).await?;

    // 6. Send the transaction
    // `send_raw_transaction` takes the RLP encoded signed transaction.
    let rlp_signed_tx = typed_tx.rlp_signed(&signature);
    let pending_tx = provider.send_raw_transaction(rlp_signed_tx).await?;

    println!("Transaction sent, pending tx hash: {:?}", pending_tx.tx_hash());

    // 7. Wait for the transaction receipt
    // `confirmations(1)` means wait for at least 1 block confirmation.
    let receipt = pending_tx.confirmations(1).await?.ok_or_else(|| eyre::eyre!("Transaction receipt not found after 1 confirmation"))?;

    println!("Transaction mined!");
    println!("  Transaction Hash: {:?}", receipt.transaction_hash);
    println!("  Block Number: {:?}", receipt.block_number.unwrap_or_default());
    println!("  Gas Used: {:?}", receipt.gas_used.unwrap_or_default());
    println!("  Effective Gas Price: {:?}", receipt.effective_gas_price.unwrap_or_default());

    Ok(())
}
