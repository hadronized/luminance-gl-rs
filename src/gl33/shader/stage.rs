use gl;
use gl::types::*;
use gl33::token::GL33;
use error::debug_gl;
use luminance::shader::stage;
use luminance::shader::stage::{HasStage, StageError, Type};
use std::ptr::{null, null_mut};
use std::str::from_utf8;

pub use luminance::shader::stage::{TessellationControlShader, TessellationEvaluationShader,
                                   VertexShader, GeometryShader, FragmentShader};

pub type Stage<T> = stage::Stage<GL33, T>;

impl HasStage for GL33 {
  type AStage = GLuint;

  fn new_shader(shader_type: Type, src: &str) -> Result<Self::AStage, StageError> {
    // FIXME: check for GL_ARB_tessellation_shader extension if we need tessellation shaders
    let shader_type = match shader_type {
      Type::TessellationControlShader => gl::TESS_CONTROL_SHADER,
      Type::TessellationEvaluationShader => gl::TESS_EVALUATION_SHADER,
      Type::VertexShader => gl::VERTEX_SHADER,
      Type::GeometryShader => gl::GEOMETRY_SHADER,
      Type::FragmentShader => gl::FRAGMENT_SHADER
    };

    unsafe {
      let src = glsl_pragma_src(src);
      let handle = gl::CreateShader(shader_type);
      debug_gl();

      println!("shader {} source dump: {}", handle, src);
      gl::ShaderSource(handle, 1, &(src.as_ptr() as *const i8), null());
      debug_gl();
      gl::CompileShader(handle);
      debug_gl();

      let mut compiled: GLboolean = gl::FALSE;
      let mut log_len: GLint = 0;
      let mut log: Vec<u8> = Vec::with_capacity(log_len as usize + 1); // extra '\0'
      gl::GetShaderiv(handle, gl::COMPILE_STATUS, (&mut compiled as *mut GLboolean) as *mut GLint);
      debug_gl();
      gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut log_len);
      debug_gl();
      gl::GetShaderInfoLog(handle, log_len, null_mut(), log.as_mut_ptr() as *mut i8);
      debug_gl();

      if compiled == gl::TRUE {
        Ok(handle)
      } else {
        Err(StageError::CompilationFailed(String::from(from_utf8(&log[..]).unwrap())))
      }
    }
  }
}

fn glsl_pragma_src(src: &str) -> String {
  let mut pragma = String::from(GLSL_PRAGMA);
  pragma.push_str(src);

  pragma
}

const GLSL_PRAGMA: &'static str =
"#version 330 core\n\
#extension GL_ARB_separate_shader_objects : require\n";
