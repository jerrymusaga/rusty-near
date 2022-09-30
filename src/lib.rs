use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,near_bindgen};

const PUZZLE_NUMBER: u8 = 1; 

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    word: String,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            word: String::from("No Default word")
        }
    }
}

#[near_bindgen]
impl Contract {
    // ADD CONTRACT METHODS HERE

    #[init]
    pub fn new(word: String) -> Self {
        Self {
            word
        }
    }

    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    } 

    pub fn set_word(&mut self, word: String) {
        self.word = word;
    }

    pub fn get_word(&self) -> String {
        self.word.clone()
    }

    pub fn guess_word(&mut self, word: String) -> bool {
        if word == self.word {
            env::log_str("You guessed right!");
            true
        }
        else {
            env::log_str("Wrong guess");
            false
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
    #[test]
    fn debug_get_hash() {
        // Basic set up for a unit test
        testing_env!(VMContextBuilder::new().build());

        // Using a unit test to rapidly debug and iterate
        let debug_solution = "I love Polkadot";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    #[test]
fn check_guess_word() {
    // Get Alice as an account ID
    let alice = AccountId::new_unchecked("alice.testnet".to_string());
    // Set up the testing context and unit test environment
    let context = get_context(alice);
    testing_env!(context.build());

    // Set up contract object and call the new method
    let mut contract = Contract::new(
        "49ac7213f0e1c58134e5e2fba771d99ebec4f36c692301747e1b3299558134a8".to_string(),
    );
    let mut guess_result = contract.guess_word("wrong answer here".to_string());
    assert!(!guess_result, "Expected a failure from the wrong guess");
    assert_eq!(get_logs(), ["Wrong guess"], "Expected a failure log.");
    guess_result = contract.guess_word("near nomicon ref finance".to_string());
    assert!(guess_result, "Expected the correct answer to return true.");
    assert_eq!(
        get_logs(),
        ["Wrong guess", "You guessed right!"],
        "Expected a successful log after the previous failed log."
    );
}
}
