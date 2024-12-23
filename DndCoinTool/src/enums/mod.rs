use std::fmt;
mod enums;
mod conversions;



pub fn coinstack_to_coin_vec(coin_stack: CoinStack) -> Vec<Coin> {
    let mut coins = Vec::new();
    match coin_stack {
        CoinStack::Copper(amount) => for _ in 0..amount { coins.push(Coin::Copper); },
        CoinStack::Silver(amount) => for _ in 0..amount { coins.push(Coin::Silver); },
        CoinStack::Gold(amount) => for _ in 0..amount { coins.push(Coin::Gold); },
        CoinStack::Electrum(amount) => for _ in 0..amount { coins.push(Coin::Electrum); },
        CoinStack::Platinum(amount) => for _ in 0..amount { coins.push(Coin::Platinum); },
    }
    coins
}

pub use enums::{Coin, CoinStack};