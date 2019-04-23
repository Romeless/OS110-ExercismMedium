// https://exercism.io/tracks/rust/exercises/queen-attack

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
		if 0 <= rank && rank <= 7 && 0 <= file && file <= 7 {
			Some(ChessPosition {
				rank: rank, 
				file: file,
			})
		} else {
			None
		}
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.rank == other.position.rank { true }
		else if self.position.file == other.position.file { true }
		else if (self.position.rank - other.position.rank).abs() == 
			(self.position.file - other.position.file).abs() { true }
		else { false }
    }
}
