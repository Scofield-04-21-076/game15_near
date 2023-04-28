use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, BorshStorageKey, env, log, near_bindgen, require};
use near_sdk::collections::{LookupMap};
use near_sdk::serde::{Deserialize, Serialize};


const SIZE: usize = 16;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey{
    VectorNumber,
    Players,
    Games,

}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde" )]
pub struct Player {
    is_finish_game: bool,

}

impl Default for Player {
    fn default() -> Self {
        Self {
            is_finish_game: true,
        }
    }
}

#[derive(Default, BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde" )]
pub struct Game {
    pub tiles: [u8; SIZE],

}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Pazzle {
    pub players: LookupMap<AccountId, Player>,
    pub games: LookupMap<AccountId, Game>,

}


impl Default for Pazzle {
    fn default() -> Self {
        Pazzle{
            players: LookupMap::new(StorageKey::Players),
            games: LookupMap::new(StorageKey::Games),
        }
    }
}

#[near_bindgen]
impl Pazzle {
    pub fn new_game(&mut self, shuffle: [u8; SIZE]) {

        self.check_tiles(shuffle.clone());

        require!(self.is_solvable(shuffle.clone()),
                "the resulting permutation does not resolve");

        let mut game: Game = Game::default();
        game.tiles = shuffle.clone();
        self.games.insert(&env::predecessor_account_id(), &game);

        let mut player: Player = Player::default();
        player.is_finish_game = false;
        self.players.insert(&env::predecessor_account_id(), &player);
    }

    #[private]
    pub fn is_solvable(&self, tiles: [u8; SIZE]) -> bool {
        let mut count_inversions: u8 = 0;

        for i in 0..SIZE-1 {
            for j in 0..i {
                if tiles[j] > tiles[i] {
                    count_inversions += 1;
                }
            }
        }

        count_inversions % 2 != 0
    }



    pub fn run(&mut self, tiles: [u8; SIZE]) {

        let mut tiles_copy = tiles.clone();

        self.check_tiles(tiles.clone());

        let x: u8;
        let x0: u8;

        let mut x_x0: Vec<u8> = Vec::new();

        let mut game = self.expect_value_found(
            self.games.get(&env::predecessor_account_id()));
        let game_tiles = game.tiles.clone();

        for i in 0..SIZE {

            if game_tiles[i] != tiles[i] {

                x_x0.push(i as u8);
            }
        }

        require!(x_x0.len() != 0, "the move was not made");
        require!(x_x0.len() == 2, "only one permutation can be done in one turn");

        match tiles[x_x0[0] as usize] {
            0 => {
                x = x_x0[0];
                x0 = x_x0[1];
            },
            _ => {
                x0 = x_x0[0];
                x = x_x0[1];
            }
        }

        let i_x: i8 = x as i8;
        let i_x0: i8 = x0 as i8;

        require!(
            game_tiles[x as usize] == 0 &&
            ((i_x % 4 != 0 && i_x0 % 4 != 3) ||
                (i_x0 % 4 != 0 && i_x % 4 != 3)) &&
            (((i_x0 - i_x) == 1 || (i_x0 - i_x) == -1 ) ||
                ((i_x0 - i_x) == 4 || (i_x0 - i_x) == -4 )),
            "not a correct move");

        let value = tiles_copy[x0 as usize];
        tiles_copy[x as usize] = value;
        tiles_copy[x0 as usize] = 0;

        game.tiles = tiles_copy;

        self.games.insert(&env::predecessor_account_id(), &game);
        log!("the move is successful");
    }

    pub fn get_tiles(&self) -> [u8; SIZE] {
        let game = self.expect_value_found(
            self.games.get(&env::predecessor_account_id()));

        game.tiles
    }







}

impl Pazzle {

    pub fn check_tiles(&self, tiles: [u8; SIZE]) {

        require!(tiles.len() == SIZE,
            "unexpected number of values (16 needed)");

        for i in 0..SIZE {
            let buff = tiles[i];

            require!(buff <= 15,
                "unexpected number of values (0-15 needed)");

            for j in 0..SIZE {
                if i != j {
                    require!(buff != tiles[j],
                        "unexpected number of values (repetition of values)");
                }
            }
        }
    }

    pub fn is_solved(&self) -> bool {

        let game = self.expect_value_found(
            self.games.get(&env::predecessor_account_id()));

        if game.tiles[SIZE-1] != 0 {
            return false;
        }

        for i in (0..SIZE).rev() {
            if game.tiles[i] != (i + 1) as u8 {
                return false;
            }
        }

        true
    }

    pub fn expect_value_found<T>(&self, option: Option<T>) -> T {

        option.unwrap_or_else(|| env::panic_str("Not found"))
    }
}
