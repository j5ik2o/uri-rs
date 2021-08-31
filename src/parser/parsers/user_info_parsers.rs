use nom::branch::alt;
use nom::character::complete;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::many1;
use nom::sequence::{preceded, tuple};

use crate::ast::user_info::UserInfo;
use crate::parser::parsers::{Elms, UResult};
use crate::parser::parsers::basic_parsers::{pct_encoded, sub_delims, unreserved};

#[inline]
fn code_point(i: Elms) -> UResult<Elms, String> {
  map(
    many1(alt((
      map(unreserved, |c| c.into()),
      pct_encoded,
      map(sub_delims, |c| c.into()),
    ))),
    |sl| sl.into_iter().collect(),
  )(i)
}

// *( unreserved / pct-encoded / sub-delims / ":" )
#[inline]
pub(crate) fn user_info(i: Elms) -> UResult<Elms, UserInfo> {
  context(
    "user_info",
    map(
      tuple((code_point, opt(preceded(complete::char(':'), code_point)))),
      |(user_name, password)| (user_name.as_str(), password.as_ref().map(|s| s.as_str())).into(),
    ),
  )(i)
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parser::parsers::basic_parsers::gens::*;

  pub fn user_info_gen() -> Gen<String> {
    let gen = || {
      rep_str_gen(1, 5, || {
        Gens::choose_u8(1, 3).bind(|n| match n {
          1 => unreserved_str_gen(1),
          2 => pct_encoded_str_gen(),
          3 => sub_delims_str_gen(1),
          x => panic!("x = {}", x),
        })
      })
    };
    Gens::one_bool().bind(move |b| {
      if b {
        gen().bind(move |s1| gen().fmap(move |s2| format!("{}:{}", s1, s2)))
      } else {
        gen().fmap(|s| format!("{}", s))
      }
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
  fn test_user_info() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || user_info_gen(),
      move |s| {
        counter += 1;
        log::debug!("{:>03}, value = {}", counter, s);
        let (_, r) = user_info(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r.to_string(), s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
