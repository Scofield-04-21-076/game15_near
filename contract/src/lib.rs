use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, near_bindgen, require};
use near_sdk::collections::LookupMap;
use near_sdk::store::Vector;


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

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde" )]
pub struct Game {
    pub vector: Vector<u8>,

}

impl Default for Game {
    fn default() -> Self {
        Self {
            vector: Vector::new(StorageKey::VectorNumber),

        }
    }
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
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
    #[init]
    #[private]
    pub fn new(shuffle: Vector<u8>) -> Self {
        let mut res: Pazzle = Self::default();

        require!(shuffle.len() == 16,
            "unexpected number of values (16 needed)");

        for i in 1..= SIZE {
            if i == SIZE {
                res.vector.push(0);
                break;
            }
            res.vector.push(i);
        }

        res
    }

    pub fn new_game(&mut self, shuffle: Vector<u8>) {

        require!(shuffle.len() == SIZE,
            "unexpected number of values (16 needed)");

        for i in 0..SIZE {
            let buff = shuffle.get(i as u32);
            require!(buff > 0 && buff < 15,
                "unexpected number of values (0-15 needed)");
            for j in 0..SIZE {
                if i != j {
                    require!(buff != shuffle.get(j as u32),
                        "unexpected number of values (repetition of values)");
                }
            }
        }
        let mut game: Game = Game::default();
        game.vector = shuffle;
        self.games.insert(&env::predecessor_account_id(), &game);

        let mut player: Player = Player::default();
        player.is_finish_game = false;
        self.players.insert(&env::predecessor_account_id(), &player);



    }




}
