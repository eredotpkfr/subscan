use crate::utils::env::format_env;
use dotenvy;

/// Struct that stores any environment variable value with its name
#[derive(Debug)]
pub struct Env {
    /// Environment variable name as a string, it look like `FOO_APIKEY`
    pub name: String,
    /// Environment variable's value
    pub value: Option<String>,
}

impl From<String> for Env {
    fn from(value: String) -> Self {
        Self {
            name: value.to_uppercase(),
            value: dotenvy::var(value.to_uppercase()).ok(),
        }
    }
}

impl From<&str> for Env {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_uppercase(),
            value: dotenvy::var(value.to_uppercase()).ok(),
        }
    }
}

/// Store basic HTTP authentication credentials as a environment
#[derive(Debug)]
pub struct Credentials {
    /// Username as a [`Env`]
    pub username: Env,
    /// Password as a [`Env`]
    pub password: Env,
}

impl Credentials {
    /// Check both username and password environments set for a any module
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::env::{Env, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let credentials = Credentials {
    ///         username: Env { name: String::from("username"), value: None, },
    ///         password: Env { name: String::from("password"), value: None, },
    ///     };
    ///
    ///     assert!(!credentials.is_ok())
    /// }
    /// ```
    pub fn is_ok(&self) -> bool {
        self.username.value.is_some() && self.password.value.is_some()
    }
}

#[derive(Debug)]
pub struct SubscanModuleEnvs {
    /// Host value as a [`Env`], we stores the API host name as environment c/o
    /// some APIs gives different sub host names like `bcgad.api.foo` to user
    pub host: Env,
    /// Module API key as a [`Env`]
    pub apikey: Env,
    /// Basic HTTP authentication credentials as a [`Env`]. See [`Credentials`]
    pub credentials: Credentials,
}

impl From<&str> for SubscanModuleEnvs {
    fn from(name: &str) -> Self {
        Self {
            host: format_env(name, "host").into(),
            apikey: format_env(name, "apikey").into(),
            credentials: Credentials {
                username: format_env(name, "username").into(),
                password: format_env(name, "password").into(),
            },
        }
    }
}
