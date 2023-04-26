use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, BorshStorageKey, env, log, near_bindgen, require};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::serde::{Deserialize, Serialize};


const SIZE: u8 = 16;

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
    pub tiles: String,

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
    pub fn new_game(&mut self, shuffle: String) {

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
    pub fn is_solvable(&self, tiles: String) -> bool {
        let mut count_inversions:u8 = 0;

        let tiles_vec = self.string_to_vector(tiles);

        for i in 0..SIZE-1 {
            for j in 0..i {
                if tiles_vec.get(j as u64) > tiles_vec.get(i as u64) {
                    count_inversions += 1;
                }
            }
        }

        count_inversions % 2 == 0
    }



    pub fn run(&mut self, tiles: String) {

        let tiles_vec = self.string_to_vector(tiles.clone());

        self.check_tiles(tiles.clone());

        let x: u8;
        let x0: u8;

        let mut x_x0: Vector<u8> = Vector::new(b"vec-uid-1".to_vec());

        let mut game = self.expect_value_found(
            self.games.get(&env::predecessor_account_id()));
        let game_tiles = self.string_to_vector(game.tiles);

        for i in 0..SIZE {
            if self.expect_value_found(
                game_tiles.get(i as u64)) != self.expect_value_found(tiles_vec.get(i as u64)) {
                x_x0.push(&i);
            }
        }

        require!(x_x0.len() != 0, "the move was not made");
        require!(x_x0.len() == 2, "only one permutation can be done in one turn");

        match self.expect_value_found(tiles_vec.get(self.expect_value_found(x_x0.get(0)) as u64)) {
            0 => {
                x = self.expect_value_found(x_x0.get(0));
                x0 = self.expect_value_found(x_x0.get(1));
            },
            _ => {
                x0 = self.expect_value_found(x_x0.get(0));
                x = self.expect_value_found(x_x0.get(1));
            }
        }

        let i_x: i8 = x as i8;
        let i_x0: i8 = x0 as i8;

        require!(
            self.expect_value_found(game_tiles.get(x.into())) == 0 &&
            ((i_x % 4 != 0 && i_x0 % 4 != 3) ||
                (i_x0 % 4 != 0 && i_x % 4 != 3)) &&
            (((i_x0 - i_x) == 1 || (i_x0 - i_x) == -1 ) ||
                ((i_x0 - i_x) == 4 || (i_x0 - i_x) == -4 )),
            "not a correct move");

        let value = self.expect_value_found(tiles_vec.get(x0 as u64));

        let mut vec_to_string: String = "".to_string();
        for i in 0..SIZE {
            vec_to_string +=
                &self.expect_value_found(game_tiles.get(i as u64)).to_string();
            if i != SIZE - 1 {
                vec_to_string += ",";
            }
        }

        vec_to_string = vec_to_string.replace(
            &(self.expect_value_found(tiles_vec.get(x as u64))).to_string(),
            &value.to_string());

        vec_to_string = vec_to_string.replace(
            &(self.expect_value_found(tiles_vec.get(x0 as u64))).to_string(),
            "0");

        game.tiles = vec_to_string;

        self.games.insert(&env::predecessor_account_id(), &game);
        log!("the move is successful");
    }








}

impl Pazzle {
    pub fn string_to_vector(&self, tiles: String) -> Vector<u8>{
        let mut vector: Vector<u8> = Vector::new(b"vec-uid-1".to_vec());

        let v: Vec<u8> = tiles.split(',').map(|x| x.parse::<u8>().unwrap()).collect();

        for x in &v {
            vector.push(x);
        }

        vector
    }

    pub fn check_tiles(&self, tiles: String) {

        let tiles_vec = self.string_to_vector(tiles);

        require!(tiles_vec.len() == SIZE as u64,
            "unexpected number of values (16 needed)");

        for i in 0..SIZE {
            let buff = tiles_vec.get(i as u64);

            require!(buff > Some(0) && buff <= Some(15),
                "unexpected number of values (0-15 needed)");

            for j in 0..SIZE {
                if i != j {
                    require!(buff != tiles_vec.get(j as u64),
                        "unexpected number of values (repetition of values)");
                }
            }
        }
    }

    pub fn is_solved(&self) -> bool {

        let game = self.expect_value_found(
            self.games.get(&env::predecessor_account_id()));

        if game.tiles.chars().last().unwrap() != '0' {
            return false;
        }

        let vector = self.string_to_vector(game.tiles);

        for i in (0..SIZE).rev() {
            if self.expect_value_found(
                vector.get(i as u64)) != (i + 1) as u8 {
                return false;
            }
        }

        true
    }

    pub fn expect_value_found<T>(&self, option: Option<T>) -> T {

        option.unwrap_or_else(|| env::panic_str("Not found"))
    }
}
