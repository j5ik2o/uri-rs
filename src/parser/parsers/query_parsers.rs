use nom::branch::alt;
use nom::character::complete;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::{preceded, tuple};

use crate::ast::query::Query;
use crate::parser::parsers::{Elms, UResult};
use crate::parser::parsers::basic_parsers::pchar_without_eq_and;

#[inline]
fn code_point(i: Elms) -> UResult<Elms, String> {
  map(
    many0(alt((
      pchar_without_eq_and,
      map(complete::char('/'), |c| c.into()),
      map(complete::char('?'), |c| c.into()),
    ))),
    |s| s.into_iter().collect(),
  )(i)
}

// query = *( pchar / "/" / "?" )
#[inline]
pub(crate) fn query(i: Elms) -> UResult<Elms, Query> {
  let key_values = || tuple((code_point, opt(preceded(complete::char('='), code_point))));
  context(
    "query",
    map(
      tuple((
        key_values(),
        many0(preceded(complete::char('&'), key_values())),
      )),
      |(head, tail)| {
        let mut m = vec![head];
        m.extend(tail);
        Query::new(m)
      },
    ),
  )(i)
}

#[cfg(test)]
pub mod gens {
  use itertools::Itertools;
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parser::parsers::basic_parsers::gens::*;

  fn sub_delims_without_char_gen() -> Gen<char> {
    Gens::one_of_vec(vec!['!', '$', '\'', '(', ')', '*', '+', ',', ';'])
  }

  fn sub_delims_without_str_gen(len: u8) -> Gen<String> {
    rep_char_gen(len, || sub_delims_without_char_gen())
  }

  pub fn pchar_without_eq_and_str_gen(min: u8, max: u8) -> Gen<String> {
    rep_str_gen(min, max, || {
      Gens::choose_u8(1, 4).bind(|n| match n {
        1 => unreserved_char_gen().fmap(|c| c.into()),
        2 => pct_encoded_str_gen(),
        3 => sub_delims_without_char_gen().fmap(|c| c.into()),
        4 => Gens::one_of_vec(vec![':', '@']).fmap(|c| c.into()),
        x => panic!("x = {}", x),
      })
    })
  }

  pub fn query_gen() -> Gen<String> {
    Gens::list_of_n(3, move || {
      pchar_without_eq_and_str_gen(1, 10).bind(|key| {
        Gens::list_of_n(2, || pchar_without_eq_and_str_gen(1, 10)).fmap(move |vl| {
          let kvl = vl
            .into_iter()
            .map(|v| format!("{}={}", key, v))
            .collect_vec();
          kvl.join("&")
        })
      })
    })
    .fmap(|v| v.join("&"))
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use anyhow::Result;

  use nom::multi::many1;

  use prop_check_rs::prop;
  use prop_check_rs::prop::TestCases;
  use prop_check_rs::rng::RNG;

  use crate::parser::parsers::Elms;

  use super::*;
  use super::gens::*;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_pchar_without_eq_and() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || pchar_without_eq_and_str_gen(1, u8::MAX - 1),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = many1(pchar_without_eq_and)(Elms::new(s.as_bytes()))
          .ok()
          .unwrap();
        r.into_iter().collect::<String>() == s
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_query() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || query_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, query) = query(Elms::new(s.as_bytes())).ok().unwrap();
        log::debug!("as_string = {:?}", query.as_string());
        let params = query.params();
        log::debug!("params = {:?}", params);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
