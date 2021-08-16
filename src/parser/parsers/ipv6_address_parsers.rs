use itertools::Itertools;
use nom::branch::*;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::*;
use nom::error::context;
use nom::multi::*;
use nom::sequence::*;

use crate::parser::parsers::{Elms, ipv4_address_parsers, UResult};
use crate::parser::parsers::basic_parsers::*;

#[inline]
pub(crate) fn h16(i: Elms) -> UResult<Elms, String> {
  context(
    "h16",
    map(many_m_n(1, 4, hex_digit), |sl| sl.into_iter().collect()),
  )(i)
}

#[inline]
pub(crate) fn ls32(i: Elms) -> UResult<Elms, String> {
  context(
    "ls32",
    alt((
      map(
        tuple((h16, preceded(complete::char(':'), h16))),
        |(c1, c2)| [c1, c2].join(":"),
      ),
      ipv4_address_parsers::ipv4_address,
    )),
  )(i)
}

// 6( h16 ":" ) ls32
#[inline]
pub(crate) fn ipv6_address1(i: Elms) -> UResult<Elms, String> {
  context(
    "ipv6_address1",
    map(
      tuple((
        map(count(terminated(h16, complete::char(':')), 6), |sl| {
          sl.iter().join(":")
        }),
        ls32,
      )),
      |(s1, s2)| [s1, s2].join(":"),
    ),
  )(i)
}

// "::" 5( h16 ":" ) ls32
#[inline]
pub(crate) fn ipv6_address2(i: Elms) -> UResult<Elms, String> {
  context(
    "ipv6_address2",
    map(
      tuple((
        preceded(
          tag("::"),
          map(count(terminated(h16, tag(":")), 5), |sl| sl.join(":")),
        ),
        ls32,
      )),
      |(b, c)| format!("::{}:{}", b, c),
    ),
  )(i)
}

// [ h16 ] "::" 4( h16 ":" ) ls32
#[inline]
pub(crate) fn ipv6_address3(i: Elms) -> UResult<Elms, String> {
  context(
    "ipv6_address3",
    map(
      tuple((
        terminated(opt(h16), tag("::")),
        map(
          count(
            map(terminated(h16, complete::char(':')), |s| format!("{}:", s)),
            4,
          ),
          |sl| sl.into_iter().collect::<String>(),
        ),
        ls32,
      )),
      |(s1, s2, s3)| format!("{}::{}{}", s1.unwrap_or("".to_string()), s2, s3),
    ),
  )(i)
}

#[inline]
fn ipv6_address_<'a>(n1: usize, n2: usize) -> impl FnMut(Elms<'a>) -> UResult<Elms<'a>, String> {
  let h16_with_colon = || {
    map(tuple((h16, complete::char(':'))), |(s, c)| {
      format!("{}{}", s, c)
    })
  };
  let colon_with_h16 = || {
    map(tuple((complete::char(':'), h16)), |(c, s)| {
      format!("{}{}", c, s)
    })
  };
  map(
    tuple((
      terminated(
        map(
          opt(map(
            tuple((h16, many_m_n(0, n1, colon_with_h16()))),
            |(s1, s2)| format!("{}{}", s1, s2.into_iter().collect::<String>()),
          )),
          |v| v.unwrap_or("".to_string()),
        ),
        tag("::"),
      ),
      count(h16_with_colon(), n2),
      ls32,
    )),
    |(s1, s2, s3)| format!("{}::{}{}", s1, s2.into_iter().collect::<String>(), s3),
  )
}

// [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
// [ h16 [ ":" h16 ] ] "::" 3( h16 ":" ) ls32
#[inline]
pub(crate) fn ipv6_address4(i: Elms) -> UResult<Elms, String> {
  context("ipv6_address4", ipv6_address_(1, 3))(i)
}

// [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
// [ h16 *2(":" h16) ] "::" 2( h16 ":" ) ls32
#[inline]
pub(crate) fn ipv6_address5(i: Elms) -> UResult<Elms, String> {
  context("ipv6_address5", ipv6_address_(2, 2))(i)
}

// [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
// [ h16 *3(":" h16) ] "::"    h16 ":"   ls32
#[inline]
pub(crate) fn ipv6_address6(i: Elms) -> UResult<Elms, String> {
  context("ipv6_address6", ipv6_address_(3, 1))(i)
}

// [ *4( h16 ":" ) h16 ] "::"              ls32
// [ h16 *4(":" h16) ] "::"              ls32
#[inline]
pub(crate) fn ipv6_address7(i: Elms) -> UResult<Elms, String> {
  context(
    "ipv6_address7",
    map(
      tuple((
        terminated(
          map(
            opt(map(
              tuple((
                h16,
                many_m_n(
                  0,
                  4,
                  map(preceded(complete::char(':'), h16), |s| format!(":{}", s)),
                ),
              )),
              |(s1, s2)| format!("{}{}", s1, s2.into_iter().collect::<String>()),
            )),
            |v| v.unwrap_or("".to_string()),
          ),
          tag("::"),
        ),
        ls32,
      )),
      |(s1, s2)| [s1, s2].join("::"),
    ),
  )(i)
}

