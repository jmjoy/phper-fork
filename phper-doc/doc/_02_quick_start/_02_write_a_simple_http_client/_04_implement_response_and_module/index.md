# Implement response and module registration

This chapter completes the remaining pieces:

- `HttpClient\\Response`
- module bootstrap (`src/lib.rs`)

## 1) Implement `Response` class

From `src/response.rs`:

```rust,ignore
use crate::errors::HttpClientError;
use phper::{
    arrays::{InsertKey, ZArray},
    classes::{ClassEntity, StateClass, Visibility},
    values::ZVal,
};
use reqwest::blocking::Response;
use std::mem::take;

pub type ResponseClass = StateClass<Option<Response>>;

pub const RESPONSE_CLASS_NAME: &str = "HttpClient\\Response";

pub fn make_response_class() -> ClassEntity<Option<Response>> {
    let mut class =
        ClassEntity::<Option<Response>>::new_with_default_state_constructor(RESPONSE_CLASS_NAME);

    class.add_method("body", Visibility::Public, |this, _arguments| {
        let response = take(this.as_mut_state());
        let response = response.ok_or(HttpClientError::ResponseHadRead)?;
        let body = response.bytes().map_err(HttpClientError::Reqwest)?;
        Ok::<_, phper::Error>(body.to_vec())
    });

    class.add_method("status", Visibility::Public, |this, _arguments| {
        let response =
            this.as_state()
                .as_ref()
                .ok_or_else(|| HttpClientError::ResponseAfterRead {
                    method_name: "status".to_owned(),
                })?;

        Ok::<_, HttpClientError>(response.status().as_u16() as i64)
    });

    class.add_method("headers", Visibility::Public, |this, _arguments| {
        let response =
            this.as_state()
                .as_ref()
                .ok_or_else(|| HttpClientError::ResponseAfterRead {
                    method_name: "headers".to_owned(),
                })?;
        let headers_map = response
            .headers()
            .iter()
            .fold(ZArray::new(), |mut acc, (key, value)| {
                let arr = acc.entry(key.as_str()).or_insert(ZVal::from(ZArray::new()));
                arr.as_mut_z_arr()
                    .unwrap()
                    .insert(InsertKey::NextIndex, ZVal::from(value.as_bytes()));
                acc
            });
        Ok::<_, HttpClientError>(headers_map)
    });

    class
}
```

Behavior details:

- `body()` consumes the underlying response stream and returns bytes.
- Calling `body()` twice throws `ResponseHadRead`.
- Calling `status()`/`headers()` after `body()` throws `ResponseAfterRead`.

This matches stream semantics in many HTTP clients and avoids accidentally reading an already consumed body.

## 2) Register all classes in module entry

From `src/lib.rs`:

```rust,ignore
use crate::{
    client::{make_client_builder_class, make_client_class},
    errors::make_exception_class,
    request::make_request_builder_class,
    response::make_response_class,
};
use phper::{modules::Module, php_get_module};

pub mod client;
pub mod errors;
pub mod request;
pub mod response;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_class(make_exception_class());
    let response_class = module.add_class(make_response_class());
    let request_builder_class = module.add_class(make_request_builder_class(response_class));
    let client_class = module.add_class(make_client_class(request_builder_class));
    module.add_class(make_client_builder_class(client_class));

    module
}
```

The registration order is important because each class builder depends on previously registered `StateClass<T>` handles.

Next chapter: build and run verification.
