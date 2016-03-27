use gl;
use gl::types::*;
use gl33::pixel::gl_pixel_format;
use gl33::token::GL33;
use luminance::texture::*;
use luminance::pixel::Pixel;
use std::ptr;

impl HasTexture for GL33 {
  type ATex = GLuint;

  fn new<L, D, P>(size: D::Size, mipmaps: u32, sampler: &Sampler) -> Self::ATex where L: Layerable, D: Dimensionable, P: Pixel {
    let mut texture = 0;

    unsafe {
      let target = to_target(L::layering(), D::dim());

      gl::GenTextures(1, &mut texture);

      gl::BindTexture(target, texture);

      set_texture_levels(target, mipmaps);
      apply_sampler_to_texture(target, sampler);
      create_texture_storage::<L, D, P>(size, mipmaps);

      gl::BindTexture(target, 0);
    }

    texture
  }

  fn free(tex: &mut Self::ATex) {
    unsafe { gl::DeleteTextures(1, tex) }
  }

  fn clear<P>(tex: &Self::ATex, pixel: &P::Encoding) where P: Pixel {
  }

  fn upload<P>(tex: &Self::ATex, texels: &Vec<P::Encoding>) where P: Pixel {
  }
}

fn to_target(l: Layering, d: Dim) -> GLenum {
  match l {
    Layering::Flat => match d {
      Dim::DIM1 => gl::TEXTURE_1D,
      Dim::DIM2 => gl::TEXTURE_2D,
      Dim::DIM3 => gl::TEXTURE_3D,
      Dim::Cubemap => gl::TEXTURE_CUBE_MAP
    },
    Layering::Layered => match d {
      Dim::DIM1 => gl::TEXTURE_1D_ARRAY,
      Dim::DIM2 => gl::TEXTURE_2D_ARRAY,
      Dim::DIM3 => panic!("3D textures array not supported"),
      Dim::Cubemap => gl::TEXTURE_CUBE_MAP_ARRAY
    }
  }
}

fn create_texture_storage<L, D, P>(size: D::Size, mipmaps: u32) -> Option<String> where L: Layerable, D: Dimensionable, P: Pixel {
  let pf = P::pixel_format();

  match gl_pixel_format(pf) {
    Some((format, iformat, encoding)) => {
      match (L::layering(), D::dim()) {
        // 1D texture
        (Layering::Flat, Dim::DIM1) => {
          create_texture1D_storage(format, iformat, encoding, D::width(&size), mipmaps);
          None
        },
        // 2D texture
        (Layering::Flat, Dim::DIM2) => {
          create_texture2D_storage(format, iformat, encoding, D::width(&size), D::height(&size), mipmaps);
          None
        },
        // 3D texture
        (Layering::Flat, Dim::DIM3) => {
          create_texture3D_storage(format, iformat, encoding, D::width(&size), D::height(&size), D::depth(&size), mipmaps);
          None
        },
        // cubemap
        (Layering::Flat, Dim::Cubemap) => {
          create_cubemap_storage(format, iformat, encoding, D::width(&size), mipmaps);
          None
        },
        _ => Some(String::from("unsupported texture type"))
      }
    },
    None => Some(String::from("wrong pixel format"))
  }
}

fn create_texture1D_storage(format: GLenum, iformat: GLenum, encoding: GLenum, w: u32, mipmaps: u32) {
  for level in 0..mipmaps {
    let w = w / 2u32.pow(level);

    unsafe { gl::TexImage1D(gl::TEXTURE_1D, level as GLint, iformat as GLint, w as GLint, 0, format, encoding, ptr::null()) };
  }
}

fn create_texture2D_storage(format: GLenum, iformat: GLenum, encoding: GLenum, w: u32, h: u32, mipmaps: u32) {
  for level in 0..mipmaps {
    let div = 2u32.pow(level);
    let w = w / div;
    let h = h / div;

    unsafe { gl::TexImage2D(gl::TEXTURE_2D, level as GLint, iformat as GLint, w as GLint, h as GLint, 0, format, encoding, ptr::null()) };
  }
}

fn create_texture3D_storage(format: GLenum, iformat: GLenum, encoding: GLenum, w: u32, h: u32, d: u32, mipmaps: u32) {
  for level in 0..mipmaps {
    let div = 2u32.pow(level);
    let w = w / div;
    let h = h / div;
    let d = d / div;

    unsafe { gl::TexImage3D(gl::TEXTURE_3D, level as GLint, iformat as GLint, w as GLint, h as GLint, d as GLint, 0, format, encoding, ptr::null()) };
  }
}

fn create_cubemap_storage(format: GLenum, iformat: GLenum, encoding: GLenum, s: u32, mipmaps: u32) {
  for level in 0..mipmaps {
    let s = s / 2u32.pow(level);

    unsafe { gl::TexImage2D(gl::TEXTURE_CUBE_MAP, level as GLint, iformat as GLint, s as GLint, s as GLint, 0, format, encoding, ptr::null()) };
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
