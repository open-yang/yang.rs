use getset::{Getters, Setters};

#[derive(Getters, Setters, Default)]
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

#[derive(Getters, Setters, Default)]
pub struct Import {
    prefix: String,
    revision: String,
}

#[derive(Getters, Setters, Default)]
pub struct Revision {
    name: String,
    description: Option<String>,
}
