//! # Support for `Xml<T>` from `axum-serde`
//!
//! ## Feature
//!
//! Enable the `Xml` feature to use `Valid<Xml<T>>`.
//!
//! ## Usage
//!
//! 1. Implement `Deserialize` and `Validate` for your data type `T`.
//! 2. In your handler function, use `Valid<Xml<T>>` as some parameter's type.
//!
//! ## Example
//!
//! ```no_run
//! #[cfg(feature = "validator")]
//! mod validator_example {
//!     use axum::routing::post;
//!     use axum_serde::Xml;
//!     use axum::Router;
//!     use axum_valid::Valid;
//!     use serde::Deserialize;
//!     use validator::Validate;
//!
//!     pub fn router() -> Router {
//!         Router::new().route("/Xml", post(handler))
//!     }
//!
//!     async fn handler(Valid(Xml(parameter)): Valid<Xml<Parameter>>) {
//!         assert!(parameter.validate().is_ok());
//!         // Support automatic dereferencing
//!         println!("v0 = {}, v1 = {}", parameter.v0, parameter.v1);
//!     }
//!
//!     #[derive(Validate, Deserialize)]
//!     pub struct Parameter {
//!         #[validate(range(min = 5, max = 10))]
//!         pub v0: i32,
//!         #[validate(length(min = 1, max = 10))]
//!         pub v1: String,
//!     }
//! }
//!
//! #[cfg(feature = "garde")]
//! mod garde_example {
//!     use axum::routing::post;
//!     use axum_serde::Xml;
//!     use axum::Router;
//!     use axum_valid::Garde;
//!     use serde::Deserialize;
//!     use garde::Validate;
//!
//!     pub fn router() -> Router {
//!         Router::new().route("/Xml", post(handler))
//!     }
//!
//!     async fn handler(Garde(Xml(parameter)): Garde<Xml<Parameter>>) {
//!         assert!(parameter.validate(&()).is_ok());
//!         // Support automatic dereferencing
//!         println!("v0 = {}, v1 = {}", parameter.v0, parameter.v1);
//!     }
//!
//!     #[derive(Validate, Deserialize)]
//!     pub struct Parameter {
//!         #[garde(range(min = 5, max = 10))]
//!         pub v0: i32,
//!         #[garde(length(min = 1, max = 10))]
//!         pub v1: String,
//!     }
//! }
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! #     use std::net::SocketAddr;
//! #     use axum::Router;
//! #     use tokio::net::TcpListener;
//! #     let router = Router::new();
//! #     #[cfg(feature = "validator")]
//! #     let router = router.nest("/validator", validator_example::router());
//! #     #[cfg(feature = "garde")]
//! #     let router = router.nest("/garde", garde_example::router());
//! #     let listener = TcpListener::bind(&SocketAddr::from(([0u8, 0, 0, 0], 0u16))).await?;
//! #     axum::serve(listener, router.into_make_service())
//! #         .await?;
//! #     Ok(())
//! # }
//! ```

use crate::HasValidate;
#[cfg(feature = "validator")]
use crate::HasValidateArgs;
use axum_serde::Xml;
#[cfg(feature = "validator")]
use validator::ValidateArgs;

impl<T> HasValidate for Xml<T> {
    type Validate = T;
    fn get_validate(&self) -> &T {
        &self.0
    }
}

#[cfg(feature = "validator")]
impl<'v, T: ValidateArgs<'v>> HasValidateArgs<'v> for Xml<T> {
    type ValidateArgs = T;
    fn get_validate_args(&self) -> &Self::ValidateArgs {
        &self.0
    }
}

#[cfg(feature = "validify")]
impl<T: validify::Modify> crate::HasModify for Xml<T> {
    type Modify = T;

    fn get_modify(&mut self) -> &mut Self::Modify {
        &mut self.0
    }
}

#[cfg(feature = "validify")]
impl<T> crate::PayloadExtractor for Xml<T> {
    type Payload = T;

    fn get_payload(self) -> Self::Payload {
        self.0
    }
}

#[cfg(feature = "validify")]
impl<T: validify::Validify + validify::ValidifyPayload> crate::HasValidify for Xml<T> {
    type Validify = T;
    type PayloadExtractor = Xml<T::Payload>;
    fn from_validify(v: Self::Validify) -> Self {
        Xml(v)
    }
}
#[cfg(test)]
mod tests {
    use crate::tests::{ValidTest, ValidTestParameter};
    use axum::http::StatusCode;
    use axum_serde::Xml;
    use reqwest::RequestBuilder;
    use serde::Serialize;

    impl<T: ValidTestParameter + Serialize> ValidTest for Xml<T> {
        const ERROR_STATUS_CODE: StatusCode = StatusCode::UNPROCESSABLE_ENTITY;

        fn set_valid_request(builder: RequestBuilder) -> RequestBuilder {
            builder
                .header(reqwest::header::CONTENT_TYPE, "application/xml")
                .body(
                    quick_xml::se::to_string(&T::valid())
                        .expect("Failed to serialize parameters to xml"),
                )
        }

        fn set_error_request(builder: RequestBuilder) -> RequestBuilder {
            #[derive(Serialize, Default)]
            struct ErrorData {
                error_field: i32,
            }
            builder
                .header(reqwest::header::CONTENT_TYPE, "application/xml")
                .body(
                    quick_xml::se::to_string(&ErrorData::default())
                        .expect("Failed to serialize parameters to xml"),
                )
        }

        fn set_invalid_request(builder: RequestBuilder) -> RequestBuilder {
            builder
                .header(reqwest::header::CONTENT_TYPE, "application/xml")
                .body(
                    quick_xml::se::to_string(&T::invalid())
                        .expect("Failed to serialize parameters to xml"),
                )
        }
    }
}
