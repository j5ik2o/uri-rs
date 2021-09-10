use std::fmt;
use std::fmt::Formatter;

use itertools::Itertools;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Path {
  RootlessPath {
    type_name: &'static str,
    parts: Vec<String>,
    str: String,
  },
  AbemptyPath {
    type_name: &'static str,
    parts: Vec<String>,
    str: String,
  },
  AbsolutePath {
    type_name: &'static str,
    parts: Vec<String>,
    str: String,
  },
  NoSchemePath {
    type_name: &'static str,
    parts: Vec<String>,
    str: String,
  },
  EmptyPath {
    type_name: &'static str,
  },
}

impl Default for Path {
  fn default() -> Self {
    Path::of_empty()
  }
}

impl fmt::Display for Path {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let root = match self {
      Path::RootlessPath { .. } | Path::NoSchemePath { .. } | Path::EmptyPath { .. } => "",
      _ => "/",
    };
    let s = format!("{}{}", root, self.parts().join("/"));
    write!(f, "{}", s)
  }
}

impl Path {
  pub fn of_rootless_from_strs(parts: impl Iterator<Item = impl Into<Cow<'static, str>>>) -> Self {
    let type_name: &'static str = "rootless_path";
    let part_strings = parts
      .into_iter()
      .map(|e| e.into().to_string())
      .collect_vec();
    Path::RootlessPath {
      type_name,
      parts: part_strings.clone(),
      str: Self::create_string("", part_strings),
    }
  }

  pub fn of_abempty_from_strs(parts: impl Iterator<Item = impl Into<Cow<'static, str>>>) -> Self {
    let type_name: &'static str = "abempty_path";
    let part_strings = parts
      .into_iter()
      .map(|e| e.into().to_string())
      .collect_vec();
    Path::AbemptyPath {
      type_name,
      parts: part_strings.clone(),
      str: Self::create_string("/", part_strings),
    }
  }

  pub fn of_absolute_from_strs(parts: impl Iterator<Item = impl Into<Cow<'static, str>>>) -> Self {
    let type_name: &'static str = "absolute_path";
    let part_strings = parts
      .into_iter()
      .map(|e| e.into().to_string())
      .collect_vec();
    Path::AbsolutePath {
      type_name,
      parts: part_strings.clone(),
      str: Self::create_string("/", part_strings),
    }
  }

  pub fn of_no_scheme_from_strs(parts: impl Iterator<Item = impl Into<Cow<'static, str>>>) -> Self {
    let type_name: &'static str = "no_scheme_path";
    let part_strings = parts
      .into_iter()
      .map(|e| e.into().to_string())
      .collect_vec();
    Path::NoSchemePath {
      type_name,
      parts: part_strings.clone(),
      str: Self::create_string("", part_strings),
    }
  }

  pub fn of_empty() -> Self {
    Path::EmptyPath {
      type_name: "empty_path",
    }
  }

  pub fn type_name(&self) -> &'static str {
    match self {
      &Path::RootlessPath { type_name, .. } => type_name,
      &Path::AbemptyPath { type_name, .. } => type_name,
      &Path::AbsolutePath { type_name, .. } => type_name,
      &Path::NoSchemePath { type_name, .. } => type_name,
      &Path::EmptyPath { type_name } => type_name,
    }
  }
  pub fn parts(&self) -> &Vec<String> {
    static EMPTY_PARTS: Vec<String> = vec![];
    match self {
      Path::RootlessPath { parts, .. } => parts,
      Path::AbemptyPath { parts, .. } => parts,
      Path::AbsolutePath { parts, .. } => parts,
      Path::NoSchemePath { parts, .. } => parts,
      Path::EmptyPath { .. } => &EMPTY_PARTS,
    }
  }

  pub fn is_empty(&self) -> bool {
    self.parts().is_empty()
  }

  pub fn non_empty(&self) -> bool {
    !self.is_empty()
  }

  pub fn with_parts(&mut self, parts: Vec<String>) {
    self.add_parts(parts)
  }

  pub fn to_rootless(&self) -> Path {
    Path::of_rootless_from_strs(self.parts().clone().into_iter())
  }

  pub fn to_absolute(&self) -> Path {
    Path::of_absolute_from_strs(self.parts().clone().into_iter())
  }

  pub fn add_part(&mut self, part: String) {
    let parts_opt = match self {
      Path::RootlessPath { parts, .. } => Some(parts),
      Path::AbemptyPath { parts, .. } => Some(parts),
      Path::AbsolutePath { parts, .. } => Some(parts),
      Path::NoSchemePath { parts, .. } => Some(parts),
      Path::EmptyPath { .. } => None,
    };
    match parts_opt {
      Some(parts) => {
        parts.push(part);
      }
      None => (),
    }
  }

  pub fn add_parts(&mut self, parts: Vec<String>) {
    for x in parts {
      self.add_part(x)
    }
  }

  pub fn as_str(&self) -> &str {
    match self {
      Path::RootlessPath { str, .. } => str,
      Path::AbemptyPath { str, .. } => str,
      Path::AbsolutePath { str, .. } => str,
      Path::NoSchemePath { str, .. } => str,
      Path::EmptyPath { .. } => "",
    }
  }

  fn create_string(root: &str, parts: impl IntoIterator<Item = String>) -> String {
    format!("{}{}", root, parts.into_iter().join("/"))
  }
}
