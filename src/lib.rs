//! # arya.
//!
//! simple json validation. simple json repair. lightning fast.
//!
//! ## usage.
//!
//! arya contains two main classes:
//!
//! - [`JsonBuilder`](./struct.JsonBuilder.html) - a string builder for json that can repair and complete incomplete ("damaged") json.
//! - [`JsonVerifier`](./struct.JsonVerifier.html) - a fast json syntax validator.
//!
//! ## example: json validation + repair.
//!
//! ```rust
//! # use arya::JsonBuilder;
//! #
//! # fn main() {
//! #
//! let mut builder = JsonBuilder::new();
//!
//! builder.update(r#"{
//!     "name": "annie",
//!     "age": 14,
//!     "parents": {
//!         "mother": null,
//!         "broken
//! "#);
//!
//! builder.update("value");
//!
//! builder.completed_string();
//!
//! // => Ok({
//! // =>     "name": "annie",
//! // =>     "age": 14,
//! // =>     "nested": {
//! // =>         "mother": null
//! // =>     }
//! // => })
//! # }
//! ```
//!
//! # example: json validation
//!
//! ```rust
//! # use arya::JsonVerifier;
//! #
//! # fn main() {
//! #
//! let mut json = JsonVerifier::new();
//!
//! for character in r#"{ "name": "annie", "value": 1 }"#.bytes() {
//!     println!(
//!         "{} - {:?} - {:?}",
//!         character as char,
//!         json.update(character),
//!         json.status());
//! }
//!
//! //     { - Ok(()) - Continue
//! //       - Ok(()) - Continue
//! //     " - Ok(()) - Continue
//! //     n - Ok(()) - Continue
//! //     a - Ok(()) - Continue
//! //     m - Ok(()) - Continue
//! //     e - Ok(()) - Continue
//! //     " - Ok(()) - Continue
//! //     : - Ok(()) - Continue
//! //       - Ok(()) - Continue
//! //     " - Ok(()) - Continue
//! //     a - Ok(()) - Continue
//! //     n - Ok(()) - Continue
//! //     n - Ok(()) - Continue
//! //     i - Ok(()) - Continue
//! //     e - Ok(()) - Continue
//! //     " - Ok(()) - Continue
//! //     , - Ok(()) - Continue
//! //       - Ok(()) - Continue
//! //     " - Ok(()) - Continue
//! //     v - Ok(()) - Continue
//! //     a - Ok(()) - Continue
//! //     l - Ok(()) - Continue
//! //     u - Ok(()) - Continue
//! //     e - Ok(()) - Continue
//! //     " - Ok(()) - Continue
//! //     : - Ok(()) - Continue
//! //       - Ok(()) - Continue
//! //     1 - Ok(()) - Continue
//! //       - Ok(()) - Continue
//! //     } - Ok(()) - Valid
//! # }
//! ```

#![feature(
    crate_visibility_modifier,
    extern_prelude,
    in_band_lifetimes,
    nll,
)]

mod arya;
mod hina;

pub use {
    arya::*,
};
