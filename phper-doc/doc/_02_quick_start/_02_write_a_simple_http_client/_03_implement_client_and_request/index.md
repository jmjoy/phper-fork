# Implement client builder and request builder

This chapter covers three classes:

- `HttpClient\\HttpClientBuilder`
- `HttpClient\\HttpClient`
- `HttpClient\\RequestBuilder`

## 1) `HttpClientBuilder`: wrap `reqwest::blocking::ClientBuilder`

From `src/client.rs`:

```rust,ignore
use phper::{
    alloc::ToRefOwned,
    classes::{ClassEntity, StateClass, Visibility},
    functions::Argument,
};
use reqwest::blocking::{Client, ClientBuilder};
use std::{mem::take, time::Duration};

pub type ClientClass = StateClass<Option<Client>>;

const HTTP_CLIENT_BUILDER_CLASS_NAME: &str = "HttpClient\\HttpClientBuilder";

pub fn make_client_builder_class(client_class: ClientClass) -> ClassEntity<ClientBuilder> {
    let mut class = ClassEntity::new_with_default_state_constructor(HTTP_CLIENT_BUILDER_CLASS_NAME);

    class
        .add_method("timeout", Visibility::Public, |this, arguments| {
            let ms = arguments[0].expect_long()?;
            let state = this.as_mut_state();
            let builder: ClientBuilder = take(state);
            *state = builder.timeout(Duration::from_millis(ms as u64));
            Ok::<_, phper::Error>(this.to_ref_owned())
        })
        .argument(Argument::new("ms"));

    class
        .add_method("cookie_store", Visibility::Public, |this, arguments| {
            let enable = arguments[0].expect_bool()?;
            let state = this.as_mut_state();
            let builder: ClientBuilder = take(state);
            *state = builder.cookie_store(enable);
            Ok::<_, phper::Error>(this.to_ref_owned())
        })
        .argument(Argument::new("enable"));

    class.add_method("build", Visibility::Public, move |this, _arguments| {
        let state = take(this.as_mut_state());
        let client = ClientBuilder::build(state).map_err(crate::errors::HttpClientError::Reqwest)?;
        let mut object = client_class.init_object()?;
        *object.as_mut_state() = Some(client);
        Ok::<_, phper::Error>(object)
    });

    class
}
```

Why use `take(state)`? `reqwest::blocking::ClientBuilder` methods usually consume `self` and return a new builder, so we move out old state, apply config, then write new state back.

## 2) `HttpClient`: hide constructor, expose `get` / `post`

Still in `src/client.rs`:

```rust,ignore
use crate::request::RequestBuilderClass;
use std::convert::Infallible;

pub type ClientClass = StateClass<Option<Client>>;

const HTTP_CLIENT_CLASS_NAME: &str = "HttpClient\\HttpClient";

pub fn make_client_class(
    request_builder_class: RequestBuilderClass,
) -> ClassEntity<Option<Client>> {
    let mut class =
        ClassEntity::<Option<Client>>::new_with_default_state_constructor(HTTP_CLIENT_CLASS_NAME);

    class.add_method("__construct", Visibility::Private, |_, _| {
        Ok::<_, Infallible>(())
    });

    let request_build_class_ = request_builder_class.clone();
    class
        .add_method("get", Visibility::Public, move |this, arguments| {
            let url = arguments[0].expect_z_str()?.to_str().unwrap();
            let client = this.as_state().as_ref().unwrap();
            let request_builder = client.get(url);
            let mut object = request_build_class_.init_object()?;
            *object.as_mut_state() = Some(request_builder);
            Ok::<_, phper::Error>(object)
        })
        .argument(Argument::new("url"));

    class
        .add_method("post", Visibility::Public, move |this, arguments| {
            let url = arguments[0].expect_z_str()?.to_str().unwrap();
            let client = this.as_state().as_ref().unwrap();
            let request_builder = client.post(url);
            let mut object = request_builder_class.init_object()?;
            *object.as_mut_state() = Some(request_builder);
            Ok::<_, phper::Error>(object)
        })
        .argument(Argument::new("url"));

    class
}
```

Design notes:

- `__construct` is private: users must create it through `HttpClientBuilder::build()`.
- `Option<Client>` as state: object starts empty and is initialized by builder.

## 3) `RequestBuilder`: send request and return response object

From `src/request.rs`:

```rust,ignore
use crate::{errors::HttpClientError, response::ResponseClass};
use phper::classes::{ClassEntity, StateClass, Visibility};
use reqwest::blocking::RequestBuilder;
use std::{convert::Infallible, mem::take};

pub type RequestBuilderClass = StateClass<Option<RequestBuilder>>;

pub const REQUEST_BUILDER_CLASS_NAME: &str = "HttpClient\\RequestBuilder";

pub fn make_request_builder_class(
    response_class: ResponseClass,
) -> ClassEntity<Option<RequestBuilder>> {
    let mut class = ClassEntity::<Option<RequestBuilder>>::new_with_default_state_constructor(
        REQUEST_BUILDER_CLASS_NAME,
    );

    class.add_method("__construct", Visibility::Private, |_, _| {
        Ok::<_, Infallible>(())
    });

    class.add_method("send", Visibility::Public, move |this, _arguments| {
        let state = take(this.as_mut_state());
        let response = state.unwrap().send().map_err(HttpClientError::Reqwest)?;
        let mut object = response_class.new_object([])?;
        *object.as_mut_state() = Some(response);
        Ok::<_, phper::Error>(object)
    });

    class
}
```

After this chapter, PHP can execute:

```php
$response = $client->get("https://httpbin.org/ip")->send();
```

Next chapter: implement `Response::status()`, `headers()`, and `body()`.
