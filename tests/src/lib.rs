#[cfg(test)]
mod clubimmo_workshop;

#[cfg(test)]
mod deal;

#[cfg(test)]
mod tests {
    use super::clubimmo_workshop;
    //use casper_types::{account::AccountHash, U512, bytesrepr::Bytes};
    use clubimmo_workshop::ClubimmoWorkshopContract;
    use super::deal;

    #[test]
    fn should_store_deal_and_offers() {
        const ADDRESS: &str = "building address";
        let mut clubimmo_workshop = ClubimmoWorkshopContract::deploy();
        let address = String::from(ADDRESS);
        let asked_price: u64 = 100_000;
        clubimmo_workshop.call_new_building(address, asked_price);
        let deal: deal::Deal = clubimmo_workshop
            .query_contract(&ADDRESS).unwrap();

        assert_eq!(String::from(ADDRESS), deal.address);
        assert_eq!(asked_price, deal.asked_price);
        assert!(deal.offers.is_empty());
    }

    /*
    #[test]
    fn should_store_offer() {
        const ADDRESS: &str = "address2";
        let mut clubimmo_workshop = ClubimmoWorkshopContract::deploy();
        let address: String = String::from(ADDRESS);
        let offered_price: u64 = 90_000;
        clubimmo_workshop.call_make_offer(address, offered_price);
        let _json: String = clubimmo_workshop.query_contract(&ADDRESS).unwrap();
        //assert_eq!(String::from("Hello World"), check);
    }
    */
}
