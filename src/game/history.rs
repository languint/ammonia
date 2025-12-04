use crate::game::game_move::Move;

pub const STACK_SIZE: usize = 1024;

pub struct History {
    stack: [Move; STACK_SIZE],
    loc: usize,
}
impl History {
    pub fn new() -> History {
        History {
            stack: [Move(0); STACK_SIZE],
            loc: 0,
        }
    }
}

impl History {
    /// Push a [`Move`] to the stack
    ///
    /// # Panics
    /// Safe as long as the move does not overflow the stack
    pub fn push(&mut self, mv: Move) {
        debug_assert!(self.loc < STACK_SIZE, "Stack overflow in History::push");

        self.stack[self.loc] = mv;
        self.loc += 1;
    }

    /// Pop a [`Move`] off the stack
    ///
    /// # Panics
    /// Safe as long as the pop does not underflow the stack
    #[inline]
    pub fn pop(&mut self) -> Move {
        debug_assert!(self.loc > 0, "Stack underflow in History::pop");
        self.loc -= 1;
        self.stack[self.loc]
    }

    /// Pop a [`Move`] off the stack
    ///
    /// # Panics
    /// Safe as long as the peek does not underflow the stack
    #[inline]
    pub fn peek(&self) -> Move {
        debug_assert!(self.loc > 0, "Stack underflow in History::peek");
        self.stack[self.loc - 1]
    }
}
