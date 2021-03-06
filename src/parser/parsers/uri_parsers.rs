use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::{eof, map, opt};
use nom::error::context;
use nom::sequence::{preceded, terminated, tuple};

use crate::ast::uri::Uri;
use crate::parser::parsers::{
  Elms, fragment_parsers, hier_part_parsers, query_parsers, scheme_parsers, UResult,
};

// absolute-URI  = scheme ":" hier-part [ "?" query ]
#[inline]
pub fn absolute_uri(i: Elms) -> UResult<Elms, Uri> {
  context(
    "absolute_uri",
    map(
      tuple((
        scheme_parsers::scheme,
        terminated(
          preceded(
            tag(":"),
            tuple((
              hier_part_parsers::hier_part,
              opt(preceded(complete::char('?'), query_parsers::query)),
            )),
          ),
          eof,
        ),
      )),
      |(s, ((a, p), q))| Uri::new(s, a, p, q, None),
    ),
  )(i)
}

// URI = scheme ":" hier-part [ "?" query ] [ "#" fragment ]
#[inline]
pub fn uri(i: Elms) -> UResult<Elms, Uri> {
  context(
    "uri",
    map(
      tuple((
        scheme_parsers::scheme,
        terminated(
          preceded(
            tag(":"),
            tuple((
              hier_part_parsers::hier_part,
              opt(preceded(complete::char('?'), query_parsers::query)),
              opt(preceded(complete::char('#'), fragment_parsers::fragment)),
            )),
          ),
          eof,
        ),
      )),
      |(s, ((a, p), q, f))| Uri::new(s, a, p, q, f),
    ),
  )(i)
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::Gen;

  use crate::parser::parsers::fragment_parsers::gens::fragment_str_gen;
  use crate::parser::parsers::hier_part_parsers::gens::hier_part_gen;
  use crate::parser::parsers::query_parsers::gens::query_gen;
  use crate::parser::parsers::scheme_parsers::gens::scheme_gen;
  use crate::parser::parsers::path_parsers::gens::Pair;

  pub fn uri_gen() -> Gen<String> {
    scheme_gen().bind(|scheme| {
      let base_gen = hier_part_gen()
        .fmap(move |Pair(hier_part, is_empty)| (format!("{}:{}", scheme, hier_part), is_empty));
      let query_gen = base_gen.bind(|(s, is_empty_opt)| {
        if is_empty_opt.unwrap_or(false) {
          Gen::<(String, Option<bool>)>::unit(|| (s.clone(), is_empty_opt))
        } else {
          query_gen().fmap(move |q| (format!("{}?{}", s, q), is_empty_opt))
        }
      });
      let fragment_gen = query_gen.bind(|(s, is_empty_opt)| {
        if is_empty_opt.unwrap_or(false) {
          Gen::<(String, Option<bool>)>::unit(|| s.clone())
        } else {
          fragment_str_gen().fmap(move |f| format!("{}#{}", s, f))
        }
      });
      fragment_gen
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
  fn test_uri() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || uri_gen(),
      move |s| {
        counter += 1;
        log::debug!("{:>03} uri = {}", counter, s);
        let (_, uri) = uri(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(uri.to_string(), s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
