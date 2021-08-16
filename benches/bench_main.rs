#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(dead_code)]

use criterion::*;
use uri_rs::parser::parsers::{ipv4_address_parsers, Elms};
use uri_rs::Uri;

criterion_group!(benches, criterion_benchmark);

#[inline]
fn j5ik2o_uri_parse() {
  Uri::parse("http://host.com/abc?key1=abc");
}

#[inline]
fn j5ik2o_ipv4_address_parse() {
  ipv4_address_parsers::ipv4_address(Elms::new(b"255.255.255.255"));
}

fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("uri");
  let op = 0u8;
  group.bench_with_input(BenchmarkId::new("j5ik2o/uri/parse", op), &op, |b, i| {
    b.iter(|| j5ik2o_uri_parse())
  });
  group.bench_with_input(
    BenchmarkId::new("j5ik2o/uri/ipv4_address", op),
    &op,
    |b, i| b.iter(|| j5ik2o_ipv4_address_parse()),
  );
}

criterion_main! {
  benches,
}
