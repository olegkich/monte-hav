#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum HexOwner {
    P1,
    P2,
    None
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    P1,
    P2
}

impl From<&Player> for HexOwner {
    fn from(player: &Player) -> Self {
        match player {
            Player::P1 => HexOwner::P1,
            Player::P2 => HexOwner::P2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
    pub owner: HexOwner 
}

