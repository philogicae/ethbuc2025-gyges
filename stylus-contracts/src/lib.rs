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
        mapping(address => Player) players;
        mapping(string => address) usernames;
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
        self.players
            .setter(sender)
            .username
            .set_str(username.clone());
        console!("Username '{}' registered", username);
        Ok(())
    }

    pub fn get_address_by_username(&self, username: String) -> Address {
        self.usernames.get(username)
    }

    pub fn get_player_by_address(&self, address: Address) -> (String, U256) {
        let player = self.players.get(address);
        (player.username.get_string(), player.nb_games.get())
    }

    pub fn get_player_by_username(&self, username: String) -> (Address, String, U256) {
        let address = self.get_address_by_username(username);
        let player = self.get_player_by_address(address);
        (address, player.0, player.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gyges() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut _contract = Gyges::from(&vm);
        // assert_eq!(U256::ZERO, contract.number());
    }
}
