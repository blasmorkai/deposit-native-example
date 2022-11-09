use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

// derive - implements the following traits in the struct Config. 
// Serialize/Deserialize to JSON schema, Clone (copy), Debug (can be formatted with the debug trait :?), 
// PartialEq - Partial equivalence relations (e.g. we can compare with only one struct field), JsonSchema 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr
}


// Struct that is going to be stored in cw_storage_plus DEPOSITS
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposits {
    pub count: i32,
    pub owner: Addr,
    pub coins: Coin
}

//Map using a tuple as key, and the Struct Deposits as value DEPOSITS: Map<(&address, &coin_denom), Deposit>
//key is address, denom
pub const DEPOSITS: Map<(&str, &str), Deposits> = Map::new("deposits");

pub const CONFIG: Item<Config> = Item::new("config");