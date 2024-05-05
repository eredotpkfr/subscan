use core::result::Result;
use regex::{Error, Regex};

pub fn generate_domain_regex(domain: String) -> Result<Regex, Error> {
    Regex::new(&format!(
        r"(?:[a-z0-9](?:[a-z0-9-]{{0,61}}[a-z0-9])?\.)+({domain})",
        domain = domain.replace(".", r"\.")
    ))
}
