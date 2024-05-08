use crate::enums::GenericModuleTypes;
use crate::interfaces::generic::GenericModule;
use crate::modules::generic::{GenericSearchEngineModule, GenericAPIIntegrationModule};

const RUN_METHOD_NOT_SPECIFIED: &str = "Any generic module type \
    or `run` method not set! Please specify \
    any module type (`GenericModuleTypes`) by \
    using `is_generic` method or implement \
    `run` method for module";

pub trait SubScanModule {
    fn is_generic(&self) -> Option<GenericModuleTypes>;
    fn name(&self) -> String;

    fn run(&self) {
        if let Some(generic) = self.is_generic() {
            let module: Box<dyn GenericModule> = match generic {
                GenericModuleTypes::GenericSearchEngineModule => {
                    Box::new(GenericSearchEngineModule::new(self.name()))
                }
                GenericModuleTypes::GenericAPIIntegrationModule => {
                    Box::new(GenericAPIIntegrationModule::new(self.name()))
                }
            };

            module.run()
        } else {
            panic!(
                "module: {}\nreason: {}",
                self.name(),
                RUN_METHOD_NOT_SPECIFIED
            );
        }
    }
}
