use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coin {
    Copper,
    Silver,
    Gold,
    Electrum,
    Platinum,
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Coin::Copper => write!(f, "Copper"),
            Coin::Silver => write!(f, "Silver"),
            Coin::Gold => write!(f, "Gold"),
            Coin::Electrum => write!(f, "Electrum"),
            Coin::Platinum => write!(f, "Platinum"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CoinStack {
    Copper(u64),
    Silver(u64),
    Gold(u64),
    Electrum(u64),
    Platinum(u64),
}

const COPPER_TO_COPPER: u64 = 1;
const SILVER_TO_COPPER: u64 = 10;
const ELECTRUM_TO_COPPER: u64 = 50;
const GOLD_TO_COPPER: u64 = 100;
const PLATINUM_TO_COPPER: u64 = 1000;
impl CoinStack {
    pub fn to_copper_stack(&self) -> CoinStack {
        match self {
            CoinStack::Copper(_) => {
                *self
            }
            CoinStack::Silver(coins) => {
                CoinStack::Copper(coins * SILVER_TO_COPPER)
            }
            CoinStack::Gold(coins) => {
                CoinStack::Copper(coins * GOLD_TO_COPPER)
            }
            CoinStack::Electrum(coins) => {
                CoinStack::Copper(coins * ELECTRUM_TO_COPPER)
            }
            CoinStack::Platinum(coins) => {
                CoinStack::Copper(coins * PLATINUM_TO_COPPER)
            }
        }
    }

    pub fn count(&self) -> u64 {
        match self {
            CoinStack::Copper(coins) => *coins,
            CoinStack::Silver(coins) => *coins,
            CoinStack::Gold(coins) => *coins,
            CoinStack::Electrum(coins) => *coins,
            CoinStack::Platinum(coins) => *coins,
        }
    }
}

impl fmt::Display for CoinStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoinStack::Copper(amount) => write!(f, "{} Copper", amount),
            CoinStack::Silver(amount) => write!(f, "{} Silver", amount),
            CoinStack::Gold(amount) => write!(f, "{} Gold", amount),
            CoinStack::Electrum(amount) => write!(f, "{} Electrum", amount),
            CoinStack::Platinum(amount) => write!(f, "{} Platinum", amount),
        }
    }
}

impl From<Coin> for CoinStack {
    fn from(coin: Coin) -> Self {
        match coin {
            Coin::Copper => CoinStack::Copper(1),
            Coin::Silver => CoinStack::Silver(1),
            Coin::Gold => CoinStack::Gold(1),
            Coin::Electrum => CoinStack::Electrum(1),
            Coin::Platinum => CoinStack::Platinum(1),
        }
    }
}