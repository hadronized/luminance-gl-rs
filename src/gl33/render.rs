use gl;
use gl::types::*;
use gl33::token::GL33;
use luminance::blending;
use luminance::framebuffer::{ColorSlot, DepthSlot};
use luminance::render::{self, HasFrameCommand};
use luminance::texture::{Dimensionable, Layerable};

pub type FrameCommand<'a, L, D, CS, DS> = render::FrameCommand<'a, GL33, L, D, CS, DS>;
pub type ShadingCommand<'a> = render::ShadingCommand<'a, GL33>;
pub type RenderCommand = render::RenderCommand<GL33>;

impl HasFrameCommand for GL33 {
  fn run_frame_command<L, D, CS, DS>(cmd: render::FrameCommand<Self, L, D, CS, DS>)
    where L: Layerable,
          D: Dimensionable,
          D::Size: Copy,
          CS: ColorSlot<Self, L, D>,
          DS: DepthSlot<Self, L, D> {
    unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, cmd.framebuffer.repr.handle) };

    for shading_cmd in cmd.shading_commands {
      unsafe { gl::UseProgram(shading_cmd.program) };

      (shading_cmd.update)();

      for render_cmd in shading_cmd.render_commands {
        set_blending(render_cmd.blending);
        set_depth_test(render_cmd.depth_test);
        (render_cmd.update)();
        (render_cmd.tessellation)(render_cmd.rasterization_size, render_cmd.instances);
      }
    }
  }
}

fn set_blending(blending: Option<(blending::Equation, blending::Factor, blending::Factor)>) {
  match blending {
    Some((equation, src_factor, dest_factor)) => {
      unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendEquation(from_blending_equation(equation));
        gl::BlendFunc(from_blending_factor(src_factor), from_blending_factor(dest_factor));
      }
    },
    None => {
      unsafe { gl::Disable(gl::BLEND) };
    }
  }
}

fn from_blending_equation(equation: blending::Equation) -> GLenum {
  match equation {
    blending::Equation::Additive => gl::FUNC_ADD,
    blending::Equation::Subtract => gl::FUNC_SUBTRACT,
    blending::Equation::ReverseSubtract => gl::FUNC_REVERSE_SUBTRACT,
    blending::Equation::Min => gl::MIN,
    blending::Equation::Max => gl::MAX
  }
}

fn from_blending_factor(factor: blending::Factor) -> GLenum {
  match factor {
    blending::Factor::One => gl::ONE,
    blending::Factor::Zero => gl::ZERO,
    blending::Factor::SrcColor => gl::SRC_COLOR,
    blending::Factor::NegativeSrcColor => gl::ONE_MINUS_SRC_COLOR,
    blending::Factor::DestColor => gl::DST_COLOR,
    blending::Factor::NegativeDestColor => gl::ONE_MINUS_DST_COLOR,
    blending::Factor::SrcAlpha => gl::SRC_ALPHA,
    blending::Factor::NegativeSrcAlpha => gl::ONE_MINUS_SRC_ALPHA,
    blending::Factor::DstAlpha => gl::DST_ALPHA,
    blending::Factor::NegativeDstAlpha => gl::ONE_MINUS_DST_ALPHA,
    blending::Factor::SrcAlphaSaturate => gl::SRC_ALPHA_SATURATE
  }
}

fn set_depth_test(test: bool) {
  unsafe {
    if test {
      gl::Enable(gl::DEPTH_TEST);
    } else {
      gl::Disable(gl::DEPTH_TEST);
    }
  }
}
