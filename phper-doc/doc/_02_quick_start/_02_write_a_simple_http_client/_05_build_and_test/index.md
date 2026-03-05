# Build and test

This chapter verifies the extension from both runtime usage and integration tests.

## 1) Build extension

```shell
cargo build
```

You should get dynamic library output such as:

```text
target/debug/libhttp_client.so
```

## 2) Run quick PHP script

Create `http-client.php`:

```php
<?php

use HttpClient\HttpClientBuilder;
use HttpClient\HttpClientException;

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

try {
    $client->get("file:///")->send();
    throw new AssertionError("no throw exception");
} catch (HttpClientException $e) {
    echo "caught expected exception\n";
}
```

Run it:

```shell
php -d "extension=target/debug/libhttp_client.so" http-client.php
```

## 3) Run example tests

`examples/http-client/tests/integration.rs` starts a local PHP server and runs PHP assertions against your compiled extension.

```shell
cargo test --release
```

## 4) Optional install into PHP extension dir

```shell
cp target/release/libhttp_client.so `${PHP_CONFIG:=php-config} --extension-dir`
```

Then enable extension from your `php.ini`.

## Troubleshooting checklist

- `bindgen` failures: verify `libclang` and `php-config`.
- `undefined symbol` on macOS: make sure `build.rs` link args are present.
- network request failed: first verify target URL is reachable from your environment.

You now have a complete object-oriented HTTP client extension implemented with PHPER.
