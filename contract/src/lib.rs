use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, BorshStorageKey, env, log, near_bindgen, Promise, require};
use near_sdk::collections::{LookupMap};
use near_sdk::serde::{Deserialize, Serialize};

const SIZE: usize = 16;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey{
    Players,
    Games,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde" )]
pub struct Player {
    price: u128,
    opponent: Option<AccountId>,
    is_play: bool,

}

impl Default for Player {
    fn default() -> Self {
        Self {
            price: 0,
            opponent: None,
            is_play: false,
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
    pub players_vec: Vec<AccountId>,

}

impl Default for Pazzle {
    fn default() -> Self {
        Pazzle{
            players: LookupMap::new(StorageKey::Players),
            games: LookupMap::new(StorageKey::Games),
            players_vec: Vec::new(),
        }
    }
}

#[near_bindgen]
impl Pazzle {
    pub fn add_me_to_players(&mut self) {

        require!(!self.players_vec.contains(&env::predecessor_account_id()),
                "you are already in the player list");

        self.players_vec.push(env::predecessor_account_id());
        self.players.insert(&env::predecessor_account_id(), &Player::default());
    }

    pub fn is_i_in_players(&self) -> bool {
        self.players_vec.contains(&env::predecessor_account_id())
    }

    pub fn get_players(&self) -> (Vec<AccountId>, Vec<Player>) {
        require!(self.players_vec.len() != 0,"there are no players");

        let mut players: Vec<Player> = Vec::new();
        let mut accounts: Vec<AccountId> = Vec::new();
        for account_id in &self.players_vec {
            accounts.push(account_id.clone());
            players.push(self.expect_value_found(
                self.players.get(&account_id)));
        }

        (accounts, players)
    }

    #[payable]
    pub fn set_price(&mut self) {
        if !self.players_vec.contains(&env::predecessor_account_id()) {
            self.add_me_to_players();
        }

        let mut player = self.expect_value_found(
            self.players.get(&env::predecessor_account_id()));

        require!(player.price == 0,"you have already placed a bet");
        player.price = env::attached_deposit();
        self.players.insert(&env::predecessor_account_id(), &player);
    }

    #[payable]
    pub fn withdraw_and_cancel_price(&mut self) {
        let mut player = self.expect_value_found(
            self.players.get(&env::predecessor_account_id()));
        let opponent = self.expect_value_found(
            self.players.get(&self.expect_value_found(player.opponent.clone())));

        require!(player.price > 0, "you don't have a bid");
        require!(!opponent.is_play,
            "must end the game, the opponent has already accepted your challenge");

        player.price = 0;
        self.players.insert(&env::predecessor_account_id(), &player);

        Promise::new(
            AccountId::new_unchecked(
                env::predecessor_account_id().to_string())).
            transfer(player.price);
    }

    pub fn set_opponent(&mut self, opponent_id: AccountId) {
        require!(
           env::is_valid_account_id(opponent_id.as_bytes()),
           "Account does not exist");
        require!(
           self.players_vec.contains(&opponent_id),
           "the opponent is not from the list of players");
        require!(
           self.players_vec.contains(&env::predecessor_account_id()),
           "you are not in the player list");

        let mut player = self.expect_value_found(
            self.players.get(&env::predecessor_account_id()));
        if player.opponent.is_some() {
            let old_opponent = self.expect_value_found(
                self.players.get(
                    &self.expect_value_found(player.opponent)));
            require!(!old_opponent.is_play,
                "your previous opponent has already accepted the game, end the game");
        }
        let opponent = self.expect_value_found(
            self.players.get(&opponent_id));
        require!(player.price == opponent.price, "rates must be the same");

        player.opponent = Some(opponent_id);
        player.is_play = false;
        self.players.insert(&env::predecessor_account_id(), &player);
    }

    pub fn get_opponent(&self, account_id: AccountId) -> Option<AccountId> {
        let player = self.expect_value_found(
            self.players.get(&account_id));

        player.opponent
    }

    pub fn is_play_player(&self, player_id: AccountId) -> bool {
        require!(
           env::is_valid_account_id(player_id.as_bytes()),
           "Account does not exist");
        require!(
           self.players_vec.contains(&player_id),
           "player not found");
        let player = self.expect_value_found(
            self.players.get(&player_id));

        player.is_play
    }


    pub fn new_game(&mut self, shuffle: [u8; SIZE]) {

        let mut player: Player = self.expect_value_found(
            self.players.get(&env::predecessor_account_id()));
        let opponent: Player = self.expect_value_found(
            self.players.get(&self.expect_value_found(player.opponent.clone())));
        require!(!player.is_play || !opponent.is_play,
                "finish the game");

        self.check_tiles(shuffle.clone());
        self.check_allocation(shuffle.clone());

        require!(self.is_solvable(shuffle.clone()),
                "the resulting permutation does not resolve");
        require!(player.opponent.is_some(),
                "select an opponent");

        let mut game: Game = Game::default();
        game.tiles = shuffle.clone();
        self.games.insert(&env::predecessor_account_id(), &game);

        player.is_play = true;
        self.players.insert(&env::predecessor_account_id(), &player);
    }

    pub fn run(&mut self, tiles: [u8; SIZE]) {

        let mut player: Player = self.expect_value_found(
            self.players.get(&env::predecessor_account_id()));
        let mut opponent: Player = self.expect_value_found(
            self.players.get(&self.expect_value_found(player.opponent.clone())));
        require!(player.is_play && opponent.is_play,
                "you or your opponent are not yet ready to play");

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
            game_tiles[x0 as usize] == 0 &&
            ((((i_x % 4 != 0 && i_x0 % 4 != 3)) && ((i_x0 - i_x) != 1 || (i_x0 - i_x) != -1 )) ||
                (i_x0 % 4 != 0 && i_x % 4 != 3) && ((i_x0 - i_x) != 1 || (i_x0 - i_x) != -1 )) ||
            ((i_x0 - i_x) == 4 || (i_x0 - i_x) == -4 ),
            "not a correct move");

        game.tiles = tiles;
        self.games.insert(&env::predecessor_account_id(), &game);
        log!("the move is successful");

        if self.is_solved() {
            let win_price = player.price + opponent.price;

            player.is_play = false;
            player.price = 0;
            let opponent_id = self.expect_value_found(player.opponent);
            player.opponent = None;

            opponent.is_play = false;
            opponent.price = 0;
            opponent.opponent = None;

            self.players.insert(&env::predecessor_account_id(), &player);
            self.players.insert(&opponent_id, &opponent);

            log!("You WIN!!!");
            Promise::new(
                AccountId::new_unchecked(
                    env::predecessor_account_id().to_string())).
                transfer(win_price);
        }
    }

    pub fn get_tiles(&self, account_id: AccountId) -> [u8; SIZE] {
        let game = self.expect_value_found(
            self.games.get(&account_id));

        game.tiles
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

        count_inversions % 2 == 0
    }

    #[private]
    pub fn check_allocation(&self, tiles: [u8; SIZE]) {
        for i in 0..SIZE-4 {
            require!(!(tiles[i] + 1 == tiles[i+1] &&
                tiles[i] + 2 == tiles[i+2] &&
                tiles[i] + 3 == tiles[i+3] &&
                tiles[i] + 4 == tiles[i+4]),
                "this sequence is like cheating");
        }
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

        for i in (0..SIZE-1).rev() {
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
