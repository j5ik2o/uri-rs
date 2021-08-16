use nom::character::complete;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::sequence::{preceded, terminated, tuple};

use crate::ast::authority::Authority;
use crate::parser::parsers::{Elms, host_parsers, port_parsers, UResult, user_info_parsers};

#[inline]
pub(crate) fn authority(i: Elms) -> UResult<Elms, Authority> {
  context(
    "authority",
    map(
      tuple((
        opt(terminated(
          user_info_parsers::user_info,
          complete::char('@'),
        )),
        host_parsers::host_name,
        opt(preceded(complete::char(':'), port_parsers::port)),
      )),
      |(ui, h, p)| Authority::new(h, p, ui),
    ),
  )(i)
}

#[cfg(test)]
pub mod gens {
  use prop_check_rs::gen::Gen;

  use crate::parser::parsers::basic_parsers::gens::to_option;
  use crate::parser::parsers::host_parsers::gens::host_gen;
  use crate::parser::parsers::port_parsers::gens::port_gen;
  use crate::parser::parsers::user_info_parsers::gens::user_info_gen;

  pub fn authority_gen() -> Gen<String> {
    let user_info_opt_gen = || to_option(|| user_info_gen());
    let port_opt_gen = || to_option(|| port_gen());

    user_info_opt_gen().bind(move |ui| {
      host_gen()
        .bind(move |h| {
          port_opt_gen().fmap(move |p| {
            let p = p.map(|s| format!(":{}", s)).unwrap_or("".to_string());
            format!("{}{}", h, p)
          })
        })
        .fmap(move |hp| {
          let ui = ui
            .as_ref()
            .map(|s| format!("{}@", s))
            .unwrap_or("".to_string());
          format!("{}{}", ui, hp)
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

  use crate::parser::parsers::authority_parsers::gens::authority_gen;

  use super::*;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_authority() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || authority_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, authority) = authority(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(authority.to_string(), s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
