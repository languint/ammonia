pub struct NrOf;
impl NrOf {
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
    pub const SQUARES: usize = NrOf::FILES * NrOf::RANKS;

    pub const PIECES: usize = 6;
    pub const SIDES: usize = 2;
}
