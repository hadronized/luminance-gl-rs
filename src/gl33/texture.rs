use gl;
use gl::types::*;
use gl33::pixel::gl_pixel_format;
use gl33::token::GL33;
use luminance::texture::{self, DepthComparison, Dim, Dimensionable, Filter, HasTexture, Layerable,
                         Layering, Sampler, Wrap, dim_capacity};
use luminance::pixel::{Pixel, PixelFormat};
use std::os::raw::c_void;
use std::ptr;

pub type Texture<L, D, P> = texture::Texture<GL33, L, D, P>;

impl HasTexture for GL33 {
  type ATexture = GLuint;

  fn new_texture<L, D, P>(size: D::Size, mipmaps: u32, sampler: &Sampler) -> Self::ATexture
      where L: Layerable,
            D: Dimensionable,
            D::Size: Copy,
            P: Pixel {
    let mut texture = 0;

    unsafe {
      let target = to_target(L::layering(), D::dim());

      gl::GenTextures(1, &mut texture);

      gl::BindTexture(target, texture);
      create_texture::<L, D>(target, size, mipmaps, P::pixel_format(), sampler);
      gl::BindTexture(target, 0);
    }

    texture
  }

  fn free(tex: &mut Self::ATexture) {
    unsafe { gl::DeleteTextures(1, tex) }
  }

  fn clear_part<L, D, P>(texture: &Self::ATexture, gen_mipmaps: bool, off: D::Offset, size: D::Size, pixel: P::Encoding)
      where L: Layerable, D: Dimensionable, D::Offset: Copy, D::Size: Copy, P: Pixel, P::Encoding: Copy {
    Self::upload_part::<L, D, P>(texture, gen_mipmaps, off, size, &vec![pixel; dim_capacity::<D>(size) as usize])
  }

  fn upload_part<L, D, P>(texture: &Self::ATexture, gen_mipmaps: bool, off: D::Offset, size: D::Size, texels: &Vec<P::Encoding>)
      where L: Layerable, D::Offset: Copy, D::Size: Copy, D: Dimensionable, P: Pixel {
    let target = to_target(L::layering(), D::dim());

    unsafe {
      gl::BindTexture(target, *texture);

      upload_texels::<L, D, P>(target, off, size, texels);

      if gen_mipmaps {
        gl::GenerateMipmap(target);
      }

      gl::BindTexture(target, 0);
    }
  }
}

pub fn create_texture<L, D>(target: GLenum, size: D::Size, mipmaps: u32, pf: PixelFormat, sampler: &Sampler)
    where L: Layerable,
          D: Dimensionable,
          D::Size: Copy {
  set_texture_levels(target, mipmaps);
  apply_sampler_to_texture(target, sampler);
  create_texture_storage::<L, D>(size, mipmaps, pf);
}

pub fn to_target(l: Layering, d: Dim) -> GLenum {
  match l {
    Layering::Flat => match d {
      Dim::Dim1 => gl::TEXTURE_1D,
      Dim::Dim2 => gl::TEXTURE_2D,
      Dim::Dim3 => gl::TEXTURE_3D,
      Dim::Cubemap => gl::TEXTURE_CUBE_MAP
    },
    Layering::Layered => match d {
      Dim::Dim1 => gl::TEXTURE_1D_ARRAY,
      Dim::Dim2 => gl::TEXTURE_2D_ARRAY,
      Dim::Dim3 => panic!("3D textures array not supported"),
      Dim::Cubemap => gl::TEXTURE_CUBE_MAP_ARRAY
    }
  }
}

fn create_texture_storage<L, D>(size: D::Size, mipmaps: u32, pf: PixelFormat) -> Option<String>
    where L: Layerable,
          D: Dimensionable,
          D::Size: Copy {
  match gl_pixel_format(pf) {
    Some((format, iformat, encoding)) => {
      match (L::layering(), D::dim()) {
        // 1D texture
        (Layering::Flat, Dim::Dim1) => {
          create_texture_1d_storage(format, iformat, encoding, D::width(size), mipmaps);
          None
        },
        // 2D texture
        (Layering::Flat, Dim::Dim2) => {
          create_texture_2d_storage(format, iformat, encoding, D::width(size), D::height(size), mipmaps);
          None
        },
        // 3D texture
        (Layering::Flat, Dim::Dim3) => {
          create_texture_3d_storage(format, iformat, encoding, D::width(size), D::height(size), D::depth(size), mipmaps);
          None
        },
        // cubemap
        (Layering::Flat, Dim::Cubemap) => {
          create_cubemap_storage(format, iformat, encoding, D::width(size), mipmaps);
          None
        },
        _ => Some(String::from("unsupported texture type"))
      }
    },
    None => Some(String::from("wrong pixel format"))
  }
}

fn create_texture_1d_storage(format: GLenum, iformat: GLenum, encoding: GLenum, w: u32, mipmaps: u32) {
  for level in 0..mipmaps {
    let w = w / 2u32.pow(level);

    unsafe { gl::TexImage1D(gl::TEXTURE_1D, level as GLint, iformat as GLint, w as GLsizei, 0, format, encoding, ptr::null()) };
  }
}

fn create_texture_2d_storage(format: GLenum, iformat: GLenum, encoding: GLenum, w: u32, h: u32, mipmaps: u32) {
  for level in 0..mipmaps {
    let div = 2u32.pow(level);
    let w = w / div;
    let h = h / div;

    unsafe { gl::TexImage2D(gl::TEXTURE_2D, level as GLint, iformat as GLint, w as GLsizei, h as GLsizei, 0, format, encoding, ptr::null()) };
  }
}

