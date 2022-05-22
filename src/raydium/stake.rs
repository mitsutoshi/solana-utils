use serde::{Deserialize, Serialize};
use solana_account_decoder::UiAccountEncoding;
use solana_client::client_error::Result as ClientResult;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, MemcmpEncoding, RpcFilterType};
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub const STAKE_PROGRAM: &str = "EhhTKczWMGQt46ynNeRX1WfeagwwJd7ufHvCDjRxjo5Q";

#[allow(dead_code)]
pub const STAKE_PROGRAM_V4: &str = "CBuCnLe26faBpcBP2fktp4rp8abpcAnTWft6ZrP5Q4T";

pub const STAKE_PROGRAM_V5: &str = "9KEPoZmtHUrBbhWN1v1KWLMkkvwY6WLtAVUCPRtRjP4z";

pub const POOL_ID_SOLUSDT: &str = "5r878BSWPtoXgnqaeFJi7BCycKZ5CodBB2vS9SeiV8q";

pub const POOL_ID_SOLUSDC: &str = "GUzaohfNuFbBqQTnPgPSNciv3aUvriXYjQduRE3ZkqFw";

#[allow(dead_code)]
pub const LP_TOKEN_V4_SOLUSDT: &str = "Epm4KfTj4DMrvqn6Bwg2Tr2N8vhQuNbuK8bESFp4k33K";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct StakeInfo {
    pub state: u64,
    pub pool_id: Pubkey,
    pub stake_owner: Pubkey,
    pub deposit_balance: u64,
    pub reward_debt: u128,
    pub reward_debt_b: u128,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct StakeInfoV5 {
    pub state: u64,
    pub pool_id: Pubkey,
    pub stake_owner: Pubkey,
    pub deposit_balance: u64,
    pub reward_debt: u128,
    pub reward_debt_b: u128,
    pub seq: u64,
}

// Get accounts staking ray token owned by owner.
pub fn get_ray_stake_info(client: &RpcClient, owner: &Pubkey) -> StakeInfo {
    let stake_program = Pubkey::from_str(STAKE_PROGRAM).unwrap();
    let res = get_program_accounts(client, owner, &stake_program).unwrap();
    let info: StakeInfo = bincode::deserialize(&res[0].1.data).unwrap();
    info
}

// Get accounts staking lp token owned by owner.
pub fn get_stake_info_v5(client: &RpcClient, owner: &Pubkey) -> Vec<(Pubkey, Account)> {
    let stake_program = Pubkey::from_str(STAKE_PROGRAM_V5).unwrap();
    let res = get_program_accounts(client, owner, &stake_program).unwrap();
    res
}

// Call getProgramAccounts of json rpc
pub fn get_program_accounts(
    client: &RpcClient,
    owner: &Pubkey,
    stake_program: &Pubkey,
) -> ClientResult<Vec<(Pubkey, Account)>> {
    // Create memory comparison to filter accounts
    let memcmp = RpcFilterType::Memcmp(Memcmp {
        offset: 40,
        bytes: MemcmpEncodedBytes::Base58(owner.to_string()),
        encoding: Some(MemcmpEncoding::Binary),
    });

    let config = RpcProgramAccountsConfig {
        filters: Some(vec![memcmp]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64Zstd),
            data_slice: None,
            // data_slice: Some(UiDataSliceConfig {offset: 0, length: 368}),
            commitment: Some(CommitmentConfig::processed()),
            min_context_slot: None,
        },
        with_context: None,
    };

    client.get_program_accounts_with_config(&stake_program, config)
}

pub fn get_lp_token_name(pool_id: &str) -> String {
    let lp_token_name = match pool_id {
        POOL_ID_SOLUSDT => String::from("SOL-USDT"),
        POOL_ID_SOLUSDC => String::from("SOL-USDC"),
        _ => String::from("unknown"),
    };
    lp_token_name
}
