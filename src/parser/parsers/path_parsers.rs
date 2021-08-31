use nom::branch::alt;
use nom::character::complete;
use nom::combinator::{eof, map, not, opt, value};
use nom::error::context;
use nom::multi::{many0, many1};
use nom::sequence::{preceded, tuple};

use crate::ast::path::Path;
use crate::parser::parsers::{Elms, UResult};
use crate::parser::parsers::basic_parsers::*;

#[inline]
pub(crate) fn segment(i: Elms) -> UResult<Elms, String> {
  map(many0(pchar), |sl| sl.into_iter().collect())(i)
}

#[inline]
fn segment_without_colon(i: Elms) -> UResult<Elms, String> {
  map(many0(pchar_without_colon), |sl| sl.into_iter().collect())(i)
}

#[inline]
pub(crate) fn segment_nz(i: Elms) -> UResult<Elms, String> {
  map(many1(pchar), |sl| sl.into_iter().collect())(i)
}

#[inline]
pub(crate) fn segment_nz_nc(i: Elms) -> UResult<Elms, String> {
  map(
    many1(alt((
      // ALPHA / DIGIT / "-" / "." / "_" / "~"
      map(unreserved, |c| c.into()),
      pct_encoded,
      // "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "
      map(sub_delims, |c| c.into()),
      map(complete::char('@'), |c| c.into()),
    ))),
    |sl| sl.into_iter().collect(),
  )(i)
}

#[inline]
pub(crate) fn path_abempty(i: Elms) -> UResult<Elms, Path> {
  map(many0(preceded(complete::char('/'), segment)), |sl| {
    Path::of_abempty_from_strings(&sl)
  })(i)
}

#[inline]
pub(crate) fn path_absolute(i: Elms) -> UResult<Elms, Path> {
  context(
    "path_absolute",
    map(
      preceded(
        complete::char('/'),
        opt(map(
          tuple((segment_nz, many0(preceded(complete::char('/'), segment)))),
          |(s, sl)| {
            let mut r = vec![s];
            r.extend(sl);
            r
          },
        )),
      ),
      |sl_opt| Path::of_absolute_from_strings(&sl_opt.unwrap_or(Vec::new())),
    ),
  )(i)
}

pub(crate) fn path_no_scheme(i: Elms) -> UResult<Elms, Path> {
  log::debug!("path_no_scheme = {}", i.as_str().unwrap());
  let result = context(
    "path_no_scheme",
    map(
      tuple((segment_nz_nc, many0(preceded(complete::char('/'), segment)))),
      |(s, sl)| {
        let mut parts = vec![s];
        parts.extend(sl);
        Path::of_no_scheme_from_strings(&parts)
      },
    ),
  )(i);
  match result {
    Ok((p1, p2)) => {
      log::debug!("p1 = {}", p1);
      log::debug!("p2 = {}", p2);
      Ok((p1, p2))
    }
    Err(e) => {
      log::debug!("e = {}", e);
      Err(e)
    }
  }
}

pub(crate) fn path_rootless(i: Elms) -> UResult<Elms, Path> {
  context(
    "path_rootless",
    map(
      tuple((segment_nz, many0(preceded(complete::char('/'), segment)))),
      |(s, sl)| {
        let mut parts = vec![s];
        parts.extend(sl);
        Path::of_rootless_from_strings(&parts)
      },
    ),
  )(i)
}

#[inline]
pub(crate) fn path_empty(i: Elms) -> UResult<Elms, Path> {
  context("path_empty", value(Path::of_empty(), eof))(i)
}

#[inline]
pub(crate) fn path_without_abempty(i: Elms) -> UResult<Elms, Path> {
  let is_absolute = opt(preceded(complete::char('/'), not(complete::char('/'))))(i.clone())
    .map(|(_, v)| v.is_some())?;
  let is_no_scheme =
    opt(segment_nz_nc)(i.clone()).map(|(_, v)| v.iter().any(|s| !s.contains(':')))?;
  let is_empty = opt(eof)(i.clone()).map(|(_, v)| v.is_some())?;

  log::debug!("is_absolute = {}", is_absolute);
  log::debug!("is_no_scheme = {}", is_no_scheme);
  log::debug!("is_empty = {}", is_empty);

  if is_empty {
    path_empty(i.clone())
  } else {
    if is_absolute {
      path_absolute(i)
    // } else if is_no_scheme {
    //   path_no_scheme(i)
    } else {
      path_rootless(i)
    }
  }
}

#[cfg(test)]
pub mod gens {
  use std::fmt::Formatter;

  use prop_check_rs::gen::{Gen, Gens};

  use crate::parser::parsers::basic_parsers::gens::*;

  pub fn segment_str_gen() -> Gen<String> {
    pchar_str_gen(0, u8::MAX - 1)
  }

