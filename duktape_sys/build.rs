extern crate gcc;

use std::default::Default;
use std::env;
use std::path::Path;

fn main() {
    // Make sure we get a thread-safe build.  Without this, duktape refuses
    // to set DUK_USE_VARIADIC_MACROS and falls back to global variables.

	gcc::Config::new()
		.file("duktape/src/duktape.c")
		.file("src/glue.c")
		.include("duktape/src")
		.flag("-std=c99")
		.compile("libduktape.a");

}
