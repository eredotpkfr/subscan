use crate::requesters::{chrome::ChromeBrowser, client::HTTPClient};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(RequesterInterface)]
pub enum RequesterDispatcher {
    ChromeBrowser(ChromeBrowser),
    HTTPClient(HTTPClient),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum RequesterType {
    ChromeBrowser,
    HTTPClient,
}
