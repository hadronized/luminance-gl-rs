use gl;
use gl::types::*;
use gl33::token::GL33;
use luminance::shader::program;
use luminance::shader::program::{HasProgram, ProgramError, UniformWarning};
use luminance::shader::uniform::{Dim, Type};
use std::ffi::CString;
use std::ptr::null_mut;

pub type Program<T> = program::Program<GL33, T>;
pub type ProgramProxy<'a> = program::ProgramProxy<'a, GL33>;

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

      let mut linked: GLint = gl::FALSE as GLint;
      gl::GetProgramiv(program, gl::LINK_STATUS, &mut linked);

      if linked == (gl::TRUE as GLint) {
        Ok(program)
      } else {
        let mut log_len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_len);

        let mut log: Vec<u8> = Vec::with_capacity(log_len as usize);
        gl::GetProgramInfoLog(program, log_len, null_mut(), log.as_mut_ptr() as *mut GLchar);

        gl::DeleteProgram(program);

        log.set_len(log_len as usize);

        Err(ProgramError::LinkFailed(String::from_utf8(log).unwrap()))
      }
    }
  }

  fn free_program(program: &mut Self::Program) {
    unsafe { gl::DeleteProgram(*program) }
  }

  fn map_uniform(program: &Self::Program, name: String, ty: Type, dim: Dim) -> (Self::U, Option<UniformWarning>) {
    let c_name = CString::new(name.as_bytes()).unwrap();
    let location = unsafe { gl::GetUniformLocation(*program, c_name.as_ptr() as *const GLchar) };

    if location == -1 {
      return (-1, Some(UniformWarning::Inactive(name)));
    }

    if let Some(err) = uniform_type_match(*program, name, ty, dim) {
      return (location, Some(UniformWarning::TypeMismatch(err)));
    }

    (location, None)
  }

  fn update_uniforms<F>(program: &Self::Program, f: F) where F: Fn() {
    unsafe { gl::UseProgram(*program) };
    f();
    unsafe { gl::UseProgram(0) };
  }
}

// Return something if no match can be established.
fn uniform_type_match(program: GLuint, name: String, ty: Type, dim: Dim) -> Option<String> {
  let mut size: GLint = 0;
  let mut typ: GLuint = 0;

  unsafe {
    // get the index of the uniform
    let mut index = 0;
    gl::GetUniformIndices(program, 1, [name.as_ptr() as *const i8].as_ptr(), &mut index);
    // get its size and type
    gl::GetActiveUniform(program, index, 0, null_mut(), &mut size, &mut typ, null_mut());
  }

  // FIXME
  // early-return if array – we don’t support them yet
  if size != 1 {
    return None;
  }

  match (ty, dim) {
    (Type::Integral, Dim::Dim1) if typ != gl::INT => Some("requested int doesn't match".into()),
    (Type::Integral, Dim::Dim2) if typ != gl::INT_VEC2 => Some("requested ivec2 doesn't match".into()),
    (Type::Integral, Dim::Dim3) if typ != gl::INT_VEC3 => Some("requested ivec3 doesn't match".into()),
    (Type::Integral, Dim::Dim4) if typ != gl::INT_VEC4 => Some("requested ivec4 doesn't match".into()),
    (Type::Unsigned, Dim::Dim1) if typ != gl::UNSIGNED_INT => Some("requested uint doesn't match".into()),
    (Type::Unsigned, Dim::Dim2) if typ != gl::UNSIGNED_INT_VEC2 => Some("requested uvec2 doesn't match".into()),
    (Type::Unsigned, Dim::Dim3) if typ != gl::UNSIGNED_INT_VEC3 => Some("requested uvec3 doesn't match".into()),
    (Type::Unsigned, Dim::Dim4) if typ != gl::UNSIGNED_INT_VEC4 => Some("requested uvec4 doesn't match".into()),
    (Type::Floating, Dim::Dim1) if typ != gl::FLOAT => Some("requested float doesn't match".into()),
    (Type::Floating, Dim::Dim2) if typ != gl::FLOAT_VEC2 => Some("requested vec2 doesn't match".into()),
    (Type::Floating, Dim::Dim3) if typ != gl::FLOAT_VEC3 => Some("requested vec3 doesn't match".into()),
    (Type::Floating, Dim::Dim4) if typ != gl::FLOAT_VEC4 => Some("requested vec4 doesn't match".into()),
    (Type::Floating, Dim::Dim22) if typ != gl::FLOAT_MAT2 => Some("requested mat2 doesn't match".into()),
    (Type::Floating, Dim::Dim33) if typ != gl::FLOAT_MAT3 => Some("requested mat3 doesn't match".into()),
    (Type::Floating, Dim::Dim44) if typ != gl::FLOAT_MAT4 => Some("requested mat4 doesn't match".into()),
    (Type::Boolean, Dim::Dim1) if typ != gl::BOOL => Some("requested bool doesn't match".into()),
    (Type::Boolean, Dim::Dim2) if typ != gl::BOOL_VEC2 => Some("requested bvec2 doesn't match".into()),
    (Type::Boolean, Dim::Dim3) if typ != gl::BOOL_VEC3 => Some("requested bvec3 doesn't match".into()),
    (Type::Boolean, Dim::Dim4) if typ != gl::BOOL_VEC4 => Some("requested bvec4 doesn't match".into()),
    (Type::ISampler, Dim::Dim1) if typ != gl::INT_SAMPLER_1D => Some("requested isampler1D doesn't match".into()),
    (Type::ISampler, Dim::Dim2) if typ != gl::INT_SAMPLER_2D => Some("requested isampler2D doesn't match".into()),
    (Type::ISampler, Dim::Dim3) if typ != gl::INT_SAMPLER_3D => Some("requested isampler3D doesn't match".into()),
    (Type::ISampler, Dim::Cubemap) if typ != gl::INT_SAMPLER_CUBE => Some("requested isamplerCube doesn't match".into()),
    (Type::USampler, Dim::Dim1) if typ != gl::UNSIGNED_INT_SAMPLER_1D => Some("requested usampler1D doesn't match".into()),
    (Type::USampler, Dim::Dim2) if typ != gl::UNSIGNED_INT_SAMPLER_2D => Some("requested usampler2D doesn't match".into()),
    (Type::USampler, Dim::Dim3) if typ != gl::UNSIGNED_INT_SAMPLER_3D => Some("requested usampler3D doesn't match".into()),
    (Type::USampler, Dim::Cubemap) if typ != gl::UNSIGNED_INT_SAMPLER_CUBE => Some("requested usamplerCube doesn't match".into()),
    (Type::Sampler, Dim::Dim1) if typ != gl::SAMPLER_1D => Some("requested sampler1D doesn't match".into()),
    (Type::Sampler, Dim::Dim2) if typ != gl::SAMPLER_2D => Some("requested sampler2D doesn't match".into()),
    (Type::Sampler, Dim::Dim3) if typ != gl::SAMPLER_3D => Some("requested sampler3D doesn't match".into()),
    (Type::Sampler, Dim::Cubemap) if typ != gl::SAMPLER_CUBE => Some("requested samplerCube doesn't match".into()),
    _ => None
  }
}
