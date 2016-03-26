use gl;
use gl::types::*;
use gl33::token::GL33;
use luminance::texture::*;
use luminance::pixel::Pixel;

impl HasTexture for GL33 {
  type ATex = GLuint;

  fn new<L, D, P>(size: D::Size, mipmaps: u32, sampler: &Sampler) -> Self::ATex where L: Layerable, D: Dimensionable, P: Pixel {
    let mut texture = 0;

    unsafe {
      let target = to_target(L::layering(), D::dim());
      gl::GenTextures(1, &mut texture);
      gl::BindTexture(target, texture);

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

fn create_texture_storage<L, D, P>(texture: GLuint, size: D::Size, mipmaps: u32)
    where L: Layerable,
          D: Dimensionable,
          P: Pixel {
  match L::layering() {
    Layering::Flat => match D::dim() {
      Dim::DIM1 => {
        for level in 0..mipmaps {

        }
      }
    }
  }
}
