use gl;
use gl::types::*;
use gl33::texture::{create_texture, to_target};
use gl33::token::GL33;
use luminance::framebuffer::{ColorSlot, DepthSlot, FramebufferError, HasFramebuffer};
use luminance::texture::{Dimensionable, Layerable};
use std::default::Default;

pub struct GLFramebuffer {
  pub handle: GLuint,
  pub renderbuffer: Option<GLuint>
}

impl HasFramebuffer for GL33 {
  type Framebuffer = GLFramebuffer;

  fn new_framebuffer<L, D, CS, DS>(size: D::Size, mipmaps: u32) -> Result<(Self::Framebuffer, Vec<Self::ATexture>, Option<Self::ATexture>), FramebufferError>
    where L: Layerable,
          D: Dimensionable,
          D::Size: Copy,
          CS: ColorSlot<Self, L, D>,
          DS: DepthSlot<Self, L, D> {
    let mut framebuffer: GLuint = 0;
    let mut textures: GLuint = 0;
    let color_formats = CS::color_formats();
    let depth_format = DS::depth_format();

    unsafe {
      let target = to_target(L::layering(), D::dim());

      gl::GenFramebuffers(1, &mut framebuffer);

      gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);

      // generate all the required textures with the correct formats
      gl::GenTextures((color_formats.len() + if depth_format.is_some() { 1 } else { 0 }) as GLint, &mut textures);

      // color textures
      if color_formats.is_empty() {
        gl::DrawBuffer(gl::NONE);
      } else {
        for (i, format) in color_formats.iter().enumerate() {
          gl::BindTexture(target, textures + i as GLuint);
          create_texture::<L, D>(target, size, mipmaps, *format, &Default::default());
        }
      }

      // depth texture, if exists
      if let Some(format) = depth_format {
        gl::BindTexture(target, textures + color_formats.len() as GLuint);
        create_texture::<L, D>(target, size, mipmaps, format, &Default::default());
      } else {
        // TODO: create render buffer
      }

      gl::BindTexture(target, 0);
      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    Err(FramebufferError::Incomplete(String::from("TODO")))
  }

  fn free_framebuffer(framebuffer: &mut Self::Framebuffer) {
    unsafe { gl::DeleteFramebuffers(1, &framebuffer.handle) };
  }

  fn default_framebuffer() -> Self::Framebuffer {
    GLFramebuffer {
      handle: 0,
      renderbuffer: None
    }
  }
}
