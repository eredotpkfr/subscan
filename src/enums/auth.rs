use crate::types::env::Credentials;

/// Authentication methods for API calls or HTTP requests. [`GenericIntegrationModule`](crate::modules::generics::integration::GenericIntegrationModule)
/// uses them to apply correct auth method. See the method descriptions to learn how it works
#[derive(PartialEq)]
pub enum AuthenticationMethod {
    /// Some APIs uses request headers to get API key. If this auth type selected API key
    /// will add in request headers with a given header key
    APIKeyAsHeader(String),
    /// This auth type uses when API require API key as a query param. If this method chose
    /// API key will be added in URL as a query param with given parameter key
    APIKeyAsQueryParam(String),
    /// Use basic HTTP authentication method. If the credentials are not provided, module
    /// tries to fetch from environment variables using pre-formatted
    /// (see [`format_env`](crate::utils::env::format_env)) variable names. Module specific
    /// variable names looks like `SUBSCAN_FOO_USERNAME`, `SUBSCAN_FOO_PASSWORD`
    BasicHTTPAuthentication(Credentials),
    /// This auth type does nothing for auth
    NoAuthentication,
}

impl From<Credentials> for AuthenticationMethod {
    fn from(credentials: Credentials) -> Self {
        Self::BasicHTTPAuthentication(credentials)
    }
}

impl AuthenticationMethod {
    /// Checks the any auth method selector or not
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::enums::auth::AuthenticationMethod;
    ///
    /// let as_header = AuthenticationMethod::APIKeyAsHeader("X-API-Key".to_string());
    /// let no_auth = AuthenticationMethod::NoAuthentication;
    ///
    /// assert!(as_header.is_set());
    /// assert!(!no_auth.is_set());
    /// ```
    pub fn is_set(&self) -> bool {
        self != &Self::NoAuthentication
    }
}
