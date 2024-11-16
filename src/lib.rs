use near_sdk::{
    env,
    log,
    near,
    require,
    AccountId,
    NearToken,
    PanicOnDefault,
    Promise,
    json_types::U64
};


#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Bid {
    pub bidder: AccountId,
    pub bid: NearToken
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    highest_bid: Bid,
    auction_end_time: U64,
    auctioneer: AccountId,
    claimed: bool,
}

#[near]
impl Contract {

    #[init]
    #[private]
    pub fn init(end_time: U64, auctioneer: AccountId) -> Self {
        Self {
            highest_bid: Bid {
                bidder: env::current_account_id(),
                bid: NearToken::from_yoctonear(1),
            },
            auction_end_time: end_time,
            auctioneer: auctioneer,
            claimed: false,
        }
    }

    #[payable]
    pub fn bid(&mut self) -> Promise {
        require!(env::block_timestamp() < self.auction_end_time.into(), "Auction had ended");

        let bid = env::attached_deposit();
        let bidder = env::predecessor_account_id();

        let Bid {
            bidder: last_bidder,
            bid: last_bid
        } = self.highest_bid.clone();

        require!(bid > last_bid, "bid must exceed highest bid");

        self.highest_bid = Bid { bid, bidder };

        // transfer tokens back to previous high bidder
        Promise::new(last_bidder).transfer(last_bid)
    }

    pub fn claim(&mut self) -> Promise {
        require!(env::block_timestamp() > self.auction_end_time.into(), "auction not ended");
        require!(!self.claimed, "auction already claimed");
        self.claimed = true;
        // Transfer tokens to auctioneer
        Promise::new(self.auctioneer.clone()).transfer(self.highest_bid.bid)
    }

    pub fn get_highest_bid(&self) -> &Bid {
        &self.highest_bid
    }

    pub fn get_auction_end_time(&self) -> &U64 {
        &self.auction_end_time
    }

}


// // Define the contract structure
// #[near(contract_state)]
// pub struct Contract {
//     greeting: String,
// }

// // Define the default, which automatically initializes the contract
// impl Default for Contract {
//     fn default() -> Self {
//         Self {
//             greeting: "Hello".to_string(),
//         }
//     }
// }

// // Implement the contract structure
// #[near]
// impl Contract {
//     // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
//     pub fn get_greeting(&self) -> String {
//         self.greeting.clone()
//     }

//     // Public method - accepts a greeting, such as "howdy", and records it
//     pub fn set_greeting(&mut self, greeting: String) {
//         log!("Saving greeting: {greeting}");
//         self.greeting = greeting;
//     }
// }

// /*
//  * The rest of this file holds the inline tests for the code above
//  * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
//  */
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn get_default_greeting() {
//         let contract = Contract::default();
//         // this test did not call set_greeting so should return the default "Hello" greeting
//         assert_eq!(contract.get_greeting(), "Hello");
//     }

//     #[test]
//     fn set_then_get_greeting() {
//         let mut contract = Contract::default();
//         contract.set_greeting("howdy".to_string());
//         assert_eq!(contract.get_greeting(), "howdy");
//     }
// }
