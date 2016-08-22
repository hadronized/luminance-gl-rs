use gl;
use gl::types::*;
use luminance::pixel::{Format, PixelFormat, Type};

// Return the format, internal sized-format and type.
pub fn gl_pixel_format(pf: PixelFormat) -> Option<(GLenum, GLenum, GLenum)> {
  match (pf.format, pf.encoding) {
    (Format::RGB(8, 8, 8), Type::Unsigned) => Some((gl::RGB_INTEGER, gl::RGB8UI, gl::UNSIGNED_BYTE)),
    (Format::RGBA(8, 8, 8, 8), Type::Unsigned) => Some((gl::RGBA_INTEGER, gl::RGBA8UI, gl::UNSIGNED_BYTE)),
    (Format::RGB(32, 32, 32), Type::Floating) => Some((gl::RGB, gl::RGB32F, gl::FLOAT)),
    (Format::RGBA(32, 32, 32, 32), Type::Floating) => Some((gl::RGBA, gl::RGBA32F, gl::FLOAT)),
    (Format::Depth(32), Type::Floating) => Some((gl::DEPTH_COMPONENT, gl::DEPTH_COMPONENT32F, gl::FLOAT)),
    _ => panic!("unsupported pixel format")
  }
}

// Return the number of components.
pub fn pixel_components(pf: PixelFormat) -> usize {
  match pf.format {
    Format::RGB(_, _, _) => 3,
    Format::RGBA(_, _, _, _) => 4,
    Format::Depth(_) => 1,
    _ => panic!("unsupported pixel format")
  }
}
