# Prepare the project

This chapter starts from the real example at:
<https://github.com/phper-framework/phper/tree/master/examples/http-client>

Goal: build a PHP extension that exposes a simple object-oriented HTTP client API backed by `reqwest::blocking`.

Expected PHP usage:

```php
<?php

use HttpClient\HttpClientBuilder;

$client = (new HttpClientBuilder())
    ->timeout(15000)
    ->cookie_store(true)
    ->build();

$response = $client->get("https://httpbin.org/ip")->send();

var_dump([
    "status" => $response->status(),
    "headers" => $response->headers(),
    "body" => $response->body(),
]);
```

## 1) Environment

Make sure `libclang` is installed (required by [bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html)).

```shell
# Debian/Ubuntu example
sudo apt install llvm-10-dev libclang-10-dev
```

If your PHP is not globally installed, you can set:

```bash
export PHP_CONFIG=<path-to-php-config>
```

## 2) Create a new extension crate

```shell
cargo new --lib http-client
cd http-client
```

Add `cdylib` in `Cargo.toml` so Cargo builds a PHP-loadable dynamic library:

```toml
[lib]
crate-type = ["cdylib"]
```

## 3) Add dependencies

```shell
cargo add phper
cargo add reqwest --features blocking --features cookies
cargo add thiserror
```

- `phper`: PHP extension framework.
- `reqwest` with `blocking`: synchronous HTTP client API, easier for first extension.
- `cookies`: enables reqwest cookie store support.
- `thiserror`: concise error definitions.

## 4) Add `build.rs`

Create `build.rs`:

```rust,no_run
fn main() {
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-undefined");
        println!("cargo:rustc-link-arg=dynamic_lookup");
    }
}
```

## 5) File layout used in this tutorial

```text
src/
  lib.rs
  errors.rs
  client.rs
  request.rs
  response.rs
```

Next chapter: define custom exception and Rust-side error mapping.
