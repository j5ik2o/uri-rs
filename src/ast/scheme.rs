use std::fmt::Formatter;

use crate::parser::parsers::Elms;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Scheme(Cow<'static, str>);

impl Default for Scheme {
  fn default() -> Self {
    Scheme(Cow::default())
  }
}

impl std::fmt::Display for Scheme {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

// impl From<T> for Scheme where T: Into<Cow<'static, str>> {
//   fn from(src: T) -> Self {
//     Self(src.into())
//   }
// }
//
// impl From<&[u8]> for Scheme {
//   fn from(src: &[u8]) -> Self {
//     Self(String::from_utf8(src.to_vec()).unwrap())
//   }
// }
//
impl From<Elms<'_>> for Scheme {
  fn from(src: Elms) -> Self {
    Self::new(src.as_string().unwrap())
  }
}

impl Scheme {
  pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
    Self(value.into())
  }
}
