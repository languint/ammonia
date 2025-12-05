use crate::{board::Board, game::history::History};

pub mod defs;
pub mod game_move;
pub mod history;

pub struct Game {
    pub board: Board,
    pub history: History,
}
