use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

/// Core subdomain data type
pub type Subdomain = String;
/// Inner extract method type definition for [`JSONExtractor`](crate::extractors::json::JSONExtractor)
/// In summary it takes a [`Value`] as a parameter and parse subdomains
pub type InnerExtractMethod = Box<dyn Fn(Value) -> BTreeSet<Subdomain> + Sync + Send>;
/// Simple tuple type to store environment API key
/// variable with variable name
pub type APIKeyAsEnv = (String, Result<String, dotenvy::Error>);
/// Method definition type, [`GenericAPIIntegrationModule`](crate::modules::generics::api_integration::GenericAPIIntegrationModule)
/// uses this type to define method that gets query URL
pub type GetQueryUrlFunc = Box<dyn Fn(&str) -> String + Sync + Send>;
/// Method definition type, [`GenericAPIIntegrationModule`](crate::modules::generics::api_integration::GenericAPIIntegrationModule)
/// uses this type to define function that gets next query URL to fetch API fully
pub type GetNextUrlFunc = Box<dyn Fn(Url, Value) -> Option<Url> + Sync + Send>;
