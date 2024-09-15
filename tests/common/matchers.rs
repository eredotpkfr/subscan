use wiremock::{http::HeaderName, Match, Request};

/// Custom `wiremock` header matcher
///
/// Basically incoming request has not given header key
/// panics otherwise returns 200 OK response  
pub struct HeaderExactMatcherWithPanic(pub HeaderName);

impl Match for HeaderExactMatcherWithPanic {
    fn matches(&self, request: &Request) -> bool {
        if request.headers.contains_key(&self.0) {
            true
        } else {
            panic!("Header {} must be set", self.0.to_string())
        }
    }
}

/// Alias for [`HeaderExactMatcherWithPanic`]
pub fn header_with_panic(header_key: &str) -> HeaderExactMatcherWithPanic {
    HeaderExactMatcherWithPanic(header_key.try_into().unwrap())
}
