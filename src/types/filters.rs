#[derive(Clone, Debug)]
pub struct ModuleNameFilter {
    pub valids: Vec<String>,
    pub invalids: Vec<String>,
}

impl From<(Vec<String>, Vec<String>)> for ModuleNameFilter {
    fn from(tuple: (Vec<String>, Vec<String>)) -> Self {
        Self {
            valids: tuple.0,
            invalids: tuple.1,
        }
    }
}
