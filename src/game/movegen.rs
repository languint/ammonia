use crate::{board::Board, game::game_move::Move};

pub struct Movegen {
    pub board: Board,
}
impl Movegen {
    pub fn new(board: Board) -> Movegen {
        Movegen { board }
    }
}
