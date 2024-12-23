use crate::enums::{Coin, CoinStack};

pub fn priority_for_coinstack(coinstack: CoinStack) -> u32 {
    match coinstack {
        CoinStack::Copper(_) => 1,
        CoinStack::Silver(_) => 2,
        CoinStack::Gold(_) => 3,
        CoinStack::Electrum(_) => 4,
        CoinStack::Platinum(_) => 5,
    }
}
pub fn priority_for_coin(coin: Coin) -> u32 {
    match coin {
        Coin::Copper => 1,
        Coin::Silver => 2,
        Coin::Gold => 3,
        Coin::Electrum => 4,
        Coin::Platinum => 5,
    }
}

const COINS_BY_PRIORITY: [(Coin, CoinStack); 5] = [
    (Coin::Platinum, CoinStack::Platinum(0)),
    (Coin::Gold, CoinStack::Gold(0)),
    (Coin::Electrum, CoinStack::Electrum(0)),
    (Coin::Silver, CoinStack::Silver(0)),
    (Coin::Copper, CoinStack::Copper(0)),
];

pub fn convert_coin_stack(coinstack: CoinStack, to: Coin) -> (CoinStack, u32) {
    let priority_coinstack = priority_for_coinstack(coinstack);
    let priority_to = priority_for_coin(to);
    
    let mut peekable = COINS_BY_PRIORITY.iter().peekable();
    loop {
        match peekable.peek() {
            Some((coin, coin_stack)) => {
                todo!()
            },
            None => break,
        }
    }
    todo!()
}
