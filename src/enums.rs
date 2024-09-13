use crate::requesters::{chrome::ChromeBrowser, client::HTTPClient};
use enum_dispatch::enum_dispatch;
use strum_macros::EnumIter;

#[enum_dispatch(RequesterInterface)]
pub enum RequesterDispatcher {
    ChromeBrowser(ChromeBrowser),
    HTTPClient(HTTPClient),
}

#[derive(Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum RequesterType {
    ChromeBrowser,
    HTTPClient,
}
