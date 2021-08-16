# https://github.com/microsoft/vscode-remote-try-rust/
FROM rust:1.48-buster

RUN apt-get update & apt-get install git
RUN rustup component add rust-src
RUN rustup component add rustfmt
RUN cargo install cargo-edit

