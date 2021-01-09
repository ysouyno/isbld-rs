# isbld-rs

## How to compile into a static executable file

``` shellsession
set RUSTFLAGS=-C target-feature=+crt-static
cargo build
```
