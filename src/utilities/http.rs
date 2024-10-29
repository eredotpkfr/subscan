use reqwest::Url;

/// Update query params without remove old query params. If the
/// given parameter name non-exists it will append end of the
/// query otherwise it's value will be updated
///
/// # Examples
///
/// ```
/// use subscan::utilities::http::update_url_query;
/// use reqwest::Url;
///
/// let mut url: Url = "https://foo.com".parse().unwrap();
///
/// update_url_query(&mut url, "a".into(), "b".into());
/// assert_eq!(url.to_string(), "https://foo.com/?a=b");
///
/// // does not override old `a` parameter
/// update_url_query(&mut url, "x".into(), "y".into());
/// assert_eq!(url.to_string(), "https://foo.com/?a=b&x=y");
///
/// update_url_query(&mut url, "a".into(), "c".into());
/// assert_eq!(url.to_string(), "https://foo.com/?x=y&a=c");
/// ```
pub fn update_url_query(url: &mut Url, name: &str, value: &str) {
    let binding = url.clone();
    let pairs = binding.query_pairs();
    let old = pairs.filter(|item| item.0.to_lowercase() != name.to_lowercase());

    url.query_pairs_mut()
        .clear()
        .extend_pairs(old)
        .append_pair(name, value)
        .finish();
}
