use crate::requesters::{chrome::ChromeBrowser, client::HTTPClient};
use enum_dispatch::enum_dispatch;
use strum_macros::EnumIter;

/// Dispatcher enumeration to decide requester types.
/// It allows to made static type dispatching instead of
/// dynamic dispatch and speed up performance. For more
/// technical details please follow up `enum_dispatch` package
#[enum_dispatch(RequesterInterface)]
pub enum RequesterDispatcher {
    /// Chrome browser struct definition as a enum value.
    /// On this requester type, Chrome browser will run and
    /// all HTTP requests made with browser. Has pros according
    /// to [`HTTPClient`] requester like running Js, rendering
    /// pages, etc.
    ChromeBrowser(ChromeBrowser),
    /// Simple HTTP client interface to make requesters, it does
    /// not allows to run Js, rendering pages or user interface.
    /// Just send HTTP requests via [`reqwest`]
    HTTPClient(HTTPClient),
}
/// Enumeration for HTTP requester types
#[derive(Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum RequesterType {
    /// Chrome browser requester type, see [`ChromeBrowser`]
    /// struct to understand what does it
    ChromeBrowser,
    /// HTTP client requester type, see [`HTTPClient`]
    /// struct to understand what does it
    HTTPClient,
}
