use std::fs;

use ethabi::ethereum_types::H160;
use ethabi::Token::Address;
use ethabi::{Contract, Token};
use hex;

fn main() {

    // load abi
    let contract = Contract::load(fs::read("res/contract.abi").unwrap().as_slice()).unwrap();
    let function = contract.function("quoteExactInputSingle").unwrap();

    // method id removed data
    let encoded = function.encode_input(&[
        Address(H160::from(
            TryInto::<[u8; 20]>::try_into(
                hex::decode("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(),
            )
            .unwrap(),
        )),
        Address(H160::from(
            TryInto::<[u8; 20]>::try_into(
                hex::decode("dac17f958d2ee523a2206206994597c13d831ec7").unwrap(),
            )
            .unwrap(),
        )),
        Token::Uint(500.into()),
        Token::Uint(1000000000000000000i64.into()),
        Token::Uint(0.into()),
    ]);

    let encoded_final = format!("0x{}", hex::encode(encoded.unwrap()));
    println!("{:?}", encoded_final);

}
