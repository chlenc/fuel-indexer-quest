use std::{env, str::FromStr};

use dotenv::dotenv;
use fuels::{
    accounts::fuel_crypto::rand::{self, Rng},
    prelude::{abigen, Bech32ContractId, Provider, TxParameters, WalletUnlocked},
    types::{ContractId, SizedAsciiString},
};
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/contract-abi.json"
));

const RPC: &str = "beta-3.fuel.network";
const CONTRACT_ID: &str = "0x3dd1c6e811fa1eb21e649aca12630e44f6a937c1b33b00fb26e4716230fd7b69";

#[tokio::test]
async fn can_get_contract_id() {
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let pk = env::var("PK").unwrap().parse().unwrap();
    let wallet = WalletUnlocked::new_from_private_key(pk, Some(provider.clone()));

    let contract_id: Bech32ContractId = ContractId::from_str(&CONTRACT_ID).unwrap().into();
    let instance = MyContract::new(contract_id, wallet);

    let id = rand::thread_rng().gen_range(0..100000);
    let greeting: SizedAsciiString<32> =
        SizedAsciiString::new(String::from("Hello                           ")).unwrap();
    let person_name: SizedAsciiString<32> =
        SizedAsciiString::new(String::from("Indexer                         ")).unwrap();
    instance
        .methods()
        .new_greeting(id, greeting, person_name)
        .tx_params(TxParameters::default().set_gas_price(1))
        .call()
        .await
        .unwrap();
}
