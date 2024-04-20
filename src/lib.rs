#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

//! ## Important note
//!
//! This project is currently only an experiment. At this point, it is highly uncertain if the project will be continued or dropped soon.See the on-going discussion on literal prefixes [here](https://internals.rust-lang.org/t/syntactic-sugar-for-str-to-string-conversion)
//!
//! ## Description
//!
//! Prefixes delivers various prefix-like proc macro attributes for literals to easily create common types.
//!
//! Quick example:
//!
//! ```
//! #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//!
//! use prefixes::{f, ms, P, s};
//!
//! fn _deep_thought2() -> String {
//!     let answer_path = #[P]"./answer.txt";
//!     let answer = if answer_path.exists() {
//!         std::fs::read_to_string(&answer_path).unwrap()
//!     } else {
//!         std::thread::sleep(#[s]6 + #[ms]9);
//!         #[f]"42"
//!     };
//!     
//!     #[f]"Answer to the Ultimate Question of Life, the Universe, and Everything = {answer}"
//! }
//! ```
//!
//! ## Usage
//!
//! To use prefixes, first add it to your project in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! prefixes = "0.1.0"
//! ```
//!
//! Then enable `stmt_expr_attributes` and `proc_macro_hygiene` features (this is possible only on nightly Rust):
//!
//! ```
//! #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! ```
//!
//! Finally, use prefixes that you need, e.g.
//!
//! ```
//! #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! use prefixes::f;
//!
//! fn greeting(name: &str) -> String {
//!     #[f]"Welcome, {name}!"
//! }
//! ```
//!
//! ## Prefixes
//!
//! ### Prefix #[f]
//!
//! Build formatted string from a string literal using the `format!` macro.
//!
//! ```
//! #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! use prefixes::f;
//!
//! let n = 2137;
//! let s1 = format!("n = {n}");
//! let s2 = #[f]"n = {n}";
//!
//! assert_eq!(s1, s2);
//! ```
//!
//! Might be also useful for creating owned strings, e.g.
//!
//! ```
//! #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! use prefixes::f;
//!
//! let s1 = #[f]"2137";
//! let s2 = "2137".to_string();
//!
//! assert_eq!(s1, s2);
//! ```
//!
//! ### Prefixes #[ms], #[s]
//!
//! Build builds `std::time::Duration` from an integer literal using `from_millis` (`#[ms]`) or `from_secs[_f32|_f64]` methods (`#[s]`).
//!
//! ```
//! #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! use std::time::Duration;
//! use prefixes::{ms, s};
//!
//! let d1 = Duration::from_millis(1000);
//! let d2 = #[ms]1000;
//!
//! assert_eq!(d1, d2);
//!
//! let d3 = Duration::from_secs(2);
//! let d4 = #[s]2;
//!
//! assert_eq!(d3, d4);
//!
//! let d5 = Duration::from_secs_f32(3.0f32);
//! let d6 = #[s]3.0f32;
//!
//! assert_eq!(d5, d6);
//!
//! let d7 = Duration::from_secs_f64(4.0f64);
//! let d8 = #[s]4.0f64;
//!
//! assert_eq!(d7, d8);
//! ```
//!
//! ### Prefixes #[os], #[OS]
//!
//! Build `OsStr` (`#[os]`) or `OsString` (`#[OS]`) from a string literal. Additionally, `#[OS]` supports string interpolation like `#[f]`.
//!
//! ```
//! #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! use std::ffi::{OsStr, OsString};
//! use prefixes::{os, OS};
//!
//! let os1 = OsStr::new("foo");
//! let os2 = #[os]"foo";
//!
//! assert_eq!(os1, os2);
//!
//! let n = 42;
//! let os3 = OsString::from(format!("n = 42"));
//! let os4 = #[OS]"n = 42";
//!
//! assert_eq!(os3, os4);
//! ```
//!
//! ### Prefixes #[p], #[P]
//!
//! Build `Path` (`#[p]`) or `PathBuf` (`#[P]`) from a string literal. Additionally, `#[P]` supports string interpolation.
//!
//! ```
//! #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! use std::path::{Path, PathBuf};
//! use prefixes::{p, P};
//!
//! let p1 = Path::new("/foo");
//! let p2 = #[p]"/foo";
//!
//! assert_eq!(p1, p2);
//!
//! let ext = "txt";
//! let p3 = PathBuf::from(format!("/foo.{ext}"));
//! let p4 = #[P]"/foo.{ext}";
//!
//! assert_eq!(p3, p4);
//! ```
//!
//! ### Prefixes #[re], #[RE]
//!
//! Build [Regex](https:://crates.io/crates/regex) from a string literal. Additionally, `#[RE]` calls `.unwrap()` on the result. Works only if `regex` crate is included in the dependencies. Doesn't require explicit `use regex::Regex`.
//!
//! ```
//! #![feature(stmt_expr_attributes, proc_macro_hygiene)]
//! use regex::Regex;
//! use prefixes::{re, RE};
//!
//! let re1 = Regex::new("1|2");
//! let re2 = #[re]"1|2";
//!
//! assert_eq!(format!("{re1:?}"), format!("{re2:?}"));
//!
//! let re3 = Regex::new("[A-Z]").unwrap();
//! let re4 = #[RE]"[A-Z]";
//!
//! assert_eq!(format!("{re3:?}"), format!("{re4:?}"));
//! ```
//!
//! ## License
//!
//! This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