// [ *5( h16 ":" ) h16 ] "::" h16
// [ h16 *5(":" h16) ] "::"  h16
#[inline]
pub(crate) fn ipv6_address8(i: Elms) -> UResult<Elms, String> {
  context(
    "ipv6_address8",
    map(
      tuple((
        terminated(
          map(
            opt(map(
              tuple((
                h16,
                many_m_n(
                  0,
                  5,
                  map(preceded(complete::char(':'), h16), |s| format!(":{}", s)),
                ),
              )),
              |(s1, s2)| format!("{}{}", s1, s2.into_iter().collect::<String>()),
            )),
            |v| v.unwrap_or("".to_string()),
          ),
          tag("::"),
        ),
        h16,
      )),
      |(s1, s2)| [s1, s2].join("::"),
    ),
  )(i)
}

// [ *6( h16 ":" ) h16 ] "::"
// [ h16 *6(":" h16) ] "::"
#[inline]
pub(crate) fn ipv6_address9(i: Elms) -> UResult<Elms, String> {
  context(
    "ipv6_address9",
    map(
      tuple((
        map(
          opt(map(
            tuple((
              h16,
              many_m_n(
                0,
                6,
                map(tuple((complete::char(':'), h16)), |(c, s)| {
                  let mut r = String::new();
                  r.push(c);
                  r.push_str(&s);
                  r
                }),
              ),
            )),
            |(s1, s2)| [s1, s2.join("")].join(""),
          )),
          |v: Option<String>| v.unwrap_or("".to_string()),
        ),
        tag("::"),
      )),
      |(s1, s2): (String, Elms)| s1 + s2.as_str().unwrap(),
    ),
  )(i)
}

#[inline]
pub fn ipv6_address(i: Elms) -> UResult<Elms, String> {
  context(
    "ipv6_address",
    alt((
      ipv6_address1,
      ipv6_address2,
      ipv6_address3,
      ipv6_address4,
      ipv6_address5,
      ipv6_address6,
      ipv6_address7,
      ipv6_address8,
      ipv6_address9,
    )),
  )(i)
}

#[cfg(test)]
pub mod gens {
  use itertools::Itertools;
  use prop_check_rs::gen::{Gen, Gens};

  use crate::parser::parsers::ipv4_address_parsers::gens::*;
  use crate::parser::parsers::basic_parsers::gens::*;

  pub fn h16_gen() -> Gen<String> {
    Gens::choose_u8(1, 4).bind(|n| rep_char_gen(n, || hex_digit_char_gen()))
  }

  pub fn ls32_gen() -> Gen<String> {
    Gens::choose_u8(1, 2).bind(|n| match n {
      1 => ipv4_address_str_gen(),
      2 => Gens::list_of_n(2, || h16_gen()).fmap(|sl| sl.join(":")),
      x => panic!("x = {}", x),
    })
  }
  pub fn ipv6_address_gen1() -> Gen<String> {
    Gens::list_of_n(6, || h16_gen())
      .bind(|sl| ls32_gen().fmap(move |ls32| format!("{}:{}", sl.join(":"), ls32)))
  }

  pub fn ipv6_address_gen2() -> Gen<String> {
    Gens::list_of_n(5, || h16_gen())
      .bind(|sl| ls32_gen().fmap(move |ls32| format!("::{}:{}", sl.join(":"), ls32)))
  }

  pub fn ipv6_address_gen3() -> Gen<String> {
    Gens::list_of_n(5, || h16_gen()).bind(|sl| {
      ls32_gen().fmap(move |ls32| {
        let (h, t) = sl.split_first().unwrap();
        format!("{}::{}:{}", h, t.join(":"), ls32)
      })
    })
  }