fn create_texture_3d_storage(format: GLenum, iformat: GLenum, encoding: GLenum, w: u32, h: u32, d: u32, mipmaps: u32) {
  for level in 0..mipmaps {
    let div = 2u32.pow(level);
    let w = w / div;
    let h = h / div;
    let d = d / div;

    unsafe { gl::TexImage3D(gl::TEXTURE_3D, level as GLint, iformat as GLint, w as GLsizei, h as GLsizei, d as GLsizei, 0, format, encoding, ptr::null()) };
  }
}

fn create_cubemap_storage(format: GLenum, iformat: GLenum, encoding: GLenum, s: u32, mipmaps: u32) {
  for level in 0..mipmaps {
    let s = s / 2u32.pow(level);

    unsafe { gl::TexImage2D(gl::TEXTURE_CUBE_MAP, level as GLint, iformat as GLint, s as GLsizei, s as GLsizei, 0, format, encoding, ptr::null()) };
  }
}

fn set_texture_levels(target: GLenum, mipmaps: u32) {
  unsafe {
    gl::TexParameteri(target, gl::TEXTURE_BASE_LEVEL, 0);
    gl::TexParameteri(target, gl::TEXTURE_MAX_LEVEL, mipmaps as GLint - 1);
  }
}

fn apply_sampler_to_texture(target: GLenum, sampler: &Sampler) {
  unsafe {
    gl::TexParameteri(target, gl::TEXTURE_WRAP_R, from_wrap(sampler.wrap_r) as GLint);
    gl::TexParameteri(target, gl::TEXTURE_WRAP_S, from_wrap(sampler.wrap_s) as GLint);
    gl::TexParameteri(target, gl::TEXTURE_WRAP_T, from_wrap(sampler.wrap_t) as GLint);
    gl::TexParameteri(target, gl::TEXTURE_MIN_FILTER, from_filter(sampler.minification) as GLint);
    gl::TexParameteri(target, gl::TEXTURE_MAG_FILTER, from_filter(sampler.minification) as GLint);
    match sampler.depth_comparison {
      Some(fun) => {
        gl::TexParameteri(target, gl::TEXTURE_COMPARE_FUNC, from_depth_comparison(fun) as GLint);
        gl::TexParameteri(target, gl::TEXTURE_COMPARE_MODE, gl::COMPARE_REF_TO_TEXTURE as GLint);
      },
      None => {
        gl::TexParameteri(target, gl::TEXTURE_COMPARE_MODE, gl::NONE as GLint);
      }
    }
  }
}

fn from_wrap(wrap: Wrap) -> GLenum {
  match wrap {
    Wrap::ClampToEdge => gl::CLAMP_TO_EDGE,
    Wrap::Repeat => gl::REPEAT,
    Wrap::MirroredRepeat => gl::MIRRORED_REPEAT
  }
}

fn from_filter(filter: Filter) -> GLenum {
  match filter {
    Filter::Nearest => gl::NEAREST,
    Filter::Linear => gl::LINEAR
  }
}

fn from_depth_comparison(fun: DepthComparison) -> GLenum {
  match fun {
    DepthComparison::Never => gl::NEVER,
    DepthComparison::Always => gl::ALWAYS,
    DepthComparison::Equal => gl::EQUAL,
    DepthComparison::NotEqual => gl::NOTEQUAL,
    DepthComparison::Less => gl::LESS,
    DepthComparison::LessOrEqual => gl::LEQUAL,
    DepthComparison::Greater => gl::GREATER,
    DepthComparison::GreaterOrEqual => gl::GEQUAL
  }
}


fn upload_texels<L, D, P>(target: GLenum, off: D::Offset, size: D::Size, texels: &Vec<P::Encoding>)
    where L: Layerable,
          D: Dimensionable,
          D::Offset: Copy,
          D::Size: Copy,
          P: Pixel {
  let pf = P::pixel_format();

  match gl_pixel_format(pf) {
    Some((format, _, encoding)) => {
      match L::layering() {
        Layering::Flat => {
          match D::dim() {
            Dim::Dim1 => unsafe { gl::TexSubImage1D(target, 0, D::x_offset(off) as GLint, D::width(size) as GLsizei, format, encoding, texels.as_ptr() as *const c_void) },
            Dim::Dim2 => unsafe { gl::TexSubImage2D(target, 0, D::x_offset(off) as GLint, D::y_offset(off) as GLint, D::width(size) as GLsizei, D::height(size) as GLsizei, format, encoding, texels.as_ptr() as *const c_void) },
            Dim::Dim3 => unsafe { gl::TexSubImage3D(target, 0, D::x_offset(off) as GLint, D::y_offset(off) as GLint, D::z_offset(off) as GLint, D::width(size) as GLsizei, D::height(size) as GLsizei, D::depth(size) as GLsizei, format, encoding, texels.as_ptr() as *const c_void) },
            Dim::Cubemap => unsafe { gl::TexSubImage3D(target, 0, D::x_offset(off) as GLint, D::y_offset(off) as GLint, (gl::TEXTURE_CUBE_MAP_POSITIVE_X + D::z_offset(off)) as GLint, D::width(size) as GLsizei, D::width(size) as GLsizei, 1, format, encoding, texels.as_ptr() as *const c_void) }
          }
        },
        Layering::Layered => panic!("Layering::Layered not implemented yet")
      }
    },
    None => panic!("unknown pixel format")
  }
}
