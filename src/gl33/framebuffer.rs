use gl;
use gl::types::*;
use gl33::texture::{GLTexture, create_texture, to_target};
use gl33::token::GL33;
use luminance::framebuffer::{self, ColorSlot, DepthSlot, FramebufferError, HasFramebuffer};
use luminance::texture::{Dimensionable, Layerable};
use std::default::Default;

pub type Framebuffer<L, D, CS, DS> = framebuffer::Framebuffer<GL33, L, D, CS, DS>;
pub type Slot<L, D, P> = framebuffer::Slot<GL33, L, D, P>;

pub struct GLFramebuffer {
  pub handle: GLuint,
  pub renderbuffer: Option<GLuint>,
  pub w: u32,
  pub h: u32,
}

impl HasFramebuffer for GL33 {
  type Framebuffer = GLFramebuffer;

  fn new_framebuffer<L, D, CS, DS>(size: D::Size, mipmaps: usize) -> Result<(Self::Framebuffer, Vec<Self::ATexture>, Option<Self::ATexture>), FramebufferError>
    where L: Layerable,
          D: Dimensionable,
          D::Size: Copy,
          CS: ColorSlot<Self, L, D>,
          DS: DepthSlot<Self, L, D> {
    let mut framebuffer: GLuint = 0;
    let color_formats = CS::color_formats();
    let depth_format = DS::depth_format();
    let target = to_target(L::layering(), D::dim());
    let mut textures: Vec<GLuint> = vec![0; (color_formats.len() + if depth_format.is_some() { 1 } else { 0 })]; // FIXME: remove that (inference)
    let mut depth_texture: Option<GLuint> = None;
    let mut depth_renderbuffer: Option<GLuint> = None;

    unsafe {
      gl::GenFramebuffers(1, &mut framebuffer);

      gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);

      // generate all the required textures with the correct formats
      gl::GenTextures((textures.len()) as GLint, textures.as_mut_ptr());

      // color textures
      if color_formats.is_empty() {
        gl::DrawBuffer(gl::NONE);
      } else {
        for (i, (format, texture)) in color_formats.iter().zip(&textures).enumerate() {
          gl::BindTexture(target, *texture);
          create_texture::<L, D>(target, size, mipmaps, *format, &Default::default());
          gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0 + i as GLenum, *texture, 0);
        }
      }

      // depth texture, if exists
      if let Some(format) = depth_format {
        let texture = textures.pop().unwrap();

        gl::BindTexture(target, texture);
        create_texture::<L, D>(target, size, mipmaps, format, &Default::default());
        gl::FramebufferTexture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, texture, 0);

        depth_texture = Some(texture);
      } else {
        let mut renderbuffer: GLuint = 0;

        gl::GenRenderbuffers(1, &mut renderbuffer);
        gl::BindRenderbuffer(gl::RENDERBUFFER, renderbuffer);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT32F, D::width(size) as GLsizei, D::height(size) as GLsizei);
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, renderbuffer);

        depth_renderbuffer = Some(renderbuffer);
      }

      gl::BindTexture(target, 0);

      let mut gl_framebuffer = GLFramebuffer {
        handle: framebuffer,
        renderbuffer: depth_renderbuffer,
        w: D::width(size),
        h: D::height(size)
      };

      match get_status() {
        Some(incomplete) => {
          gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

          Self::free_framebuffer(&mut gl_framebuffer);

          Err(FramebufferError::Incomplete(incomplete))
        },
        None => {
          gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

          let textures = textures.into_iter().map(|t| GLTexture::new(t, target)).collect();
          let depth_texture = depth_texture.map(|t| GLTexture::new(t, target));
          Ok((gl_framebuffer, textures, depth_texture))
        }
      }
    }
  }

  fn free_framebuffer(framebuffer: &mut Self::Framebuffer) {
    unsafe {
      if let Some(renderbuffer) = framebuffer.renderbuffer {
        gl::DeleteRenderbuffers(1, &renderbuffer);
      }

      if framebuffer.handle != 0 {
        gl::DeleteFramebuffers(1, &framebuffer.handle);
      }
    }
  }

  fn default_framebuffer<D>(size: D::Size) -> Self::Framebuffer
      where D: Dimensionable,
            D::Size: Copy {
    GLFramebuffer {
      handle: 0,
      renderbuffer: None,
      w: D::width(size),
      h: D::height(size)
    }
  }
}

fn get_status() -> Option<String> {
  let status = unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) };

  match status {
    gl::FRAMEBUFFER_COMPLETE => None,
    gl::FRAMEBUFFER_UNDEFINED => Some(String::from("framebuffer undefined")),
    gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => Some(String::from("incomplete attachment")),
    gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => Some(String::from("incomplete missing attachment")),
    gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => Some(String::from("incomplete draw buffer")),
    gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => Some(String::from("incomplete read buffer")),
    gl::FRAMEBUFFER_UNSUPPORTED => Some(String::from("unsupported")),
    gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => Some(String::from("incomplete multisample")),
    gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => Some(String::from("incomplete layer targets")),
    _ => Some(String::from("unknown"))
  }
}
