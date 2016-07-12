use gl;
use gl::types::*;
use gl33::token::GL33;
use luminance::blending;
use luminance::framebuffer::{ColorSlot, DepthSlot};
use luminance::pipeline::{self, HasPipeline};
use luminance::texture::{Dimensionable, Layerable};

pub type Pipeline<'a, L, D, CS, DS> = pipeline::Pipeline<'a, GL33, L, D, CS, DS>;
pub type ShadingCommand<'a, T> = pipeline::ShadingCommand<'a, GL33, T>;
pub type RenderCommand<'a, T> = pipeline::RenderCommand<'a, GL33, T>;

impl HasPipeline for GL33 {
  fn run_pipeline<L, D, CS, DS>(cmd: &pipeline::Pipeline<Self, L, D, CS, DS>)
    where L: Layerable,
          D: Dimensionable,
          D::Size: Copy,
          CS: ColorSlot<Self, L, D>,
          DS: DepthSlot<Self, L, D> {
    let clear_color = cmd.clear_color;

    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, cmd.framebuffer.repr.handle);
      gl::Viewport(0, 0, cmd.framebuffer.repr.w as GLint, cmd.framebuffer.repr.h as GLint);
      gl::ClearColor(clear_color[0], clear_color[1], clear_color[2], clear_color[3]);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    for shading_cmd in &cmd.shading_commands {
      shading_cmd.run_shading_command();
    }
  }

  fn run_shading_command<T>(shading_cmd: &pipeline::ShadingCommand<Self, T>) {
    unsafe { gl::UseProgram(shading_cmd.program.repr) };

    let uniform_interface = &shading_cmd.program.uniform_interface;
    (shading_cmd.update)(uniform_interface);

    for render_cmd in &shading_cmd.render_commands {
      set_blending(render_cmd.blending);
      set_depth_test(render_cmd.depth_test);
      (render_cmd.update)(uniform_interface);
      (render_cmd.tessellation.repr.render)(render_cmd.rasterization_size, render_cmd.instances);
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
    blending::Factor::SrcColorComplement => gl::ONE_MINUS_SRC_COLOR,
    blending::Factor::DestColor => gl::DST_COLOR,
    blending::Factor::DestColorComplement => gl::ONE_MINUS_DST_COLOR,
    blending::Factor::SrcAlpha => gl::SRC_ALPHA,
    blending::Factor::SrcAlphaComplement => gl::ONE_MINUS_SRC_ALPHA,
    blending::Factor::DstAlpha => gl::DST_ALPHA,
    blending::Factor::DstAlphaComplement => gl::ONE_MINUS_DST_ALPHA,
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
