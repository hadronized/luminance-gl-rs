use gl;
use gl::types::*;
use gl33::token::GL33;
use luminance::shader::program;
use luminance::shader::program::{HasProgram, ProgramError};
use luminance::shader::uniform::UniformName;
use std::ffi::CString;
use std::ptr::null_mut;

pub type Program<T> = program::Program<GL33, T>;

impl HasProgram for GL33 {
  type Program = GLuint;

  fn new_program(tess: Option<(&Self::AStage, &Self::AStage)>, vertex: &Self::AStage, geometry: Option<&Self::AStage>, fragment: &Self::AStage) -> Result<Self::Program, ProgramError> {
    unsafe {
      let program = gl::CreateProgram();

      if let Some((tcs, tes)) = tess {
        gl::AttachShader(program, *tcs);
        gl::AttachShader(program, *tes);
      }

      gl::AttachShader(program, *vertex);

      if let Some(geometry) = geometry {
        gl::AttachShader(program, *geometry);
      }

      gl::AttachShader(program, *fragment);

      gl::LinkProgram(program);

      let mut linked: GLboolean = gl::FALSE;
      gl::GetProgramiv(program, gl::LINK_STATUS, (&mut linked as *mut GLboolean) as *mut GLint);

      if linked == gl::TRUE {
        Ok(program)
      } else {
        let mut log_len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_len);

        let mut log: Vec<u8> = Vec::with_capacity(log_len as usize);
        gl::GetProgramInfoLog(program, log_len, null_mut(), log.as_mut_ptr() as *mut GLchar);

        log.set_len(log_len as usize);
        Err(ProgramError::LinkFailed(String::from_utf8(log).unwrap()))
      }
    }
  }

  fn free_program(program: &mut Self::Program) {
    unsafe { gl::DeleteProgram(*program) }
  }

  fn map_uniform(program: &Self::Program, name: UniformName) -> Result<Self::U, ProgramError> {
    match name {
      UniformName::StringName(name) => {
        let location = unsafe { gl::GetUniformLocation(*program, CString::new(name.as_bytes()).unwrap().as_ptr() as *const GLchar) };
        if location != -1 { Ok(location) } else { Err(ProgramError::InactiveUniform(name)) }
      },
      UniformName::SemanticName(sem) => { Ok(sem as GLint) }
    }
  }

  fn update_uniforms<F>(program: &Self::Program, f: F) where F: Fn() {
    unsafe { gl::UseProgram(*program) };
    f();
    unsafe { gl::UseProgram(0) };
  }
}
