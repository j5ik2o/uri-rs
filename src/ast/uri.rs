use std::fmt::Formatter;

use crate::ast::authority::Authority;
use crate::ast::path::Path;
use crate::ast::query::Query;
use crate::ast::scheme::Scheme;
use crate::parser::parsers::{Elms, uri_parsers, UriParseError};

pub type Fragment = String;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Uri {
  schema: Scheme,
  authority: Option<Authority>,
  path: Path,
  query: Option<Query>,
  fragment: Option<String>,
}

impl Default for Uri {
  fn default() -> Self {
    Uri {
      schema: Scheme::default(),
      authority: Option::default(),
      path: Path::default(),
      query: Option::default(),
      fragment: Option::default()
    }
  }
}

impl std::fmt::Display for Uri {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}:{}{}{}{}",
      self.schema.to_string(),
      self
        .authority
        .as_ref()
        .map(|a| format!("//{}", a.to_string()))
        .unwrap_or("".to_string()),
      self.path.to_string(),
      self
        .query
        .as_ref()
        .map(|q| format!("?{}", q.to_string()))
        .unwrap_or("".to_string()),
      self
        .fragment
        .as_ref()
        .map(|s| format!("#{}", s))
        .unwrap_or("".to_string())
    )
  }
}

impl Uri {
  pub fn parse(text: &str) -> Result<Uri, nom::Err<UriParseError>> {
    uri_parsers::uri(Elms::new(text.as_bytes())).map(|(_, v)| v)
  }

  pub fn new(
    schema: Scheme,
    authority: Option<Authority>,
    path: Path,
    query: Option<Query>,
    fragment: Option<Fragment>,
  ) -> Self {
    Self {
      schema,
      authority,
      path,
      query,
      fragment,
    }
  }

  pub fn schema(&self) -> &Scheme {
    &self.schema
  }

  pub fn authority(&self) -> Option<&Authority> {
    self.authority.as_ref()
  }

  pub fn path(&self) -> &Path {
    &self.path
  }

  pub fn path_as_opt(&self) -> Option<&Path> {
    if self.path.is_empty() {
      None
    } else {
      Some(&self.path)
    }
  }

  pub fn query(&self) -> Option<&Query> {
    self.query.as_ref()
  }

  pub fn fragment(&self) -> Option<&Fragment> {
    self.fragment.as_ref()
  }
}

#[cfg(test)]
mod test {
  use std::env;

  use crate::{Scheme, Uri};

  use super::*;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_parse() {
    init();
    let s = "http://user1:pass1@localhost:8080/example?key1=value1&key2=value2&key1=value2#f1";
    match Uri::parse(s) {
      Ok(uri) => println!("{:?}", uri),
      Err(e) => println!("{:?}", e),
    }
  }
}
