pub struct CastlingRights(pub u8);
impl CastlingRights {
    pub const WK: CastlingRights = CastlingRights(0b0001);
    pub const WQ: CastlingRights = CastlingRights(0b0010);
    pub const BK: CastlingRights = CastlingRights(0b0100);
    pub const BQ: CastlingRights = CastlingRights(0b1000);
}
