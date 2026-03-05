# Write a simple HTTP client

This quick start is based on the full example:
<https://github.com/phper-framework/phper/tree/master/examples/http-client>.

The example uses [reqwest](https://crates.io/crates/reqwest) to expose an object-oriented HTTP API in PHP through a Rust extension.

Target PHP API:

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

## Architecture overview

The extension exposes these PHP classes:

- `HttpClient\HttpClientBuilder`
- `HttpClient\HttpClient`
- `HttpClient\RequestBuilder`
- `HttpClient\Response`
- `HttpClient\HttpClientException`

And these Rust modules:

- `errors.rs`: exception class and Rust-to-PHP error mapping.
- `client.rs`: `HttpClientBuilder` and `HttpClient`.
- `request.rs`: `RequestBuilder::send`.
- `response.rs`: response status/headers/body handling.
- `lib.rs`: module entry and class registration.
