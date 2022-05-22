use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

pub const RAYDIUM_RPC_ENDPOINT: &str = "https://raydium.rpcpool.com";
pub const ENDPOINT_PRICE: &str = "https://api.raydium.io/v1/main/price";
pub const ENDPOINT_TOKEN: &str = "https://api.raydium.io/v1/main/token";

#[derive(Deserialize, Debug)]
//#[serde()]
pub struct Token {
    pub symbol: String,
    pub decimals: u8,
    pub icon: String,
}

//
// ```
// let price = raydium::api::get_price().await;
// ```
//
pub async fn get_prices() -> HashMap<String, f64> {
    let url = String::from(ENDPOINT_PRICE);
    let client = Client::builder().build().unwrap();
    let res = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<HashMap<String, f64>>()
        .await
        .unwrap();
    return res;
}

pub async fn get_token() -> HashMap<String, Value> {
    let url = String::from(ENDPOINT_TOKEN);
    let client = Client::builder().build().unwrap();
    client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<HashMap<String, Value>>()
        //.json()
        .await
        .unwrap()
}

pub async fn get_ray_staking_info() -> HashMap<String, Value> {
    let json_body = format!(
        r#"{{
                "id": "1e56ea48-bf83-4316-a0ed-8a0f9d4dde97",
                "method": "{}",
                "jsonrpc": "2.0",
                "params": [
                    "EhhTKczWMGQt46ynNeRX1WfeagwwJd7ufHvCDjRxjo5Q",
                    {{
                        "commitment": "confirmed",
                        "encoding": "base64",
                        "filters": [
                            {{
                                "memcmp": {{
                                    "offset": 40,
                                    "bytes": "J3fD5fZKjUvaeJwdNCsxTmkX3x5Z5CM3ipQLgETmCMAt"
                                }}
                            }}
                        ]
                    }}
                ]
            }}"#,
        "getProgramAccounts"
    );

    let url = String::from("https://raydium.rpcpool.com/");
    let client = Client::builder().build().unwrap();
    let a = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Origin", "https://v1.raydium.io")
        //.header("Referer", "https://v1.raydium.io/")
        //.header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.75 Safari/537.36")
        //.header(HeaderName::from_str(""))
        .body(json_body)
        .send()
        .await
        .unwrap()
        .json::<HashMap<String, Value>>()
        //.text()
        .await
        .unwrap();
    //let result: &Value = a.get(&String::from("result")).unwrap();
    //match *result {
    //    Value::Object => {
    //        //println!("{:?}", account);
    //    },
    //    _ => unreachable!
    //}
    //print_typename(result.unwrap().value());

    //let account: StakeInfoAccountV5 = bincode::deserialize(&).unwrap();
    a
}
