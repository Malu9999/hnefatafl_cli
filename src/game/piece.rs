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
    /// returns true if the piece is a king
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King(_))
    }

    /// returns true if the piece is a defender
    pub fn is_defender(&self) -> bool {
        matches!(
            self,
            Self::Pawn(PieceColor::Defender) | Self::King(PieceColor::Defender)
        )
    }

    /// returns true if the piece is an attacker
    pub fn is_attacker(&self) -> bool {
        matches!(
            self,
            Self::Pawn(PieceColor::Attacker) | Self::King(PieceColor::Attacker)
        )
    }

    /// returns the color of the piece
    pub fn get_color(&self) -> PieceColor {
        match self {
            Self::King(color) => color.clone(),
            Self::Pawn(color) => color.clone(),
        }
    }

    /// returns true if the piece is of the given color
    pub fn is_color(&self, color: &PieceColor) -> bool {
        match color {
            PieceColor::Attacker => self.is_attacker(),
            PieceColor::Defender => self.is_defender(),
        }
    }

    /// returns true if the piece is of the same color
    #[allow(unused)]
    pub fn same_color(&self, other: &Piece) -> bool {
        (self.is_attacker() && other.is_attacker()) || (self.is_defender() && other.is_defender())
    }
}

impl PieceColor {
    /// flips the color
    pub fn flip(&mut self) {
        match self {
            Self::Attacker => *self = Self::Defender,
            Self::Defender => *self = Self::Attacker,
        }
    }

    /// returns the opposite color
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
