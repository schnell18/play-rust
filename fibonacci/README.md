# Introduction

Some notes on fibonacci implementation in rust.

## Create static binary using musl libc

Static binary makes distribution of software easier. You can build
statically linked rust binary using [musl libc][1]. To install musl
target, type the following commands:

    rustup target add x86_64-unknown-linux-musl

Then you build the binary using `cargo` as follows:

    cargo build --target x86_64-unknown-linux-musl
