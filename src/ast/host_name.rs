use std::fmt::Formatter;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HostName(Cow<'static, str>);

impl Default for HostName {
  fn default() -> Self {
    HostName(Cow::default())
  }
}

impl std::fmt::Display for HostName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<&'static str> for HostName {
  fn from(src: &'static str) -> Self {
    Self::new(src)
  }
}

impl From<String> for HostName {
  fn from(src: String) -> Self {
    Self::new(src)
  }
}

impl HostName {
  pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
    Self(value.into())
  }

  pub fn as_str(&self) -> &str {
    &self.0
  }
}
