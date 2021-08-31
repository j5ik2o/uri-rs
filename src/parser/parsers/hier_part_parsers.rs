use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::{preceded, tuple};

use crate::ast::authority::Authority;
use crate::ast::path::Path;
use crate::parser::parsers::{authority_parsers, Elms, path_parsers, UResult};

// hier-part     = "//" authority path-abempty
// / path-absolute
// / path-rootless
// / path-empty
#[inline]
pub(crate) fn hier_part(i: Elms) -> UResult<Elms, (Option<Authority>, Path)> {
  if let (i, Some((authority, path))) = opt(preceded(
    tag("//"),
    tuple((authority_parsers::authority, path_parsers::path_abempty)),
  ))(i.clone())?
  {
    Ok((i, (Some(authority), path)))
  } else {
    log::debug!("path_without_abempty = {}", i.clone());
    let (i, path) = path_parsers::path_without_abempty(i)?;
    Ok((i, (None, path)))
  }
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parser::parsers::authority_parsers::gens::authority_gen;
  use crate::parser::parsers::path_parsers::gens::*;

  pub fn hier_part_gen() -> Gen<Pair<String, Option<bool>>> {
    let gen1 = || {
      authority_gen().bind(move |authority| {
        path_abempty_str_gen().fmap(move |path_abempty| format!("//{}{}", authority, path_abempty))
      })
    };
    let gen2 = || {
      path_str_without_abempty_gen().fmap(|Pair(p1, p2)| {
        println!("p1 = {}", p1);
        Pair(p2, Some(p1 == "empty_path".to_string()))
      })
    };
    Gens::one_bool().bind(move |b| {
      if b {
        gen1().fmap(|s| Pair(s, None))
      } else {
        gen2()
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
  use crate::parser::parsers::path_parsers::gens::Pair;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_hier_part() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || hier_part_gen(),
      move |Pair(s, _b)| {
        counter += 1;
        log::debug!("{:>03}, hier_part = {}", counter, s);
        let (_, (authority, path)) = hier_part(Elms::new(s.as_bytes())).ok().unwrap();
        let sa = authority
          .map(|e| format!("//{}", e))
          .unwrap_or("".to_string());
        let sp = path.to_string();
        let sap = format!("{}{}", sa, sp);
        assert_eq!(sap, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
