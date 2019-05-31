use super::Argument;

#[derive(Clone, Debug)]
pub struct Definition {
    arguments: Vec<Argument>,
}

impl Definition {
    pub fn new(arguments: Vec<Argument>) -> Definition {
        Definition { arguments }
    }
}

impl std::fmt::Display for Definition {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}
