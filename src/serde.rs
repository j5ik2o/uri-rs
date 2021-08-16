use serde::{Serialize, Serializer, Deserialize, Deserializer};
use crate::Uri;

/// Default Serializer
impl Serialize for Uri {
  fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
  where
    S: Serializer,
  {
    let text = self.to_string();
    text.serialize(serializer)
  }
}

/// Default Deserializer
impl<'de> Deserialize<'de> for Uri {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let deserialized_str = String::deserialize(deserializer)?;
    Uri::parse(&deserialized_str).map_err(serde::de::Error::custom)
  }
}