  pub fn segment_nz_str_gen() -> Gen<String> {
    pchar_str_gen(1, u8::MAX - 1)
  }

  pub fn segment_nz_nc_str_gen() -> Gen<String> {
    rep_str_gen(1, u8::MAX - 1, || {
      Gens::choose_u8(1, 2).bind(|n| match n {
        1 => unreserved_char_gen().fmap(|c| c.into()),
        2 => pct_encoded_str_gen(),
        3 => sub_delims_char_gen().fmap(|c| c.into()),
        4 => Gens::one_of_vec(vec!['@']).fmap(|c| c.into()),
        x => panic!("x = {}", x),
      })
    })
  }

  pub fn path_abempty_str_gen() -> Gen<String> {
    rep_str_gen(1, 10, || segment_str_gen().fmap(|s| format!("/{}", s)))
  }

  pub fn path_absolute_str_gen() -> Gen<String> {
    rep_str_gen(1, 10, || segment_nz_str_gen().fmap(|s| format!("/{}", s))).bind(|s1| {
      path_abempty_str_gen().fmap(move |s2| {
        let prefix = if !s1.starts_with("/") { "/" } else { "" };
        format!("{}{}{}", prefix, s1, s2)
      })
    })
  }

  pub fn path_no_scheme_str_gen() -> Gen<String> {
    segment_nz_nc_str_gen().bind(|s1| {
      rep_str_gen(1, 10, || segment_str_gen().fmap(|s2| format!("/{}", s2)))
        .fmap(move |s2| format!("{}{}", s1, s2))
    })
  }

  pub fn path_rootless_str_gen() -> Gen<String> {
    segment_nz_str_gen().bind(|s1| {
      rep_str_gen(1, 10, || segment_str_gen().fmap(|s2| format!("/{}", s2)))
        .fmap(move |s2| format!("{}{}", s1, s2))
    })
  }

  #[derive(Clone, Debug)]
  pub struct Pair<A, B>(pub(crate) A, pub(crate) B);

  impl<A, B> std::fmt::Display for Pair<A, B>
  where
    A: std::fmt::Display,
    B: std::fmt::Display,
  {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "({},{})", self.0, self.1)
    }
  }

  pub fn path_str_with_abempty_gen() -> Gen<Pair<String, String>> {
    Gens::choose_u8(1, 5).bind(|n| match n {
      1 => path_abempty_str_gen().fmap(|s| Pair("abempty_path".to_string(), s)),
      2 => path_absolute_str_gen().fmap(|s| Pair("absolute_path".to_string(), s)),
      3 => path_no_scheme_str_gen().fmap(|s| Pair("no_scheme_path".to_string(), s)),
      4 => path_rootless_str_gen().fmap(|s| Pair("rootless_path".to_string(), s)),
      5 => Gen::<String>::unit(|| Pair("empty_path".to_string(), "".to_string())),
      x => panic!("x = {}", x),
    })
  }

  pub fn path_str_without_abempty_gen() -> Gen<Pair<String, String>> {
    Gens::choose_u8(1, 3).bind(|n| match n {
      1 => path_absolute_str_gen().fmap(|s| Pair("absolute_path".to_string(), s)),
      2 => path_rootless_str_gen().fmap(|s| Pair("rootless_path".to_string(), s)),
      3 => Gen::<String>::unit(|| Pair("empty_path".to_string(), "".to_string())),
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
  fn test_segment() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || segment_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, segment = {}", counter, s);
        let (_, r) = segment(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, 1000, RNG::new())
  }

  #[test]
  fn test_segment_nz() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || segment_nz_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = segment_nz(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_segment_nz_nc() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || segment_nz_nc_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, segment_nz_nc = {}", counter, s);
        let (_, r) = segment_nz_nc(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_abempty() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || path_abempty_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{:>03}, path_abempty = {}", counter, s);
        let (_, r) = path_abempty(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r.to_string(), s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_absolute() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || path_absolute_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{:>03}, path_absolute = {}", counter, s);
        let (_, r) = path_absolute(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r.to_string(), s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_no_scheme() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || path_no_scheme_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{:>03}, path_no_scheme = {}", counter, s);
        let (_, r) = path_no_scheme(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r.to_string(), s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_rootless() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || path_rootless_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{:>03}, path_rootless = {}", counter, s);
        let (_, r) = path_rootless(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r.to_string(), s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_path_without_abempty() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || path_str_without_abempty_gen(),
      move |s| {
        counter += 1;
        log::debug!(
          "{:>03}, path_type = {:?}, path_without_abempty = {:?}",
          counter,
          s.0,
          s.1
        );
        let (_, r) = path_without_abempty(Elms::new(s.1.as_bytes()))
          .ok()
          .unwrap();
        assert_eq!(r.type_name(), s.0);
        assert_eq!(r.to_string(), s.1);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
