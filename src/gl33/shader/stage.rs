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

//impl<T> Drop for Stage<T> {
//  fn drop
//}

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
      let src = CString::new(glsl_pragma_src(src).as_bytes()).unwrap();
      let handle = gl::CreateShader(shader_type);

      if handle == 0 {
        return Err(StageError::CompilationFailed(String::from("unable to create shader stage")));
      }

      gl::ShaderSource(handle, 1, [src.as_ptr()].as_ptr(), null());
      gl::CompileShader(handle);

      let mut compiled: GLboolean = gl::FALSE;
      gl::GetShaderiv(handle, gl::COMPILE_STATUS, (&mut compiled as *mut GLboolean) as *mut GLint);

      if compiled == gl::TRUE {
        Ok(handle)
      } else {
        let mut log_len: GLint = 0;
        gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut log_len);

        let mut log: Vec<u8> = Vec::with_capacity(log_len as usize);
        gl::GetShaderInfoLog(handle, log_len, null_mut(), log.as_mut_ptr() as *mut GLchar);

        log.set_len(log_len as usize);
        Err(StageError::CompilationFailed(String::from_utf8(log).unwrap()))
      }
    }
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
