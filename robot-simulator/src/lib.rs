// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
    delta: (i32, i32),
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        let delta = delta(&d);
        Robot {
            x,
            y,
            direction: d,
            delta,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let d = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self::new(self.x, self.y, d)
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let d = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self::new(self.x, self.y, d)
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let x = self.x + self.delta.0;
        let y = self.y + self.delta.1;

        Self::new(x, y, self.direction)
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {

        for instruction in instructions.to_ascii_uppercase().chars() {
            self = match instruction {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                'A' => self.advance(),
                _ => panic!("Invalid instruction"),
            };
        }

        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

fn delta(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::North => (0, 1),
        Direction::East => (1, 0),
        Direction::South => (0, -1),
        Direction::West => (-1, 0),
    }
}
