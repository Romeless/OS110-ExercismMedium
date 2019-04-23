// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
	loc_x: i32,
	loc_y: i32,
	direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
			loc_x : x,
			loc_y : y,
			direction : d,
		}
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = match self.direction {
			Direction::North => Direction::East,
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North,
		};
		
		self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction = match self.direction {
			Direction::North => Direction::West,
			Direction::East => Direction::North,
			Direction::South => Direction::East,
			Direction::West => Direction::South,
		};
		
		self
    }

    pub fn advance(mut self) -> Self {
        self.loc_x += match self.direction {
			Direction::East => 1,
			Direction::West => -1,
			_ => 0,
		};
		
		self.loc_y += match self.direction {
			Direction::North => 1,
			Direction::South => -1,
			_ => 0,
		};
		
		self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for chars in instructions.chars() {
			self = match chars {
				'L' => self.turn_left(),
				'R' => self.turn_right(),
				'A' => self.advance(),
				_ => self,
			}
		}
		
		self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.loc_x, self.loc_y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
