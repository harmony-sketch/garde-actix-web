//! Actix-web wrapper for [garde](https://github.com/jprochazk/garde), a Rust validation library.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! garde = "0.14"
//! actix-web-garde = "0.1.0"
//! ```
//!
//! # Usage example
//!
//! Simply use `actix-web-garde` exposed types as a drop in for actix types.
//!
//! Your types must implement `Validate` from `garde`. Validation happens during actix's `FromRequest` invocation.
//!
//! If payload is invalid, a 400 error is returned (404 for Path).
//!
//! Custom error handling can be implemented with an extractor config (`actix_web_garde::web::QueryConfig` in place of `actix_web::web::QueryConfig` for example).
//!
//! ```rust
//! use actix_web::HttpResponse;
//! // instead of actix_web::web::Path
//! use actix_web_garde::web::Path;
//! use garde::Validate;
//!
//! #[derive(Validate)]
//! struct MyStruct<'a> {
//!   #[garde(ascii, length(min=3, max=25))]
//!   username: &'a str,
//! }
//!
//! fn test(id: Path<MyStruct>) -> HttpResponse {
//!   todo!()
//! }
//! ```
//!
//! ⚠️ When using `garde` [custom validation](https://github.com/jprochazk/garde#custom-validation), the `Context` type needs to implement `Default` which is not required by `garde`.
//!
//! # Feature flags
//!
//! | name       | description                                                   | extra dependencies                                                                           |
//! |------------|---------------------------------------------------------------|----------------------------------------------------------------------------------------------|
//! | `serde_qs` | Enables the usage of `garde` for `serde_qs::actix::QsQuery<T>` | [`serde_qs`](https://crates.io/crates/serde_qs)                                      |

#![forbid(unsafe_code)]
use actix_web::web::Data;
use actix_web::HttpRequest;
use garde::Validate;

pub mod error;
pub mod web;

fn validate_for_request<T>(data: T, req: &HttpRequest) -> Result<T, error::Error>
where
  T: Validate + 'static,
  T::Context: Default,
{
  let context = req
    .app_data::<T::Context>()
    .or_else(|| req.app_data::<Data<T::Context>>().map(|d| d.as_ref()));

  match context {
    None => data.validate(&T::Context::default()).map(|_| data).map_err(Into::into),
    Some(ctx) => data.validate(ctx).map(|_| data).map_err(Into::into),
  }
}