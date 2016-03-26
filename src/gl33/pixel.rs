use gl;
use gl::types::*;
use luminance::pixel::{Format, PixelFormat, Type};

// Return the format and internal sized-format.
pub fn gl_pixel_format(pf: &PixelFormat) -> Option<(GLenum, GLenum)> {
  match (pf.format, pf.encoding) {
    (Format::RGB(8, 8, 8), Type::Unsigned) => Some((gl::RGB_INTEGER, gl::RGB8UI)),
    (Format::RGBA(8, 8, 8, 8), Type::Unsigned) => Some((gl::RGBA_INTEGER, gl::RGBA8UI)),
    (Format::RGB(32, 32, 32), Type::Floating) => Some((gl::RGB, gl::RGB32F)),
    (Format::RGBA(32, 32, 32, 32), Type::Floating) => Some((gl::RGBA, gl::RGBA32F)),
    (Format::Depth(32), Type::Floating) => Some((gl::DEPTH_COMPONENT, gl::DEPTH_COMPONENT32F)),
    _ => None
  }
}

//RGB32F
//RGBA32F
//Depth32F
