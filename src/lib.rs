use scrypto::prelude::*;

blueprint! {
    struct MessiToken {
        messi_token: Vault
    }

    impl MessiToken {

        pub fn instantiate_mt() -> ComponentAddress {
            let mt_bucket: Bucket = ResourceBuilder::new_fungible().divisibility(18)
                .metadata("name", "MessiToken")
                .metadata("symbol", "MST")
                .initial_supply(1000);

            Self {
                messi_token: Vault::with_bucket(mt_bucket)
            }
            .instantiate()
            .globalize()
        }

        pub fn free_token(&mut self) -> Bucket {
            info!("My balance is: {} MessiToken. Now giving away a token!", self.messi_token.amount());
            self.messi_token.take(1)
        }
    }
}
