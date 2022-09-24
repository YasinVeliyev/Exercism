// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let direction = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            d: direction,
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let direction = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            d: direction,
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = match self.d {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
        };
        Self { x, y, ..self }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        // for i in instructions.chars() {
        //     if i == 'L' {
        //         robot = robot.turn_left()
        //     } else if i == 'A' {
        //         robot = robot.advance()
        //     } else {
        //         robot = robot.turn_right()
        //     }
        // }
        // for i in instructions.chars() {
        //     robot = if i == 'L' {
        //         robot.turn_left()
        //     } else if i == 'A' {
        //         robot.advance()
        //     } else {
        //         robot.turn_right()
        //     }
        // }

        for i in instructions.chars() {
            robot = match i {
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                'R' => robot.turn_right(),
                _ => robot,
            };
        }
        robot

        // instructions
        //     .chars()
        //     .fold(self, |robot, instruction| match instruction {
        //         'L' => robot.turn_left(),
        //         'A' => robot.advance(),
        //         'R' => robot.turn_right(),
        //         _ => robot,
        //     })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
