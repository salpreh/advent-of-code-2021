pub enum Command {
    FORWARD(i32),
    DOWN(i32),
    UP(i32)
}

pub struct Submarine {
    position: i32,
    depth: i32
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine { position: 0, depth: 0 }
    }

    pub fn getPosition(&self) -> i32 {
        self.position
    }

    pub fn getDepth(&self) -> i32 {
        self.depth
    }

    pub fn process_command(&mut self, command: Command) {
        match command {
            Command::DOWN(y) => self.depth += y,
            Command::UP(y) => self.depth -= y,
            Command::FORWARD(x) => self.position += x
        };
    }
}

pub struct AdvancedSubmarine {
    position: i32,
    depth: i32,
    aim: i32
}

impl AdvancedSubmarine {
    pub fn new() -> AdvancedSubmarine {
        AdvancedSubmarine { position: 0, depth: 0, aim: 0 }
    }

    pub fn getPosition(&self) -> i32 {
        self.position
    }

    pub fn getDepth(&self) -> i32 {
        self.depth
    }

    pub fn process_command(&mut self, command: Command) {
        match command {
            Command::DOWN(a) => self.aim += a,
            Command::UP(a) => self.aim -= a,
            Command::FORWARD(x) => {
                self.position += x;
                self.depth += self.aim * x;
            }
        };
    }
}