use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,near_bindgen};

const PUZZLE_NUMBER: u8 = 1; 

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    word: String,
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE
    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    } 

    pub fn get_word(&self) -> String {
        self.word.clone()
    }

    pub fn set_word(&mut self, word: String ) {
        self.word = word;
    }

    pub fn guess_word(&mut self, word: String){
        if word == self.word {
            env::log_str("You guessed right")
        }
        else {
            env::log_str("Wrong guess")
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