  // [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
  pub fn ipv6_address_gen4() -> Gen<String> {
    Gens::one_bool()
      .bind(|b| {
        if b {
          Gens::choose_u8(1, 2).bind(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(2, || h16_gen()).fmap(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .bind(|s0| {
        Gens::list_of_n(3, || h16_gen().fmap(|v| format!("{}:", v)))
          .fmap(|sl| sl.iter().join(""))
          .fmap(|s| format!("::{}", s))
          .bind(|s2| ls32_gen().fmap(move |s3| format!("{}{}", s2, s3)))
          .fmap(move |s| format!("{}{}", s0, s))
      })
  }

  //  [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
  pub fn ipv6_address_gen5() -> Gen<String> {
    Gens::one_bool()
      .bind(|b| {
        if b {
          Gens::choose_u8(1, 2).bind(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(3, || h16_gen()).fmap(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .bind(|s0| {
        Gens::list_of_n(2, || h16_gen().fmap(|v| format!("{}:", v)))
          .fmap(|sl| sl.iter().join(""))
          .fmap(|s| format!("::{}", s))
          .bind(|s2| ls32_gen().fmap(move |s3| format!("{}{}", s2, s3)))
          .fmap(move |s| format!("{}{}", s0, s))
      })
  }

  //  [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
  pub fn ipv6_address_gen6() -> Gen<String> {
    Gens::one_bool()
      .bind(|b| {
        if b {
          Gens::choose_u8(1, 2).bind(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(3, || h16_gen()).fmap(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .bind(|s0| {
        Gens::list_of_n(1, || h16_gen().fmap(|v| format!("{}:", v)))
          .fmap(|sl| sl.join(""))
          .fmap(|s| format!("::{}", s))
          .bind(|s2| ls32_gen().fmap(move |s3| format!("{}{}", s2, s3)))
          .fmap(move |s| format!("{}{}", s0, s))
      })
  }

  // [ *4( h16 ":" ) h16 ] "::"              ls32
  pub fn ipv6_address_gen7() -> Gen<String> {
    Gens::one_bool()
      .bind(|b| {
        if b {
          Gens::choose_u8(1, 2).bind(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(4, || h16_gen()).fmap(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .bind(|s0| ls32_gen().fmap(move |s1| format!("{}::{}", s0, s1)))
  }

  //  [ *5( h16 ":" ) h16 ] "::"              h16
  pub fn ipv6_address_gen8() -> Gen<String> {
    Gens::one_bool()
      .bind(|b| {
        if b {
          Gens::choose_u8(1, 2).bind(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(5, || h16_gen()).fmap(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .bind(|s0| h16_gen().fmap(move |s1| format!("{}::{}", s0, s1)))
  }

  //  [ *6( h16 ":" ) h16 ] "::"
  pub fn ipv6_address_gen9() -> Gen<String> {
    Gens::one_bool()
      .bind(|b| {
        if b {
          Gens::choose_u8(1, 2).bind(|n| match n {
            1 => h16_gen(),
            2 => Gens::list_of_n(6, || h16_gen()).fmap(|sl| sl.join(":")),
            x => panic!("x = {}", x),
          })
        } else {
          Gen::<String>::unit(|| "".to_string())
        }
      })
      .fmap(|s0| format!("{}::", s0))
  }

  pub fn ipv6_address_str_gen() -> Gen<String> {
    Gens::choose_u8(1, 9).bind(|n| match n {
      1 => ipv6_address_gen1(),
      2 => ipv6_address_gen2(),
      3 => ipv6_address_gen3(),
      4 => ipv6_address_gen4(),
      5 => ipv6_address_gen5(),
      6 => ipv6_address_gen6(),
      7 => ipv6_address_gen7(),
      8 => ipv6_address_gen8(),
      9 => ipv6_address_gen9(),
      x => panic!("x = {}", x),
    })
  }
}

#[cfg(test)]
mod test {
  use std::env;

  use anyhow::Result;

  use prop_check_rs::prop;
  use prop_check_rs::prop::TestCases;
  use prop_check_rs::rng::RNG;

  use crate::parser::parsers::ipv6_address_parsers;

  use super::*;
  use super::gens::*;

  const TEST_COUNT: TestCases = 100;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_h16() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || h16_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = h16(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ls32() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ls32_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = ls32(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv6_address1() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv6_address_gen1(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = ipv6_address1(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv6_address2() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv6_address_gen2(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = ipv6_address2(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv6_address3() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv6_address_gen3(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        ipv6_address3(Elms::new(s.as_bytes())).is_ok()
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv6_address4() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv6_address_gen4(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let r = ipv6_address4(Elms::new(s.as_bytes()));
        let (_, r) = r.ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv6_address5() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv6_address_gen5(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = ipv6_address_parsers::ipv6_address5(Elms::new(s.as_bytes()))
          .ok()
          .unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv6_address6() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv6_address_gen6(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = ipv6_address6(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv6_address7() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv6_address_gen7(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = ipv6_address_parsers::ipv6_address7(Elms::new(s.as_bytes()))
          .ok()
          .unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv6_address8() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv6_address_gen8(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = ipv6_address8(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv6_address9() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv6_address_gen9(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = ipv6_address9(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }

  #[test]
  fn test_ipv6_address() -> Result<()> {
    init();
    let mut counter = 0;
    let prop = prop::for_all(
      || ipv6_address_str_gen(),
      move |s| {
        counter += 1;
        log::debug!("{}, value = {}", counter, s);
        let (_, r) = ipv6_address(Elms::new(s.as_bytes())).ok().unwrap();
        assert_eq!(r, s);
        true
      },
    );
    prop::test_with_prop(prop, 5, TEST_COUNT, RNG::new())
  }
}
