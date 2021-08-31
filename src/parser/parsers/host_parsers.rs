use nom::{AsChar, InputTakeAtPosition, IResult};
use nom::branch::alt;
use nom::character::complete;
use nom::combinator::map;
use nom::error::{context, ErrorKind, ParseError};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated, tuple};

use crate::ast::host_name::HostName;
use crate::parser::parsers::{Elms, ipv4_address_parsers, ipv6_address_parsers, UResult};
use crate::parser::parsers::basic_parsers::*;

#[inline]
fn hd_code_point<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input.split_at_position1_complete(
    |item| {
      let c = item.as_char();
      !is_hex_digit(c)
    },
    ErrorKind::HexDigit,
  )
}

#[inline]
fn p_code_point<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input.split_at_position1_complete(
    |item| {
      let c = item.as_char();
      !(is_unreserved(c) || is_sub_delims(c) || c == ':')
    },
    ErrorKind::Char,
  )
}

// "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )
#[inline]
pub(crate) fn ipv_future(i: Elms) -> UResult<Elms, String> {
  map(
    tuple((
      preceded(
        complete::char('v'),
        terminated(hd_code_point, complete::char('.')),
      ),
      p_code_point,
    )),
    |(k, m): (Elms, Elms)| {
      let ks = k.as_str().unwrap();
      let ms = m.as_str().unwrap();
      format!("v{}.{}", ks, ms)
    },
  )(i)
}

#[inline]
pub(crate) fn reg_name(i: Elms) -> UResult<Elms, String> {
  context(
    "reg_name",
    map(
      many0(alt((
        map(unreserved, |c| c.into()),
        pct_encoded,
        map(sub_delims, |c| c.into()),
      ))),
      |sl| sl.into_iter().collect(),
    ),
  )(i)
}

#[inline]
pub(crate) fn ip_literal(i: Elms) -> UResult<Elms, String> {
  context(
    "ip_literal",
    map(
      delimited(
        complete::char('['),
        alt((ipv_future, ipv6_address_parsers::ipv6_address)),
        complete::char(']'),
      ),
      |s| format!("[{}]", s),
    ),
  )(i)
}

#[inline]
pub fn host_name(i: Elms) -> UResult<Elms, HostName> {
  map(
    context(
      "host",
      alt((ip_literal, ipv4_address_parsers::ipv4_address, reg_name)),
    ),
    |s| HostName::new(s),
  )(i)
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parser::parsers::basic_parsers::gens::*;
  use crate::parser::parsers::ipv4_address_parsers::gens::*;
  use crate::parser::parsers::ipv6_address_parsers::gens::*;

  pub fn reg_name_str_gen() -> Gen<String> {
    rep_str_gen(1, 10, || {
      Gens::choose_u8(1, 3).bind(|n| match n {
        1 => unreserved_char_gen().fmap(|c| c.into()),
        2 => sub_delims_char_gen().fmap(|c| c.into()),
        3 => pct_encoded_str_gen(),
        x => panic!("x = {}", x),
      })
    })
  }

  pub fn ipv_future_str_gen() -> Gen<String> {
    let a = || rep_char_gen(5, || hex_digit_char_gen());
    let b = || {
      rep_char_gen(5, || {
        Gens::choose_u8(1, 3).bind(|n| match n {
          1 => unreserved_char_gen(),
          2 => sub_delims_char_gen(),
          3 => Gen::<char>::unit(|| ':'),
          x => panic!("x = {}", x),
        })
      })
    };
    a().bind(move |s1| b().fmap(move |s2| format!("v{}.{}", s1, s2)))
  }

  pub fn ip_literal_str_gen() -> Gen<String> {
    Gens::choose_u8(1, 2)
      .bind(|n| match n {
        1 => ipv6_address_str_gen(),
        2 => ipv_future_str_gen(),
        x => panic!("x = {}", x),
      })
      .fmap(|s| format!("[{}]", s))
  }

  pub fn host_gen() -> Gen<String> {
    Gens::choose_u8(1, 3).bind(|n| match n {
      1 => ip_literal_str_gen(),
      2 => ipv4_address_str_gen(),
      3 => reg_name_str_gen(),
      x => panic!("x = {}", x),
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
  fn test_ip_literal() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ip_literal_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = ip_literal(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv_future() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv_future_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = ipv_future(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_reg_name() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || reg_name_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        reg_name(Elms::new(s.as_bytes())).is_ok()
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_host() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || host_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, host_name) = host_name(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(host_name.to_string(), s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
