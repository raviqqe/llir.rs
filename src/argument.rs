#[derive(Clone, Debug)]
pub struct Argument {
    typ: String,
    name: Option<String>,
}

impl Argument {
    pub fn new<T: Into<Option<String>>>(typ: String, name: T) -> Argument {
        Argument { typ, name }
    }
}

impl std::fmt::Display for Argument {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.typ)?;

        if let Some(name) = self.name {
            write!(formatter, " {}", name)?;
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        assert_eq!(fmt!("{}", Argument::new("i8*", "ptr")), "foo");
    }
}
