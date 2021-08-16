use std::fmt::Formatter;

use crate::parser::parsers::Elms;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Scheme(String);

impl std::fmt::Display for Scheme {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<String> for Scheme {
  fn from(src: String) -> Self {
    Self(src)
  }
}

impl From<&str> for Scheme {
  fn from(src: &str) -> Self {
    Self(src.to_string())
  }
}

impl From<&[u8]> for Scheme {
  fn from(src: &[u8]) -> Self {
    Self(String::from_utf8(src.to_vec()).unwrap())
  }
}

impl From<Elms<'_>> for Scheme {
  fn from(src: Elms) -> Self {
    Self(src.as_string().unwrap())
  }
}

impl Scheme {
  pub fn new(value: String) -> Self {
    Self(value)
  }
}
