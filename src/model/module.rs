use getset::{Getters, Setters};
use std::fmt::{Display, Formatter, Result};

#[derive(Getters, Setters, Default, PartialEq, Debug)]
#[getset(get, set)]
pub struct Module {
    name: String,
    version: u8,
    namespace: String,
    revision: Revision,
    prefix: String,
    organization: Option<String>,
    description: Option<String>,
    import: Vec<Import>,
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.name, self.version)
    }
}

#[allow(dead_code)]
#[derive(Getters, Setters, Default, PartialEq, Debug)]
pub struct Import {
    prefix: String,
    revision: String,
}

#[allow(dead_code)]
#[derive(Getters, Setters, Default, PartialEq, Debug)]
pub struct Revision {
    name: String,
    description: Option<String>,
}
