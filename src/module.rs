#[derive(Clone, Debug)]
pub struct Module {
    declarations: Vec<Declaration>,
    definitions: Vec<Definition>,
}

impl Module {
    pub fn new(declarations: Vec<Declaration>) -> Module {
        Module {
            declarations,
            definitions,
        }
    }
}

impl std::fmt::Display for Module {}
