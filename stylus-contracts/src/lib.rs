#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use alloy_primitives::{Address, FixedBytes, U256};
use alloy_sol_types::sol;
use stylus_sdk::prelude::*;
use stylus_sdk::stylus_core::log;

sol_storage! {
    #[entrypoint]
    pub struct Gyges {
        uint256 nb_games;
        mapping(uint256 => Game) games;
        mapping(address => Player) players;
        mapping(string => address) usernames;
    }

    pub struct Game { // by id
        address player_one;
        address player_two;
        bytes32 state; // bytes: 6x6-board=18, padding=2, start=4, end=4, turn=4
    }

    pub struct Player { // by address
        string username;
        uint256 nb_games;
        uint256 nb_wins;
        mapping(uint256 => uint256) game_ids;
    }
}

sol! {
    event GameCreated(uint256 game_id, address player_one, address player_two);

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

    pub fn new_game(&mut self, adversary: String) -> Result<(), GygesError> {
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

        // Gyges layout
        self.nb_games.set(game_id + U256::from(1));

        // Game layout
        let mut new_game = self.games.setter(game_id);
        new_game.player_one.set(sender);
        new_game.player_two.set(adversary_addr);
        new_game.state.set(new_board);

        // Player 1 layout
        let player_one_nbg = self.players.get(sender).nb_games.get();
        let mut player_one = self.players.setter(sender);
        player_one.nb_games.set(player_one_nbg + U256::from(1));
        player_one.game_ids.setter(player_one_nbg).set(game_id);

        // Player 2 layout
        let player_two_nbg = self.players.get(adversary_addr).nb_games.get();
        let mut player_two = self.players.setter(adversary_addr);
        player_two.nb_games.set(player_two_nbg + U256::from(1));
        player_two.game_ids.setter(player_two_nbg).set(game_id);

        log(
            self.vm(),
            GameCreated {
                game_id,
                player_one: sender,
                player_two: adversary_addr,
            },
        );
        Ok(())
    }

    pub fn game_by_id(&self, game_id: U256) -> (Address, Address, FixedBytes<32>) {
        let game = self.games.get(game_id);
        (
            game.player_one.get(),
            game.player_two.get(),
            game.state.get(),
        )
    }
}

// Private methods
impl Gyges {
    // Linear congruential generator for pseudo-random numbers
    pub fn lcg_step(&self, seed: &mut u64) -> u64 {
        let a: u64 = 1664525;
        let c: u64 = 1013904223;
        *seed = seed.wrapping_mul(a).wrapping_add(c); // ranqd1
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
        let timestamp = self.vm().block_timestamp();
        let mut seed = timestamp;
        let mut first_row: [u8; 6] = [1, 1, 2, 2, 3, 3];
        let mut last_row: [u8; 6] = [3, 3, 2, 2, 1, 1];

        self.shuffle(&mut first_row, &mut seed);
        self.shuffle(&mut last_row, &mut seed);

        // Game state on a single bytes32
        FixedBytes::<32>::from_slice(
            [
                vec![
                    (first_row[0] << 4) | first_row[1],
                    (first_row[2] << 4) | first_row[3],
                    (first_row[4] << 4) | first_row[5],
                ], // 1 row (3 bytes)
                vec![0; 12], // 4 empty rows (12 bytes)
                vec![
                    (last_row[0] << 4) | last_row[1],
                    (last_row[2] << 4) | last_row[3],
                    (last_row[4] << 4) | last_row[5],
                ], // 1 row (3 bytes)
                vec![0; 2],  // padding (2 bytes)
                timestamp.to_be_bytes()[4..].to_vec(), // start (4 bytes)
                vec![0; 8],  // end=null (4 bytes) + turn=0 (4 bytes)
            ]
            .concat()
            .as_slice(),
        )
    }
}

/* #[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gyges() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut _contract = Gyges::from(&vm);
        // assert_eq!(U256::ZERO, contract.number());
    }
} */
