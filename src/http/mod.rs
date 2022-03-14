//We are pulling these two inside the scope of this module
pub use method::Method;
pub use request::{Request,ParseError};
pub use query_string::QueryString;
pub mod query_string;
pub mod request;
pub mod method;
