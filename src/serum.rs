pub const MARKET_RAY_SRM: &str = "Cm4MmknScg7qbKqytb1mM92xgDxv3TNXos4tKbBqTDy7";

pub const PROGRAM_SERUM_DEX_V3: &str = "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin";

pub const OPEN_ORDERS_RAY_SOL: &str = "JQEY8R9frhxuvcsewGfgkCVdGWztpHLx4P9zmTAsZFM";

pub const OPEN_ORDERS_RAY_SRM: &str = "6CVRtzecMaPZ1pdfT2ZzJ1qf89yuFsD7MKYGwvjYsy6w";

//use serum_dex::critbit::Slab;
use serum_dex::state::Market;
//use serum_dex::state::MarketState;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account_info::IntoAccountInfo;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn aaa(client: &RpcClient) {
    // get multiple accounts (serum market, serum open order and rent)
    let market_ray_srm: Pubkey = Pubkey::from_str(MARKET_RAY_SRM).unwrap();
    let open_orders_ray_srm: Pubkey = Pubkey::from_str(OPEN_ORDERS_RAY_SRM).unwrap();
    let rent_id: Pubkey = solana_program::sysvar::rent::id();
    let pubkeys: Vec<Pubkey> = vec![market_ray_srm, open_orders_ray_srm, rent_id];
    let mut accounts = client.get_multiple_accounts(&pubkeys).unwrap();

    //let market_account = std::mem::take(&mut accounts[0]);
    //println!("take >> {:?}", market_account);

    let serum_program_id: Pubkey = Pubkey::from_str(PROGRAM_SERUM_DEX_V3).unwrap();

    let mut mkt_act = std::mem::take(&mut accounts[0]).unwrap();
    let market_account_info = (&market_ray_srm, &mut mkt_act).into_account_info();
    //println!("market_account_info : {:?}", &market_account_info);

    //let mut oo_act = std::mem::take(&mut accounts[1]).unwrap();
    //let open_order_account_info = (&open_orders_ray_srm, &mut oo_act).into_account_info();
    //println!("open_order_account_info  : {:?}", &open_order_account_info);

    //let market_state = MarketState::load(&market_account_info, &serum_program_id, false).unwrap();
    //println!("market_state: {:?}", &market_state);
    //println!(">>> coin_lot_size: {:?}", &market_state.coin_lot_size);
    //println!(">>> pc_lot_size: {:?}", &market_state.pc_lot_size);
    //println!(">>> asks: {:?}", &market_state.asks);

    let market: Market = Market::load(&market_account_info, &serum_program_id, true).unwrap();
    print_typename(&market);

    // ask: 7bKEjcZEqVAWsiRGDnxXvTnNwhZLt2SH6cHi5hpcg5de
    // bid: G65a5G6xHpc9zV8tGhVSKJtz7AcAJ8Q3hbMqnDJQgMkz
    let u: Pubkey = Pubkey::from_str("7bKEjcZEqVAWsiRGDnxXvTnNwhZLt2SH6cHi5hpcg5de").unwrap();
    let mut o = client.get_account(&u).unwrap();
    let p = (&u, &mut o).into_account_info();

    use std::ops::DerefMut;
    let mut asks = market.load_asks_mut(&p).unwrap();
    let mut x = asks.deref_mut();
    //println!(">>> find_max: {:?}", asks.find_max().unwrap());
    //println!(">>> find_min: {:?}", &asks.find_min().unwrap());
    let min = &x.find_min().unwrap();
    let max = &x.find_max().unwrap();
    print_typename(x);
    println!(">>> min: {:?}", &min);
    println!(">>> max: {:?}", &max);

    //let o = marke
    //    .load_orders_mut(
    //        &open_order_account_info,
    //        Some(&open_order_account_info),
    //        &serum_program_id,
    //        None,
    //        None,
    //    )
    //    .unwrap();
    //println!("{:?}", c);

    //use std::convert::identity;
    //let a: Vec<u8> = vec![
    //    market_state.asks[0],
    //    market_state.asks[1],
    //    market_state.asks[2],
    //    market_state.asks[3],
    //];
    //let k: Pubkey = Pubkey::new(a).unwrap();
    //println!("asks: {:?}", identity(market_state.asks) as [_]);

    //let o: std::cell::RefMut<Slab> = market_state.load_asks_mut(&market_account_info).unwrap();
    //println!("find_max: {:?}", o.deref_mut().or);

    //let asks = market_state.load_asks_mut(&account_info).unwrap();
    //println!("asks: {:?}", asks.root());
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
