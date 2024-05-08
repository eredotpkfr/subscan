use crate::interfaces::generic::GenericModule;

pub struct GenericSearchEngineModule {
    name: String,
}

pub struct GenericAPIIntegrationModule {
    name: String,
}

impl GenericSearchEngineModule {
    pub fn new(name: String) -> Self {
        GenericSearchEngineModule { name }
    }
}

impl GenericModule for GenericSearchEngineModule {
    fn run(&self) {
        println!("Started generic search engine! (name: {})", self.name);
    }
}

impl GenericAPIIntegrationModule {
    pub fn new(name: String) -> Self {
        GenericAPIIntegrationModule { name }
    }
}

impl GenericModule for GenericAPIIntegrationModule {
    fn run(&self) {
        println!("Started generic API integration! (name: {})", self.name);
    }
}
