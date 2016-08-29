use gl;
use gl::types::*;
use gl33::token::GL33;
use luminance::shader::stage;
use luminance::shader::stage::{HasStage, StageError, Type};
use std::ffi::CString;
use std::ptr::{null, null_mut};

pub use luminance::shader::stage::{TessellationControlShader, TessellationEvaluationShader,
                                   VertexShader, GeometryShader, FragmentShader};

pub type Stage<T> = stage::Stage<GL33, T>;

impl HasStage for GL33 {
  type AStage = GLuint;

  fn new_shader(shader_type: Type, src: &str) -> Result<Self::AStage, StageError> {
    unsafe {
      let src = CString::new(glsl_pragma_src(src).as_bytes()).unwrap();
      let handle = gl::CreateShader(from_shader_type(shader_type));

      if handle == 0 {
        return Err(StageError::CompilationFailed(shader_type, String::from("unable to create shader stage")));
      }

      gl::ShaderSource(handle, 1, [src.as_ptr()].as_ptr(), null());
      gl::CompileShader(handle);

      let mut compiled: GLint = gl::FALSE as GLint;
      gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut compiled);

      if compiled == (gl::TRUE as GLint) {
        Ok(handle)
      } else {
        let mut log_len: GLint = 0;
        gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut log_len);

        let mut log: Vec<u8> = Vec::with_capacity(log_len as usize);
        gl::GetShaderInfoLog(handle, log_len, null_mut(), log.as_mut_ptr() as *mut GLchar);

        gl::DeleteShader(handle);

        log.set_len(log_len as usize);

        Err(StageError::CompilationFailed(shader_type, String::from_utf8(log).unwrap()))
      }
    }
  }

  fn free_shader(shader: &mut Self::AStage) {
    unsafe { gl::DeleteShader(*shader) }
  }
}

// FIXME: check for GL_ARB_tessellation_shader extension if we need tessellation shaders
fn from_shader_type(t: Type) -> GLenum {
  match t {
    Type::TessellationControlShader => gl::TESS_CONTROL_SHADER,
    Type::TessellationEvaluationShader => gl::TESS_EVALUATION_SHADER,
    Type::VertexShader => gl::VERTEX_SHADER,
    Type::GeometryShader => gl::GEOMETRY_SHADER,
    Type::FragmentShader => gl::FRAGMENT_SHADER
  }
}

fn glsl_pragma_src(src: &str) -> String {
  let mut pragma = String::from(GLSL_PRAGMA);
  pragma.push_str(src);

  pragma
}

const GLSL_PRAGMA: &'static str = "\
#version 330 core\n\
#extension GL_ARB_separate_shader_objects : require\n";
