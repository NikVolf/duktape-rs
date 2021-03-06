//! Rust interface to [Duktape][] JavaScript interpreter.  This is still
//! a work in progress!
//!
//! [Source code](https://github.com/emk/duktape-rs).
//!
//! ```
//! use duktape::{Context,Value,DuktapeResult};
//!
//! fn add_example() -> DuktapeResult<Value<'static>> {
//!     // Create a new JavaScript interpreter.  This will be automatically
//!     // cleaned up when `ctx` goes out of scope.
//!     let mut ctx = try!(Context::new());
//!
//!     // Load some code from a string.
//!     try!(ctx.eval("function add(x, y) { return x+y; }"));
//!
//!     // Call the function we defined.
//!     ctx.call("add", &[&2.0f64, &1.0f64])
//! }
//!
//! assert_eq!(Ok(Value::Number(3.0)), add_example());
//! ```
//!
//! We also have preliminary support for defining JavaScript functions
//! using Rust, but it's still too ugly to show off.
//!
//! [Duktape]: http://duktape.org/


#![warn(missing_docs)]

#[macro_use] extern crate log;
extern crate rustc_serialize;
extern crate libc;
extern crate cesu8;
#[macro_use] extern crate abort_on_panic;
extern crate duktape_sys;

pub use errors::{ErrorCode, DuktapeError, DuktapeResult};
pub use types::Value;
pub use context::{Context, Callback};

mod errors;
mod types;
mod encoder;
mod decoder;
mod context;
