use nom::branch::alt;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::error::context;
use nom::multi::many1;

use crate::parser::parsers::{Elms, UResult};
use crate::parser::parsers::basic_parsers::*;

#[inline]
pub(crate) fn fragment(i: Elms) -> UResult<Elms, String> {
  context(
    "fragment",
    map(many1(alt((pchar, map(one_of("/?"), |c| c.into())))), |sl| {
      sl.into_iter().collect()
    }),
  )(i)
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};
  use crate::parser::parsers::basic_parsers::gens::*;

  pub fn fragment_str_gen() -> Gen<String> {
    rep_str_gen(1, u8::MAX - 1, || {
      Gens::choose_u8(1, 2).bind(|n| match n {
        1 => pchar_str_gen(1, 1),
        2 => Gens::one_of_vec(vec!['/', '?']).fmap(|c| c.into()),
        x => panic!("x = {}", x),
      })
    })
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
  fn test_fragment() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || fragment_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, fragment) = fragment(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(fragment, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
