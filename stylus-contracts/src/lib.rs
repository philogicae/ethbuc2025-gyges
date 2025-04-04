#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use alloy_primitives::{Address, FixedBytes, U256};
use alloy_sol_types::sol;
use stylus_sdk::{console, prelude::*};

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
        uint256 nb_wins;
        mapping(uint256 => uint256) game_ids;
    }
}

sol! {
    error UserNotInitialized();
    error UserAlreadyExists(string username);
    error UserNotFound(string username);
    error InvalidOperation(string message);
}

#[derive(SolidityError)]
pub enum GygesError {
    UserNotInitialized(UserNotInitialized),
    UserAlreadyExists(UserAlreadyExists),
    UserNotFound(UserNotFound),
    InvalidOperation(InvalidOperation),
}

#[public]
impl Gyges {
    pub fn register_username(&mut self, username: String) -> Result<(), GygesError> {
        if self.usernames.getter(username.clone()).get() != Address::new([0; 20]) {
            return Err(GygesError::UserAlreadyExists(UserAlreadyExists {
                username: username.clone(),
            }));
        }
        let sender = self.vm().msg_sender();
        self.usernames.setter(username.clone()).set(sender);
        self.players
            .setter(sender)
            .username
            .set_str(username.clone());
        console!("Username '{}' registered", username); // TODO: remove
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

    pub fn new_game(&mut self, adversary: String) -> Result<U256, GygesError> {
        let sender = self.vm().msg_sender();
        if self.players.get(sender).username.get_string() == "" {
            return Err(GygesError::UserNotInitialized(UserNotInitialized {}));
        }
        let adversary_addr = self.get_address_by_username(adversary.clone());
        if adversary_addr == Address::new([0; 20]) {
            return Err(GygesError::UserNotFound(UserNotFound {
                username: adversary,
            }));
        }
        /* if adversary_addr == sender {
            return Err(GygesError::InvalidOperation(InvalidOperation {
                message: "Cannot play against yourself".to_string(),
            }));
        } */
        // TODO: for quick tests
        let game_id = self.nb_games.get();
        let new_board = self.gen_new_board();
        // TODO: finish setup
        console!("Game {} created: {}", game_id, new_board);
        Ok(game_id)
    }
}

impl Gyges {
    pub fn lcg_step(&self, seed: &mut u64) -> u64 {
        let a: u64 = 1664525;
        let c: u64 = 1013904223;
        *seed = seed.wrapping_mul(a).wrapping_add(c);
        *seed
    }

    pub fn shuffle<T>(&self, slice: &mut [T], seed: &mut u64) {
        for i in (1..6).rev() {
            let random_val = self.lcg_step(seed);
            let j = (random_val % (i as u64 + 1)) as usize;
            slice.swap(i, j);
        }
    }

    pub fn gen_new_board(&self) -> FixedBytes<32> {
        let mut timestamp = self.vm().block_timestamp();
        let mut first_row: [u8; 6] = [1, 1, 2, 2, 3, 3];
        self.shuffle(&mut first_row, &mut timestamp);
        let mut last_row: [u8; 6] = [3, 3, 2, 2, 1, 1];
        self.shuffle(&mut last_row, &mut timestamp);
        let fst_row = [
            (first_row[0] << 4) | first_row[1],
            (first_row[2] << 4) | first_row[3],
            (first_row[4] << 4) | first_row[5],
        ]
        .to_vec();
        let lst_row = [
            (last_row[0] << 4) | last_row[1],
            (last_row[2] << 4) | last_row[3],
            (last_row[4] << 4) | last_row[5],
        ]
        .to_vec();
        FixedBytes::<32>::from_slice(
            [fst_row, [0; 12].to_vec(), lst_row, [0; 14].to_vec()]
                .concat()
                .as_slice(),
        )
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
