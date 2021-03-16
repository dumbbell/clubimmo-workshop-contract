extern crate alloc;
use alloc::{collections::BTreeSet, string::String};
use std::convert::TryInto;
use std::collections::HashMap;

use casperlabs_contract_macro::{casperlabs_constructor, casperlabs_contract, casperlabs_method};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    bytesrepr::ToBytes,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    runtime_args, CLType, CLTyped, CLValue, Group, Parameter, RuntimeArgs,
    URef,
};

mod deal;

#[casperlabs_contract]
mod clubimmo_workshop_contract {

    #[casperlabs_constructor]
    fn init() {}

    #[casperlabs_method]
    fn new_building(address: String, asked_price: u64) {
        match runtime::get_key(address.as_str()) {
            None => {
                let deal = deal::Deal {
                    address: address.clone(),
                    asked_price: asked_price,
                    offers: HashMap::new(),
                };
                let key = storage::new_uref(deal).into();
                runtime::put_key(address.as_str(), key);
            }
            Some(_) => {}
        }
    }

    #[casperlabs_method]
    fn make_offer(address: String, offered_price: u64) {
        match runtime::get_key(address.as_str()) {
            Some(key) => {
                let key_ref = key.try_into().unwrap_or_revert();
                let mut deal: deal::Deal = storage::read(key_ref).unwrap().unwrap();

                deal.offers.insert(runtime::get_caller(), offered_price);
                storage::write(key_ref, deal);
            }
            None => {}
        }
    }

    /*
    #[casperlabs_method]
    fn get_winner(address: String) -> String {
        match runtime::get_key(address.as_str()) {
            Some(key) => {
                let key_ref = key.try_into().unwrap_or_revert();
                let deal: Deal = storage::read(key_ref).unwrap().unwrap();

                if deal.offers.is_empty() {
                    return String::new();
                }

                let mut winner: String = String::new();
                let mut max: u64 = 0;
                for (accounthash, offered_price) in deal.offers.iter() {
                    if *offered_price > max {
                        max = *offered_price;
                        winner = accounthash.to_formatted_string();
                    }
                }

                winner
            }
            None => {
                String::new()
            }
        }
    }*/
}
