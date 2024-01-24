#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod metadogo {
    use ink::env::hash;
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct Metadogo {
        last_feed_time: Timestamp,
        last_play_time: Timestamp,
        salt: u64,
    }

    #[ink(event)]
    pub struct Released {
        value: Balance,
        to: AccountId,
    }

    impl Metadogo {
        #[ink(constructor)]
        pub fn new(last_feed_time: Timestamp, last_play_time: Timestamp, salt: u64) -> Self {
            Self {
                last_feed_time,
                last_play_time,
                salt,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(
                Self::env().block_timestamp(),
                Self::env().block_timestamp(),
                0,
            )
        }

        #[ink(message, payable)]
        pub fn throw_ball(&mut self) {
            let received_amount = self.env().transferred_value();
            self.last_play_time = self.env().block_timestamp();
            if received_amount > 0 {
                let balance = self.env().balance();
                if balance >= 10 {
                    let n = self.get_pseudo_random(254) as u128;
                    if n < 90 {
                        let win_amount = balance / (n + 10);
                        let beneficiary = self.env().caller();
                        self.env()
                            .transfer(beneficiary, win_amount)
                            .expect("Transfer failed");
                        self.env().emit_event(Released {
                            value: win_amount,
                            to: beneficiary,
                        });
                    }
                }
            }
        }

        #[ink(message)]
        pub fn get_last_feed_time(&self) -> Timestamp {
            self.last_feed_time
        }

        #[ink(message)]
        pub fn get_last_play_time(&self) -> Timestamp {
            self.last_play_time
        }

        fn get_pseudo_random(&mut self, max_value: u8) -> u8 {
            let seed = self.env().block_timestamp();
            let mut input: Vec<u8> = Vec::new();
            input.extend_from_slice(&seed.to_be_bytes());
            input.extend_from_slice(&self.salt.to_be_bytes());
            let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
            ink::env::hash_bytes::<hash::Keccak256>(&input, &mut output);
            self.salt += 1;
            let number = output[0] % (max_value + 1);
            number
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let metadogo = Metadogo::default();
            assert_eq!(metadogo.get_last_play_time(), metadogo.last_play_time);
            assert_eq!(metadogo.get_last_feed_time(), metadogo.last_feed_time);
            assert_eq!(metadogo.salt, 0);
        }

        #[ink::test]
        fn throw_ball_without_value_works() {
            let play_time = 0 as Timestamp;
            let feed_time = 0;
            let salt = 0;
            let mut dogo = Metadogo::new(feed_time, play_time, salt);
            assert_eq!(dogo.get_last_play_time(), 0);
            assert_eq!(dogo.get_last_feed_time(), 0);
            assert_eq!(dogo.salt, salt);
            dogo.throw_ball();
            assert_eq!(dogo.get_last_play_time(), 0);
            assert_eq!(dogo.get_last_feed_time(), 0);
            assert_eq!(dogo.salt, salt);
        }

        #[ink::test]
        fn throw_ball_with_value_works() {
            let initial_timestamp = 10;
            let mut dogo = create_contract(initial_timestamp, 10);
            assert_eq!(dogo.get_last_play_time(), initial_timestamp);
            assert_eq!(dogo.get_last_feed_time(), initial_timestamp);
            assert_eq!(dogo.salt, 0);
            let current_timestamp = 1000; // random number: 82
            let accounts = default_accounts();
            set_block_timestamp(current_timestamp);
            let caller_balance_initial = 100;
            set_balance(accounts.eve, caller_balance_initial);
            set_sender(accounts.eve);
            ink::env::pay_with_call!(dogo.throw_ball(), 100);
            assert_eq!(dogo.get_last_play_time(), current_timestamp);
            assert_eq!(dogo.get_last_feed_time(), initial_timestamp);
            assert_eq!(dogo.salt, 1);
            let caller_balance_new = get_balance(accounts.eve);
            assert_ne!(caller_balance_initial, caller_balance_new)
        }

        #[ink::test]
        fn throw_ball_with_value_but_no_payout_works() {
            let initial_timestamp = 10;
            let mut dogo = create_contract(initial_timestamp, 10);
            assert_eq!(dogo.get_last_play_time(), initial_timestamp);
            assert_eq!(dogo.get_last_feed_time(), initial_timestamp);
            assert_eq!(dogo.salt, 0);
            let current_timestamp = 1001; // random number: 192
            let accounts = default_accounts();
            set_block_timestamp(current_timestamp);
            let caller_balance_initial = 100;
            set_balance(accounts.eve, caller_balance_initial);
            set_sender(accounts.eve);
            ink::env::pay_with_call!(dogo.throw_ball(), 100);
            assert_eq!(dogo.get_last_play_time(), current_timestamp);
            assert_eq!(dogo.get_last_feed_time(), initial_timestamp);
            assert_eq!(dogo.salt, 1);
            let caller_balance_new = get_balance(accounts.eve);
            assert_eq!(caller_balance_initial, caller_balance_new)
        }

        fn create_contract(block_timestamp: Timestamp, initial_balance: Balance) -> Metadogo {
            // let accounts = default_accounts();
            set_block_timestamp(block_timestamp);
            // set_sender(accounts.alice);
            set_balance(contract_id(), initial_balance);

            Metadogo::new(block_timestamp, block_timestamp, 0)
        }

        fn set_block_timestamp(timestamp: Timestamp) {
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(timestamp)
        }

        fn contract_id() -> AccountId {
            ink::env::test::callee::<ink::env::DefaultEnvironment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(account_id, balance)
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink::env::test::get_account_balance::<ink::env::DefaultEnvironment>(account_id)
                .expect("Cannot get account balance")
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;

        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = MetadogoRef::default();

            // When
            let contract_account_id = client
                .instantiate("metadogo", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<MetadogoRef>(contract_account_id.clone())
                .call(|metadogo| metadogo.get_last_play_time());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), metadogo.last_play_time));

            Ok(())
        }
    }
}
