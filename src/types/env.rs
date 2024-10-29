use crate::utilities::env::format_env;
use dotenvy;

/// Struct that stores any environment variable value with its name
#[derive(Clone, Debug, Default, PartialEq)]
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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Credentials {
    /// Username as a [`Env`]
    pub username: Env,
    /// Password as a [`Env`]
    pub password: Env,
}

impl From<&str> for Credentials {
    /// Fetch credentials from environment variables by module name. See the [`format_env`]
    /// for details
    fn from(name: &str) -> Self {
        Self {
            username: format_env(name, "username").into(),
            password: format_env(name, "password").into(),
        }
    }
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

/// This struct implemented to store module environments in a single storage
///
/// Always extensible according to needs and you can fetch all environment variables
/// with their names in a single line with [`From`] trait by using module name
///
/// See the [`format_env`] to environment variables name formats. Formatting allows to make
/// isolation between different module environments
///
/// # Examples
///
/// ```
/// use subscan::types::env::SubscanModuleEnvs;
///
/// #[tokio::main]
/// async fn main() {
///     let envs = SubscanModuleEnvs::from("foo");
///
///     assert_eq!(envs.apikey.name, "SUBSCAN_FOO_APIKEY");
///     assert_eq!(envs.apikey.value, None);
/// }
/// ```
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
            credentials: name.into(),
        }
    }
}
