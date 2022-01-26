use getset::{Getters, Setters};

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Getters, Setters, Default)]
pub struct Import {
    prefix: String,
    revision: String,
}

#[allow(dead_code)]
#[derive(Getters, Setters, Default)]
pub struct Revision {
    name: String,
    description: Option<String>,
}
