#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(dead_code)]

use criterion::*;
use uri_parsing_rs::parser::parsers::{Elms};

criterion_group!(benches, criterion_benchmark);

const uri: &'static str =
  "http://user1:pass1@localhost:8080/example?key1=value1&key2=value2&key1=value2#f1";

#[inline]
fn uri_parsers_uri() {
  let _ = uri_parsing_rs::parser::parsers::uri_parsers::uri(Elms::new(uri.as_bytes())).unwrap();
}

const hier_part: &'static str = "//user1:pass1@localhost:80801";

#[inline]
fn uri_hier_part_parsers_hier_part() {
  let _ =
      uri_parsing_rs::parser::parsers::hier_part_parsers::hier_part(Elms::new(hier_part.as_bytes())).unwrap();
}

const query: &'static str = "key1=value1&key2=value2&key1=value2";

fn uri_query_parsers_query() {
  let _ = uri_parsing_rs::parser::parsers::query_parsers::query(Elms::new(query.as_bytes())).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("uri");
  let op = 0u8;
  group.bench_with_input(
    BenchmarkId::new("j5ik2o/uri_parsers_uri", op),
    &op,
    |b, i| b.iter(|| uri_parsers_uri()),
  );
  group.bench_with_input(
    BenchmarkId::new("j5ik2o/uri_hier_part_parsers_hier_part", op),
    &op,
    |b, i| b.iter(|| uri_hier_part_parsers_hier_part()),
  );
  group.bench_with_input(
    BenchmarkId::new("j5ik2o/uri_query_parsers_query", op),
    &op,
    |b, i| b.iter(|| uri_query_parsers_query()),
  );
}

criterion_main! {
  benches,
}
