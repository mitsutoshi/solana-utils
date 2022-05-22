use super::constants::*;
use solana_sdk::account::Account;

// Return SPL token name.
pub fn get_spl_token_name(mint: &str) -> Option<&str> {
    let spl_token_name = match mint {
        TOKEN_ADDRESS_USDT => Some("USDT"),
        TOKEN_ADDRESS_FIDA => Some("FIDA"),
        TOKEN_ADDRESS_RAY => Some("RAY"),
        _ => None,
    };
    spl_token_name
}

// Extract SOL amount from account.
pub fn extract_sol_balance(account: &Account) -> f64 {
    return (account.lamports as f64) / 10_f64.powi(9);
}
