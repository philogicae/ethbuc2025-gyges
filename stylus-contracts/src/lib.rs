#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use alloy_primitives::{Address, FixedBytes, U256};
use alloy_sol_types::sol;
use stylus_sdk::{block, console, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct Gyges {
        uint256 nb_games;
        mapping(uint256 => Game) games;
        mapping(string => address) usernames;
        mapping(address => Player) players;
    }

    pub struct Game { // by id
        address playerone;
        address playertwo;
        bytes32 state; // board, turn, win
        uint256 start;
        uint256 end;
    }

    pub struct Player { // by address
        string username;
        uint256 nb_games;
        mapping(uint256 => uint256) game_ids;
    }
}

sol! {
    error UsernameAlreadyExists(string username);
}

#[derive(SolidityError)]
pub enum GygesError {
    UsernameAlreadyExists(UsernameAlreadyExists),
}

#[public]
impl Gyges {
    pub fn register_username(&mut self, username: String) -> Result<(), GygesError> {
        if self.usernames.getter(username.clone()).get() != Address::new([0; 20]) {
            return Err(GygesError::UsernameAlreadyExists(UsernameAlreadyExists {
                username: username.clone(),
            }));
        }
        let sender = self.vm().msg_sender();
        self.usernames.setter(username.clone()).set(sender);
        console!("Username '{}' registered", username);
        Ok(())
    }

    /* /// Gets the number from storage.
    pub fn number(&self) -> U256 {
        self.number.get()
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }

    /// Sets a number in storage to a user-specified value.
    pub fn mul_number(&mut self, new_number: U256) {
        self.number.set(new_number * self.number.get());
    }

    /// Sets a number in storage to a user-specified value.
    pub fn add_number(&mut self, new_number: U256) {
        self.number.set(new_number + self.number.get());
    }

    /// Increments `number` and updates its value in storage.
    pub fn increment(&mut self) {
        let number = self.number.get();
        self.set_number(number + U256::from(1));
    }

    /// Adds the wei value from msg_value to the number in storage.
    #[payable]
    pub fn add_from_msg_value(&mut self) {
        let number = self.number.get();
        self.set_number(number + self.vm().msg_value());
    } */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gyges() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut _contract = Gyges::from(&vm);

        /* assert_eq!(U256::ZERO, contract.number());

        contract.increment();
        assert_eq!(U256::from(1), contract.number());

        contract.add_number(U256::from(3));
        assert_eq!(U256::from(4), contract.number());

        contract.mul_number(U256::from(2));
        assert_eq!(U256::from(8), contract.number());

        contract.set_number(U256::from(100));
        assert_eq!(U256::from(100), contract.number());

        // Override the msg value for future contract method invocations.
        vm.set_value(U256::from(2));

        contract.add_from_msg_value();
        assert_eq!(U256::from(102), contract.number()); */
    }
}
