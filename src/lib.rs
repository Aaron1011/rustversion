//! [![github]](https://github.com/dtolnay/rustversion)&ensp;[![crates-io]](https://crates.io/crates/rustversion)&ensp;[![docs-rs]](https://docs.rs/rustversion)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! This crate provides macros for conditional compilation according to rustc
//! compiler version, analogous to [`#[cfg(...)]`][cfg] and
//! [`#[cfg_attr(...)]`][cfg_attr].
//!
//! [cfg]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute
//! [cfg_attr]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
//!
//! <br>
//!
//! # Selectors
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::stable]</code></b>
//!   —<br>
//!   True on any stable compiler.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::stable(1.34)]</code></b>
//!   —<br>
//!   True on exactly the specified stable compiler.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::beta]</code></b>
//!   —<br>
//!   True on any beta compiler.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::nightly]</code></b>
//!   —<br>
//!   True on any nightly compiler or dev build.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::nightly(2019-01-01)]</code></b>
//!   —<br>
//!   True on exactly one nightly.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::since(1.34)]</code></b>
//!   —<br>
//!   True on that stable release and any later compiler, including beta and
//!   nightly.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::since(2019-01-01)]</code></b>
//!   —<br>
//!   True on that nightly and all newer ones.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::since_date(2019-01-01)]</code></b>
//!   —<br>
//!   True on all compilers built on or after that date (stable, beta, and nightly)
//!   Dev builds are considered to be built after all dates.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::before(</code></b><i>version or date</i><b><code>)]</code></b>
//!   —<br>
//!   Negative of <i>#[rustversion::since(...)]</i>.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::before_date(2019-01-01)]</code></b>
//!   —<br>
//!   Negative of <i>#[rustversion::since_date(...)]</i>.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::not(</code></b><i>selector</i><b><code>)]</code></b>
//!   —<br>
//!   Negative of any selector; for example <i>#[rustversion::not(nightly)]</i>.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::any(</code></b><i>selectors...</i><b><code>)]</code></b>
//!   —<br>
//!   True if any of the comma-separated selectors is true; for example
//!   <i>#[rustversion::any(stable, beta)]</i>.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::all(</code></b><i>selectors...</i><b><code>)]</code></b>
//!   —<br>
//!   True if all of the comma-separated selectors are true; for example
//!   <i>#[rustversion::all(since(1.31), before(1.34))]</i>.
//!   </p>
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>#[rustversion::attr(</code></b><i>selector</i><b><code>, </code></b><i>attribute</i><b><code>)]</code></b>
//!   —<br>
//!   For conditional inclusion of attributes; analogous to
//!   <code>cfg_attr</code>.
//!   </p>
//!
//! <br>
//!
//! # Use cases
//!
//! Providing additional trait impls as types are stabilized in the standard library
//! without breaking compatibility with older compilers; in this case Pin\<P\>
//! stabilized in [Rust 1.33][pin]:
//!
//! [pin]: https://blog.rust-lang.org/2019/02/28/Rust-1.33.0.html#pinning
//!
//! ```
//! # trait MyTrait {}
//! #
//! #[rustversion::since(1.33)]
//! use std::pin::Pin;
//!
//! #[rustversion::since(1.33)]
//! impl<P: MyTrait> MyTrait for Pin<P> {
//!     /* ... */
//! }
//! ```
//!
//! Similar but for language features; the ability to control alignment greater than
//! 1 of packed structs was stabilized in [Rust 1.33][packed].
//!
//! [packed]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1330-2019-02-28
//!
//! ```
//! #[rustversion::attr(before(1.33), repr(packed))]
//! #[rustversion::attr(since(1.33), repr(packed(2)))]
//! struct Six(i16, i32);
//!
//! fn main() {
//!     println!("{}", std::mem::align_of::<Six>());
//! }
//! ```
//!
//! Augmenting code with `const` as const impls are stabilized in the standard
//! library. This use of `const` as an attribute is recognized as a special case
//! by the rustversion::attr macro.
//!
//! ```
//! use std::time::Duration;
//!
//! #[rustversion::attr(since(1.32), const)]
//! fn duration_as_days(dur: Duration) -> u64 {
//!     dur.as_secs() / 60 / 60 / 24
//! }
//! ```
//!
//! <br>

extern crate proc_macro;

mod attr;
mod bound;
mod date;
mod expr;
mod time;
mod version;

use crate::attr::Then;
use crate::date::Date;
use crate::expr::Expr;
use crate::version::{Channel::*, Version};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, ItemFn, Result};

const RUSTVERSION: Version = include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[proc_macro_attribute]
pub fn stable(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("stable", args, input)
}

#[proc_macro_attribute]
pub fn beta(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("beta", args, input)
}

#[proc_macro_attribute]
pub fn nightly(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("nightly", args, input)
}

#[proc_macro_attribute]
pub fn since(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("since", args, input)
}

#[proc_macro_attribute]
pub fn since_date(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("since_date", args, input)
}

#[proc_macro_attribute]
pub fn before(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("before", args, input)
}

#[proc_macro_attribute]
pub fn before_date(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("before_date", args, input)
}

#[proc_macro_attribute]
pub fn not(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("not", args, input)
}

#[proc_macro_attribute]
pub fn any(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("any", args, input)
}

#[proc_macro_attribute]
pub fn all(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg("all", args, input)
}

fn cfg(top: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    match try_cfg(top, args, input) {
        Ok(tokens) => tokens,
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

fn try_cfg(top: &str, args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let args = TokenStream2::from(args);
    let top = Ident::new(top, Span::call_site());

    let mut full_args = quote!(#top);
    if !args.is_empty() {
        full_args.extend(quote!((#args)));
    }

    let expr: Expr = syn::parse2(full_args)?;

    if expr.eval(RUSTVERSION) {
        Ok(input)
    } else {
        Ok(TokenStream::new())
    }
}

#[proc_macro_attribute]
pub fn attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as attr::Args);

    match try_attr(args, input) {
        Ok(tokens) => tokens,
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

fn try_attr(args: attr::Args, input: TokenStream) -> Result<TokenStream> {
    if !args.condition.eval(RUSTVERSION) {
        return Ok(input);
    }

    match args.then {
        Then::Const(const_token) => {
            let mut input: ItemFn = syn::parse(input)?;
            input.sig.constness = Some(const_token);
            Ok(TokenStream::from(quote!(#input)))
        }
        Then::Attribute(then) => {
            let input = TokenStream2::from(input);
            Ok(TokenStream::from(quote! {
                #[cfg_attr(all(), #then)]
                #input
            }))
        }
    }
}
