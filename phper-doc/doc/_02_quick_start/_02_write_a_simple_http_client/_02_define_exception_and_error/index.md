# Define exception and error mapping

In this chapter we implement `HttpClient\\HttpClientException` and map Rust errors into PHP exceptions.

## 1) Create `src/errors.rs`

```rust
use phper::{
    classes::{ClassEntity, ClassEntry, StateClass},
    errors::{Throwable, exception_class},
};

const EXCEPTION_CLASS_NAME: &str = "HttpClient\\HttpClientException";

pub fn make_exception_class() -> ClassEntity<()> {
    let mut class = ClassEntity::new(EXCEPTION_CLASS_NAME);
    class.extends(StateClass::from_name("Exception"));
    class
}
```

This creates a user-defined exception class under namespace `HttpClient`, extending PHP's built-in `Exception`.

## 2) Define Rust error enum

```rust
#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    #[error(transparent)]
    Reqwest(reqwest::Error),

    #[error("should call '{method_name}()' before call 'body()'")]
    ResponseAfterRead { method_name: String },

    #[error("should not call 'body()' multi time")]
    ResponseHadRead,
}
```

Meaning:

- `Reqwest`: wraps low-level HTTP/network errors.
- `ResponseAfterRead`: `body()` has already consumed the response stream.
- `ResponseHadRead`: repeated `body()` read attempts.

## 3) Tell PHPER which PHP exception class to throw

```rust,ignore
impl Throwable for HttpClientError {
    fn get_class(&self) -> &ClassEntry {
        ClassEntry::from_globals(EXCEPTION_CLASS_NAME)
            .unwrap_or_else(|_| exception_class())
    }
}

impl From<HttpClientError> for phper::Error {
    fn from(e: HttpClientError) -> Self {
        phper::Error::throw(e)
    }
}
```

With this mapping:

- you can return `Result<T, HttpClientError>` from methods;
- PHP receives `HttpClient\\HttpClientException` (or fallback `Exception`) automatically.

Next chapter: implement `HttpClientBuilder`, `HttpClient`, and `RequestBuilder`.
