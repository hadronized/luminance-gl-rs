use gl;
use gl::types::*;
use gl33::token::GL33;
use luminance::blending;
use luminance::framebuffer::{ColorSlot, DepthSlot};
use luminance::pipeline::{self, HasPipeline};
use luminance::texture::{Dimensionable, Layerable};

use gl33::shader::program::Program;

pub type Pipeline<'a, L, D, CS, DS> = pipeline::Pipeline<'a, GL33, L, D, CS, DS>;
pub type Pipe<'a, T> = pipeline::Pipe<'a, GL33, T>;
pub type ShadingCommand<'a> = pipeline::ShadingCommand<'a, GL33>;
pub type RenderCommand<'a> = pipeline::RenderCommand<'a, GL33>;

impl HasPipeline for GL33 {
  fn run_pipeline<L, D, CS, DS>(cmd: &Pipeline<L, D, CS, DS>)
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

      // traverse the texture set and bind required textures
      for (unit, tex) in cmd.texture_set.iter().enumerate() {
        gl::ActiveTexture(gl::TEXTURE0 + unit as GLenum);
        gl::BindTexture(tex.repr.target, tex.repr.handle);
      }

      // traverse the buffer set and bind required buffers
      for (index, buf) in cmd.buffer_set.iter().enumerate() {
        gl::BindBufferBase(gl::UNIFORM_BUFFER, index as GLuint, buf.repr.handle);
      }
    }

    for piped_shading_cmd in &cmd.shading_commands {
      Self::run_shading_command(piped_shading_cmd);
    }
  }

  fn run_shading_command<'a>(piped: &Pipe<'a, ShadingCommand>) {
    let update_program = &piped.update_program;
    let shading_cmd = &piped.next;

    unsafe { gl::UseProgram(shading_cmd.program.0.id) };

    update_program(&shading_cmd.program);

    for piped_render_cmd in &shading_cmd.render_commands {
      run_render_command(&shading_cmd.program, piped_render_cmd);
    }
  }
}

fn run_render_command<'a>(program: &Program, piped: &Pipe<'a, RenderCommand<'a>>) {
  let update_program = &piped.update_program;
  let render_cmd = &piped.next;

  update_program(program);

  set_blending(render_cmd.blending);
  set_depth_test(render_cmd.depth_test);

  for piped_tess in &render_cmd.tessellations {
    let tess_update_program = &piped_tess.update_program;
    let tess = &piped_tess.next;

    tess_update_program(program);

    (tess.repr.render)(render_cmd.rasterization_size, render_cmd.instances);
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
