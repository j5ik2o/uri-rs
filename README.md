# uri-rs

A Rust crate for URI.

[![Workflow Status](https://github.com/j5ik2o/uri-rs/workflows/Rust/badge.svg)](https://github.com/j5ik2o/uri-rs/actions?query=workflow%3A%22Rust%22)
[![crates.io](https://img.shields.io/crates/v/uri-rs.svg)](https://crates.io/crates/uri-rs)
[![docs.rs](https://docs.rs/uri-rs/badge.svg)](https://docs.rs/uri-rs)
[![dependency status](https://deps.rs/repo/github/j5ik2o/uri-rs/status.svg)](https://deps.rs/repo/github/j5ik2o/uri-rs)
[![tokei](https://tokei.rs/b1/github/j5ik2o/uri-rs)](https://github.com/XAMPPRocky/tokei)

## Install to Cargo.toml

Add this to your `Cargo.toml`:

```toml
[dependencies]
uri-rs = "<<version>>"
```

## Usage

```rust
use uri_rs::Uri;
let s = "http://user1:pass1@localhost:8080/example?key1=value1&key2=value2&key1=value2#f1";
let uri = Uri::parse(s).unwrap();
println!("{:?}", uri);
// Uri {
//    schema: Scheme("http"),
//    authority: Some(
//      Authority {
//        host_name: HostName("localhost"),
//        port: Some(8080),
//        user_info: Some(
//          UserInfo {
//            user_name: "user1",
//            password: Some("pass1")
//          }
//        )
//      }
//    ),
//    path: AbemptyPath {
//      type_name: "abempty_path",
//      parts: ["example"]
//    },
//    query: Some(
//      Query {
//        params: [
//          ("key1", Some("value1")),
//          ("key2", Some("value2")),
//          ("key1", Some("value2"))
//        ]
//      }
//    ),
//    fragment: Some("f1")
// }
println!("{}", uri.to_string());
// http://user1:pass1@localhost:8080/example?key1=value1&key2=value2&key1=value2#f1
```

## Benchmarks

TODO

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
