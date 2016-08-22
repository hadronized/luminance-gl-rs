extern crate gl;
extern crate luminance;

pub mod error;
mod pixel;

#[cfg(not(any(feature = "gl33")))]
const COMPILE_ERROR: () = "No backend selected; please select one (gl33).";

#[cfg(feature = "gl33")]
pub mod gl33;
