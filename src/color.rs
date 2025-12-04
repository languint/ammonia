/// A side of the chess board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u8);
impl Color {
    pub const WHITE: Color = Color(0b01);
    pub const BLACK: Color = Color(0b10);
}

impl std::ops::Not for Color {
    type Output = Color;
    fn not(self) -> Self::Output {
        match self {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
            _ => unreachable!(),
        }
    }
}

impl Color {
    pub fn index(self) -> usize {
        match self {
            Color::WHITE => 0,
            Color::BLACK => 1,
            _ => {
                dbg!(self);
                panic!()
            }
        }
    }
}
