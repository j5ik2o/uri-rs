use nom::{AsChar, InputTakeAtPosition, IResult};
use nom::combinator::map;
use nom::error::{context, ErrorKind, ParseError};

use crate::ast::scheme::Scheme;
use crate::parser::parsers::{Elms, UResult};

#[inline]
fn code_point<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input.split_at_position1_complete(
    |item| {
      let c = item.as_char();
      !(c == '+' || c == '-' || c == '.' || c.is_alphanumeric())
    },
    ErrorKind::Char,
  )
}

#[inline]
pub(crate) fn scheme(i: Elms) -> UResult<Elms, Scheme> {
  context("schema", map(code_point, |s: Elms| s.into()))(i)
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parser::parsers::basic_parsers::gens::*;

  pub fn scheme_gen() -> Gen<String> {
    rep_char_gen(5, || {
      Gens::choose_u8(1, 5).bind(|n| match n {
        1 => alpha_char_gen(),
        2 => digit_gen('0', '9'),
        3 => Gen::<char>::unit(|| '+'),
        4 => Gen::<char>::unit(|| '-'),
        5 => Gen::<char>::unit(|| '.'),
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

  use crate::parser::parsers::Elms;

  use super::*;
  use crate::parser::parsers::scheme_parsers::gens::scheme_gen;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_scheme() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || scheme_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = scheme(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r.to_string(), s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
