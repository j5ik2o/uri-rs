use itertools::Itertools;
use nom::AsChar;
use nom::branch::alt;
use nom::character::complete::{one_of, satisfy};
use nom::character::complete;
use nom::combinator::map;
use nom::sequence::tuple;

use crate::parser::parsers::{Elms, UResult};

pub(crate) fn is_unreserved(c: char) -> bool {
  let sc = ['-', '.', '_', '~'];
  c.is_alphanum() || sc.contains(&c)
}

// reserved    = ALPHA / DIGIT / "-" / "." / "_" / "~"
pub(crate) fn unreserved(i: Elms) -> UResult<Elms, char> {
  satisfy(move |c| is_unreserved(c))(i)
}

pub fn is_gen_delims(c: char) -> bool {
  ":/?#[]@".chars().contains(&c)
}

// gen-delims = ":" / "/" / "?" / "#" / "[" / "]" / "@"
pub(crate) fn gen_delims(i: Elms) -> UResult<Elms, char> {
  satisfy(|c| is_gen_delims(c))(i)
}

fn is_gen_delims_without_colon(c: char) -> bool {
  "/?#[]@".chars().contains(&c)
}

fn gen_delims_without_colon(i: Elms) -> UResult<Elms, char> {
  satisfy(|c| is_gen_delims_without_colon(c))(i)
}

pub(crate) fn is_sub_delims(c: char) -> bool {
  "!$&'()*+,;=".chars().contains(&c)
}

// sub-delims    = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
pub(crate) fn sub_delims(i: Elms) -> UResult<Elms, char> {
  satisfy(|c| is_sub_delims(c))(i)
}

pub(crate) fn is_sub_delims_without_eq_and(c: char) -> bool {
  "!$'()*+,;".chars().contains(&c)
}

pub(crate) fn sub_delims_without_eq_and(i: Elms) -> UResult<Elms, char> {
  satisfy(|c| is_sub_delims_without_eq_and(c))(i)
}

pub(crate) fn reserved(i: Elms) -> UResult<Elms, char> {
  alt((gen_delims, sub_delims))(i)
}

pub(crate) fn is_hex_digit(c: char) -> bool {
  c.is_hex_digit()
}

pub(crate) fn hex_digit(i: Elms) -> UResult<Elms, char> {
  satisfy(|c| is_hex_digit(c))(i)
}

pub(crate) fn is_digit(c: char) -> bool {
  c.is_digit(10)
}

pub(crate) fn digit(i: Elms) -> UResult<Elms, char> {
  satisfy(|c| is_digit(c))(i)
}

pub(crate) fn pct_encoded(i: Elms) -> UResult<Elms, String> {
  map(
    tuple((complete::char('%'), hex_digit, hex_digit)),
    |(c1, c2, c3)| [c1, c2, c3].iter().collect(),
  )(i)
}

pub(crate) fn pchar(i: Elms) -> UResult<Elms, String> {
  alt((
    map(unreserved, |c| c.into()),
    pct_encoded,
    map(sub_delims, |c| c.into()),
    map(one_of(":@"), |c| c.into()),
  ))(i)
}

pub(crate) fn pchar_without_eq_and(i: Elms) -> UResult<Elms, String> {
  alt((
    map(unreserved, |c| c.into()),
    pct_encoded,
    map(sub_delims_without_eq_and, |c| c.into()),
    map(one_of(":@"), |c| c.into()),
  ))(i)
}

pub(crate) fn pchar_without_colon(i: Elms) -> UResult<Elms, String> {
  alt((
    map(unreserved, |c| c.into()),
    pct_encoded,
    map(gen_delims_without_colon, |c| c.into()),
    map(one_of(":@"), |c| c.into()),
  ))(i)
}

#[cfg(test)]
pub mod gens {
  use itertools::Itertools;
  use prop_check_rs::gen::{Gen, Gens};

  pub fn to_option<F>(mut gen: F) -> Gen<Option<String>>
  where
    F: FnMut() -> Gen<String> + 'static,
  {
    Gens::one_bool().bind(move |b| {
      if b {
        gen().fmap(|v| Some(v))
      } else {
        Gen::<String>::unit(|| None)
      }
    })
  }

  // Generators
  fn low_alpha_gen() -> Gen<char> {
    let low_alpha_gen: Vec<char> = ('a'..='z').into_iter().collect_vec();
    Gens::one_of_vec(low_alpha_gen)
  }

  fn high_alpha_gen() -> Gen<char> {
    let low_alpha_gen: Vec<char> = ('A'..='Z').into_iter().collect_vec();
    Gens::one_of_vec(low_alpha_gen)
  }

  pub fn alpha_char_gen() -> Gen<char> {
    Gens::one_bool().bind(|b| if b { low_alpha_gen() } else { high_alpha_gen() })
  }

