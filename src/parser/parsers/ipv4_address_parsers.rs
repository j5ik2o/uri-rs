use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::one_of;
use nom::combinator::*;
use nom::error::context;
use nom::multi::count;
use nom::sequence::{terminated, tuple};

use crate::parser::parsers::{Elms, UResult};
use crate::parser::parsers::basic_parsers::*;

#[inline]
fn dec_octet1(i: Elms) -> UResult<Elms, String> {
  context("dec_octet1", map(digit, |c| c.into()))(i)
}

#[inline]
fn dec_octet2(i: Elms) -> UResult<Elms, String> {
  context(
    "dec_octet2",
    map(tuple((one_of("123456789"), digit)), |(c1, c2)| {
      [c1, c2].iter().collect()
    }),
  )(i)
}

#[inline]
fn dec_octet3(i: Elms) -> UResult<Elms, String> {
  context(
    "dec_octet3",
    map(
      tuple((complete::char('1'), digit, digit)),
      |(c1, c2, c3)| [c1, c2, c3].iter().collect(),
    ),
  )(i)
}

#[inline]
fn dec_octet4(i: Elms) -> UResult<Elms, String> {
  context(
    "dec_octet4",
    map(
      tuple((complete::char('2'), one_of("01234"), digit)),
      |(c1, c2, c3)| [c1, c2, c3].iter().collect(),
    ),
  )(i)
}

#[inline]
fn dec_octet5(i: Elms) -> UResult<Elms, String> {
  context(
    "dec_octet5",
    map(
      tuple((complete::char('2'), complete::char('5'), one_of("012345"))),
      |(c1, c2, c3)| [c1, c2, c3].iter().collect(),
    ),
  )(i)
}

#[inline]
pub(crate) fn dec_octet(i: Elms) -> UResult<Elms, String> {
  context(
    "dec_octet",
    alt((dec_octet5, dec_octet4, dec_octet3, dec_octet2, dec_octet1)),
  )(i)
}

#[inline]
pub fn ipv4_address(i: Elms) -> UResult<Elms, String> {
  context(
    "ipv4_address",
    map(
      tuple((
        count(terminated(dec_octet, complete::char('.')), 3),
        dec_octet,
      )),
      |(mut sl, s)| {
        sl.push(s);
        sl.join(".")
      },
    ),
  )(i)
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  pub fn dec_octet_str_gen() -> Gen<String> {
    Gens::choose_u32(1, 255).fmap(|n| n.to_string())
  }

  pub fn ipv4_address_str_gen() -> Gen<String> {
    Gens::list_of_n(4, || dec_octet_str_gen()).fmap(|sl| sl.join("."))
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use anyhow::Result;
  use prop_check_rs::prop;
  use prop_check_rs::prop::TestCases;
  use prop_check_rs::rng::RNG;

  use crate::parser::parsers::{Elms};

  use super::*;
  use super::gens::*;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_dec_octet() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || dec_octet_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{:>03}, dec_octet = {}", counter, s);
        let (_, r) = dec_octet(Elms::new(s.as_bytes())).ok().unwrap();
        r == s
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv4_address() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv4_address_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, ipv4_address = {}", counter, s);
        let (_, r) = ipv4_address(Elms::new(s.as_bytes())).ok().unwrap();
        r == s
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
