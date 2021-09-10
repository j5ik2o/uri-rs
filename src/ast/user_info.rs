use std::borrow::Cow;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};

use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct UserInfo {
  user_name: Cow<'static, str>,
  password: Option<Cow<'static, str>>,
  str: Option<Cow<'static, str>>,
}

impl Hash for UserInfo {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.user_name.hash(state);
    self.password.hash(state);
  }
}

impl PartialEq for UserInfo {
  fn eq(&self, other: &Self) -> bool {
    self.user_name == other.user_name && self.password == other.password
  }
}

impl Clone for UserInfo {
  fn clone(&self) -> Self {
    UserInfo::new(self.user_name.clone(), self.password.clone())
  }
}

impl Default for UserInfo {
  fn default() -> Self {
    UserInfo::new("", Option::default())
  }
}

impl std::fmt::Display for UserInfo {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

impl<T> From<(T, Option<T>)> for UserInfo
where
  T: Into<Cow<'static, str>>,
{
  fn from((user_name, password): (T, Option<T>)) -> Self {
    Self::new(user_name, password)
  }
}

impl UserInfo {
  pub fn new<T: Into<Cow<'static, str>>>(user_name: T, password: Option<T>) -> Self {
    let mut s = Self {
      user_name: user_name.into(),
      password: password.map(|e| e.into()),
      str: None,
    };
    s.update_str();
    s
  }

  pub fn user_name(&self) -> &str {
    &self.user_name
  }

  pub fn password(&self) -> Option<&str> {
    self.password.as_deref()
  }

  fn update_str(&mut self) {
    let s = Self::create_string(&self.user_name, self.password.as_deref());
    self.str = Some(s.into());
  }

  pub fn as_str(&self) -> &str {
    self.str.as_deref().unwrap()
  }

  fn create_string(user_name: &str, password: Option<&str>) -> String {
    format!(
      "{}{}",
      user_name,
      password
        .iter()
        .map(|s| format!(":{}", s))
        .fold("".to_string(), |mut acc, s| {
          acc.push_str(&s);
          acc
        })
    )
  }
}