  pub fn digit_gen(min: char, max: char) -> Gen<char> {
    let low_alpha_gen: Vec<char> = (min..=max).into_iter().collect_vec();
    Gens::one_of_vec(low_alpha_gen)
  }

  pub fn hex_digit_char_gen() -> Gen<char> {
    Gens::choose_u8(1, 3).bind(|n| match n {
      1 => digit_gen('0', '9'),
      2 => Gens::choose('A', 'F'),
      3 => Gens::choose('a', 'f'),
      x => panic!("x = {}", x),
    })
  }

  pub fn rep_char_gen<F>(len: u8, mut f: F) -> Gen<String>
  where
    F: FnMut() -> Gen<char> + 'static,
  {
    Gens::choose_u8(1, len)
      .bind(move |len| Gens::list_of_n(len as usize, || f()).fmap(|sl| sl.into_iter().collect()))
  }

  pub fn rep_str_gen<F>(min: u8, max: u8, mut f: F) -> Gen<String>
  where
    F: FnMut() -> Gen<String> + 'static,
  {
    Gens::choose_u8(min, max)
      .bind(move |len| Gens::list_of_n(len as usize, || f()).fmap(|sl| sl.into_iter().collect()))
  }

  pub fn unreserved_char_gen() -> Gen<char> {
    Gens::choose(1u8, 3).bind(|n| match n {
      1 => alpha_char_gen(),
      2 => digit_gen('0', '9'),
      3 => Gens::one_of_vec(vec!['-', '.', '_', '~']),
      x => panic!("x = {}", x),
    })
  }

  pub fn unreserved_str_gen(len: u8) -> Gen<String> {
    rep_char_gen(len, || unreserved_char_gen())
  }

  pub fn gen_delims_char_gen() -> Gen<char> {
    Gens::one_of_vec(vec![':', '/', '?', '#', '[', ']', '@'])
  }

  pub fn gen_delims_str_gen(len: u8) -> Gen<String> {
    rep_char_gen(len, || gen_delims_char_gen())
  }

  pub fn sub_delims_char_gen() -> Gen<char> {
    Gens::one_of_vec(vec!['!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '='])
  }

  pub fn sub_delims_str_gen(len: u8) -> Gen<String> {
    rep_char_gen(len, || sub_delims_char_gen())
  }

  fn reserved_char_gen() -> Gen<char> {
    Gens::one_bool().bind(|b| {
      if b {
        gen_delims_char_gen()
      } else {
        sub_delims_char_gen()
      }
    })
  }

  pub fn reserved_str_gen(len: u8) -> Gen<String> {
    rep_char_gen(len, || reserved_char_gen())
  }

  pub fn pct_encoded_str_gen() -> Gen<String> {
    Gens::list_of_n(2, || hex_digit_char_gen()).fmap(|cl| {
      let s = cl.into_iter().collect::<String>();
      format!("%{}", s)
    })
  }

  pub fn pchar_str_gen(min: u8, max: u8) -> Gen<String> {
    rep_str_gen(min, max, || {
      Gens::choose_u8(1, 4).bind(|n| match n {
        1 => unreserved_char_gen().fmap(|c| c.into()),
        2 => pct_encoded_str_gen(),
        3 => sub_delims_char_gen().fmap(|c| c.into()),
        4 => Gens::one_of_vec(vec![':', '@']).fmap(|c| c.into()),
        x => panic!("x = {}", x),
      })
    })
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

  use super::*;
  use super::gens::*;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_unreserved() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || unreserved_str_gen(u8::MAX - 1),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = many1(unreserved)(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r.into_iter().collect::<String>(), s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_gen_delims() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || gen_delims_str_gen(u8::MAX - 1),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = many1(gen_delims)(Elms::new(s.as_bytes())).ok().unwrap();
        r.into_iter().collect::<String>() == s
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_sub_delims() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || sub_delims_str_gen(u8::MAX - 1),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = many1(sub_delims)(Elms::new(s.as_bytes())).ok().unwrap();
        r.into_iter().collect::<String>() == s
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_reserved() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || reserved_str_gen(u8::MAX - 1),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = many1(reserved)(Elms::new(s.as_bytes())).ok().unwrap();
        r.into_iter().collect::<String>() == s
      },
    );
    prop::test_with_prop(prop, 5, 1000, RNG::new())
  }

  #[test]
  fn test_pct_encoded() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || pct_encoded_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = pct_encoded(Elms::new(s.as_bytes())).ok().unwrap();
        r == s
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_pchar() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || pchar_str_gen(1, u8::MAX - 1),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = many1(pchar)(Elms::new(s.as_bytes())).ok().unwrap();
        r.into_iter().collect::<String>() == s
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
