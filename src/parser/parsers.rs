use nom::{IResult, Needed, Compare, CompareResult, InputTake, AsBytes, InputLength};

use nom::error::{ErrorKind, ContextError, ParseError};

use nom::lib::std::iter::{Copied, Enumerate};
use nom::lib::std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use nom::lib::std::slice::Iter;

use std::string::FromUtf8Error;
use std::str::Utf8Error;

pub mod authority_parsers;
pub mod basic_parsers;
pub mod fragment_parsers;
pub mod hier_part_parsers;
pub mod host_parsers;
pub mod ipv4_address_parsers;
pub mod ipv6_address_parsers;
pub mod path_parsers;
pub mod port_parsers;
pub mod query_parsers;
pub mod scheme_parsers;
pub mod uri_parsers;
pub mod user_info_parsers;

/// Custom Input Type
#[derive(Clone, PartialEq)]
pub struct Elms<'a> {
  values: &'a [u8],
}

impl<'a> Elms<'a> {
  pub fn new(values: &'a [u8]) -> Self {
    Self { values }
  }

  pub fn as_str(&self) -> Result<&str, Utf8Error> {
    std::str::from_utf8(self.values)
  }

  pub fn as_string(&self) -> Result<String, FromUtf8Error> {
    String::from_utf8(self.as_bytes().to_vec())
  }
}

impl<'a> nom::InputLength for Elms<'a> {
  fn input_len(&self) -> usize {
    self.values.len()
  }
}

impl<'a> nom::AsBytes for Elms<'a> {
  fn as_bytes(&self) -> &[u8] {
    self.values
  }
}

impl<'a> nom::InputTake for Elms<'a> {
  fn take(&self, count: usize) -> Self {
    let s = self.values.take(count);
    Elms::new(s)
  }

  fn take_split(&self, count: usize) -> (Self, Self) {
    let s = self.values.take_split(count);
    (Elms::new(s.0), Elms::new(s.1))
  }
}

impl<'a> nom::InputTakeAtPosition for Elms<'a> {
  type Item = u8;

  fn split_at_position<P, E: ParseError<Self>>(&self, predicate: P) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.values.iter().position(|c| predicate(*c)) {
      Some(i) => Ok(self.take_split(i)),
      None => Err(nom::Err::Incomplete(Needed::new(1))),
    }
  }

  fn split_at_position1<P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: ErrorKind,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.values.iter().position(|c| predicate(*c)) {
      Some(0) => Err(nom::Err::Error(E::from_error_kind(self.clone(), e))),
      Some(i) => Ok(self.take_split(i)),
      None => Err(nom::Err::Incomplete(Needed::new(1))),
    }
  }

  fn split_at_position_complete<P, E: ParseError<Self>>(
    &self,
    predicate: P,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.values.iter().position(|c| predicate(*c)) {
      Some(i) => Ok(self.take_split(i)),
      None => Ok(self.take_split(self.input_len())),
    }
  }

  fn split_at_position1_complete<P, E: ParseError<Self>>(
    &self,
    predicate: P,
    e: ErrorKind,
  ) -> IResult<Self, Self, E>
  where
    P: Fn(Self::Item) -> bool,
  {
    match self.values.iter().position(|c| predicate(*c)) {
      Some(0) => Err(nom::Err::Error(E::from_error_kind(self.clone(), e))),
      Some(i) => Ok(self.take_split(i)),
      None => {
        if self.values.is_empty() {
          Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
        } else {
          Ok(self.take_split(self.input_len()))
        }
      }
    }
  }
}

impl<'a, 'b> Compare<Elms<'b>> for Elms<'a> {
  fn compare(&self, t: Elms<'b>) -> CompareResult {
    self.values.compare(t.values)
  }

  fn compare_no_case(&self, t: Elms<'b>) -> CompareResult {
    self.values.compare_no_case(t.values)
  }
}

impl<'a, 'b> Compare<&'b str> for Elms<'a> {
  #[inline(always)]
  fn compare(&self, t: &'b str) -> CompareResult {
    self.values.compare(AsBytes::as_bytes(t))
  }
  #[inline(always)]
  fn compare_no_case(&self, t: &'b str) -> CompareResult {
    self.values.compare_no_case(AsBytes::as_bytes(t))
  }
}

/// InputIter for Elms
///
/// This type provides essentially the same implementation as &'a [u8].
impl<'a> nom::InputIter for Elms<'a> {
  type Item = u8;
  type Iter = Enumerate<Self::IterElem>;
  type IterElem = Copied<Iter<'a, u8>>;

  fn iter_indices(&self) -> Self::Iter {
    self.values.iter_indices()
  }

  fn iter_elements(&self) -> Self::IterElem {
    self.values.iter_elements()
  }

  fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool,
  {
    self.values.position(predicate)
  }

  fn slice_index(&self, count: usize) -> Result<usize, Needed> {
    self.values.slice_index(count)
  }
}

impl<'a> nom::Slice<Range<usize>> for Elms<'a> {
  fn slice(&self, range: Range<usize>) -> Self {
    let s = &self.values[range];
    Elms::new(s)
  }
}

impl<'a> nom::Slice<RangeTo<usize>> for Elms<'a> {
  fn slice(&self, range: RangeTo<usize>) -> Self {
    let s = &self.values[range];
    Elms::new(s)
  }
}

impl<'a> nom::Slice<RangeFrom<usize>> for Elms<'a> {
  fn slice(&self, range: RangeFrom<usize>) -> Self {
    let s = &self.values[range];
    Elms::new(s)
  }
}

impl<'a> nom::Slice<RangeFull> for Elms<'a> {
  fn slice(&self, range: RangeFull) -> Self {
    let s = &self.values[range];
    Elms::new(s)
  }
}

/// Custom parse error type.
#[derive(Debug, PartialEq)]
pub struct UriParseError {
  message: String,
}

impl ContextError<Elms<'_>> for UriParseError {
  fn add_context(_input: Elms, _ctx: &'static str, other: Self) -> Self {
    other
  }
}

impl ParseError<Elms<'_>> for UriParseError {
  fn from_error_kind(input: Elms, kind: ErrorKind) -> Self {
    let input = input.as_str().unwrap();
    let message = format!("{:?}:\t{:?}\n", kind, input);
    Self { message }
  }

  fn append(input: Elms, kind: ErrorKind, other: Self) -> Self {
    let input = input.as_str().unwrap();
    let message = format!("{}{:?}:\t{:?}\n", other.message, kind, input);
    Self { message }
  }

  fn from_char(input: Elms, c: char) -> Self {
    let input = input.as_str().unwrap();
    let message = format!("'{}':\t{:?}\n", c, input);
    Self { message }
  }

  fn or(self, other: Self) -> Self {
    let message = format!("{}\tOR\n{}\n", self.message, other.message);
    Self { message }
  }
}

pub type UResult<T, U> = IResult<T, U, UriParseError>;
