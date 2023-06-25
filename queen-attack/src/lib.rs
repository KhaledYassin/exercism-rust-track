#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        let valid_range = 0..=7;

        if valid_range.contains(&rank) && valid_range.contains(&file) {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let position = &self.position;
        let other_position = &other.position;

        position.rank == other_position.rank
            || position.file == other_position.file
            || (position.file - other_position.file).abs()
                == (position.rank - other_position.rank).abs()
    }
}
