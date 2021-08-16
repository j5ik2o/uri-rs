//! # Usage
//!
//! ```rust
//! use uri_rs::Uri;
//! let s = "http://user1:pass1@localhost:8080/example?key1=value1&key2=value2&key1=value2#f1";
//! let uri = Uri::parse(s).unwrap();
//! println!("{:?}", uri);
//! // Uri {
//! //    schema: Scheme("http"),
//! //    authority: Some(
//! //      Authority {
//! //        host_name: HostName("localhost"),
//! //        port: Some(8080),
//! //        user_info: Some(
//! //          UserInfo {
//! //            user_name: "user1",
//! //            password: Some("pass1")
//! //          }
//! //        )
//! //      }
//! //    ),
//! //    path: AbemptyPath {
//! //      type_name: "abempty_path",
//! //      parts: ["example"]
//! //    },
//! //    query: Some(
//! //      Query {
//! //        params: [
//! //          ("key1", Some("value1")),
//! //          ("key2", Some("value2")),
//! //          ("key1", Some("value2"))
//! //        ]
//! //      }
//! //    ),
//! //    fragment: Some("f1")
//! // }
//! println!("{}", uri.to_string());
//! // http://user1:pass1@localhost:8080/example?key1=value1&key2=value2&key1=value2#f1
//! ```
pub use ast::authority::*;
pub use ast::path::*;
pub use ast::query::*;
pub use ast::scheme::*;
pub use ast::user_info::*;
pub use ast::uri::*;
pub use ast::*;

mod ast;
pub mod parser;
#[cfg(feature = "serde")]
mod serde;
