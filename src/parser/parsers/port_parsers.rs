use std::str::FromStr;

use nom::character::complete::digit1;
use nom::combinator::map;
use nom::error::context;

use crate::parser::parsers::{Elms, UResult};

#[inline]
pub(crate) fn port(i: Elms) -> UResult<Elms, u16> {
  context(
    "port",
    map(digit1, |e: Elms| {
      let s = e.as_str().unwrap();
      u16::from_str(s).unwrap()
    }),
  )(i)
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gens, Gen};

  pub fn port_gen() -> Gen<String> {
    Gens::choose_u16(1, u16::MAX - 1).fmap(move |n| n.to_string())
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use anyhow::Result;
  use prop_check_rs::prop;
  use prop_check_rs::prop::TestCases;
  use prop_check_rs::rng::RNG;

  use super::*;
  use super::gens::*;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_port() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || port_gen(),
      move |s| {
        counter += 1;
        log::debug!("{:>03}, port = {}", counter, s);
        let (_, port) = port(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(port.to_string(), s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
