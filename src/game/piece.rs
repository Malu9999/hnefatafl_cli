#[derive(Clone, PartialEq)]
pub enum PieceColor {
    Attacker,
    Defender,
}

#[derive(Clone)]
pub enum Piece {
    Pawn(PieceColor),
    King(PieceColor),
}

impl Piece {
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King(_))
    }

    pub fn is_white(&self) -> bool {
        matches!(
            self,
            Self::Pawn(PieceColor::Defender) | Self::King(PieceColor::Defender)
        )
    }

    pub fn is_black(&self) -> bool {
        matches!(
            self,
            Self::Pawn(PieceColor::Attacker) | Self::King(PieceColor::Attacker)
        )
    }

    pub fn get_color(&self) -> PieceColor {
        match self {
            Self::King(color) => color.clone(),
            Self::Pawn(color) => color.clone(),
        }
    }

    pub fn is_color(&self, color: &PieceColor) -> bool {
        match color {
            PieceColor::Attacker => self.is_black(),
            PieceColor::Defender => self.is_white(),
        }
    }

    #[allow(unused)]
    pub fn same_color(&self, other: &Piece) -> bool {
        (self.is_black() && other.is_black()) || (self.is_white() && other.is_white())
    }
}

impl PieceColor {
    pub fn flip(&mut self) {
        match self {
            Self::Attacker => *self = Self::Defender,
            Self::Defender => *self = Self::Attacker,
        }
    }

    pub fn get_opposite(&self) -> PieceColor {
        match self {
            Self::Attacker => Self::Defender,
            Self::Defender => Self::Attacker,
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pawn(col) => match col {
                PieceColor::Attacker => write!(f, "\u{265F}"),
                PieceColor::Defender => write!(f, "\u{2659}"),
            },
            _ => write!(f, "\u{2654}"),
        }
    }
}