pub use prefixes_f::f;
pub use prefixes_ms::ms;
pub use prefixes_os::os;
pub use prefixes_p::p;
pub use prefixes_re::re;
pub use prefixes_s::s;
pub use prefixes_uppercase_os::OS;
pub use prefixes_uppercase_p::P;
pub use prefixes_uppercase_re::RE;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn f() {
//         let n = 2137;
//         let s1 = format!("n = {n}");
//         let s2 = #[f]"n = {n}";

//         assert_eq!(s1, s2);

//         let s3 = #[f]"2137";
//         let s4 = "2137".to_string();
//         assert_eq!(s3, s4);
//     }

//     #[test]
//     fn s_ms() {
//         use std::time::Duration;

//         let d1 = Duration::from_millis(1000);
//         let d2 = #[ms]1000;

//         assert_eq!(d1, d2);

//         let d3 = Duration::from_secs(2);
//         let d4 = #[s]2;

//         assert_eq!(d3, d4);

//         let d5 = Duration::from_secs_f32(3f32);
//         let d6 = #[s]3.0f32;

//         assert_eq!(d5, d6);

//         let d7 = Duration::from_secs_f64(4f64);
//         let d8 = #[s]4.0f64;
//         assert_eq!(d7, d8);
//     }

//     #[test]
//     fn os() {
//         let os1 = std::ffi::OsStr::new("foo");
//         let os2 = #[os]"foo";

//         assert_eq!(os1, os2);

//         let n = 42;
//         let os3 = std::ffi::OsString::from(format!("n = 42"));
//         let os4 = #[OS]"n = 42";
//         assert_eq!(os3, os4);
//     }

//     #[test]
//     fn p() {
//         let p1 = std::path::Path::new("/foo");
//         let p2 = #[p]"/foo";

//         assert_eq!(p1, p2);

//         let ext = "txt";
//         let p3 = std::path::PathBuf::from(format!("/foo.{ext}"));
//         let p4 = #[P]"/foo.{ext}";

//         assert_eq!(p3, p4);
//     }

//     #[test]
//     fn re() {
//         use regex::Regex;

//         let re1 = Regex::new("1|2");
//         let re2 = #[re]"1|2";
//         let re3 = Regex::new("[A-Z]").unwrap();
//         let re4 = #[RE]"[A-Z]";

//         assert_eq!(format!("{re1:?}"), format!("{re2:?}"));
//         assert_eq!(format!("{re3:?}"), format!("{re4:?}"));
//     }
// }
