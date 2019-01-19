use gl;
use gl::types::*;
use luminance::linear::{M22, M33, M44};
use luminance::shader::program::{self, Dim, HasProgram, ProgramError, Type, Sem, SemIndex,
                                 UniformWarning};
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr::null_mut;

use gl33::token::GL33;

pub type Program = program::Program<GL33>;

pub struct GLProgram {
  pub id: GLuint, // OpenGL ID
  uni_sem_map: HashMap<SemIndex, GLint>, // mapping between user semantic (indexes) and OpenGL uniform locations
  ubo_sem_map: HashMap<SemIndex, GLint>, // mapping between user semantic (indexes) and OpenGL uniform block indexes
}

impl HasProgram for GL33 {
  type Program = GLProgram;

  fn new_program(tess: Option<(&Self::AStage, &Self::AStage)>, vertex: &Self::AStage, geometry: Option<&Self::AStage>, fragment: &Self::AStage, sem_map: &[Sem]) -> Result<(Self::Program, Vec<UniformWarning>), ProgramError> {
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
        let mut uni_sem_map = HashMap::new();
        let mut ubo_sem_map = HashMap::new();
        let mut warnings = Vec::new();

        for sem in sem_map {
          let (loc, warning) = get_uniform_location(program, sem.name(), sem.ty(), sem.dim());

          match loc {
            Location::Uniform(location) => uni_sem_map.insert(sem.index(), location),
            Location::UniformBlock(index) => ubo_sem_map.insert(sem.index(), index)
          };

          // if there’s a warning, add it to the list of warnings
          if let Some(warning) = warning {
            warnings.push(warning);
          }
        }

        let gl_program = GLProgram {
          id: program,
          uni_sem_map: uni_sem_map,
          ubo_sem_map: ubo_sem_map,
        };

        Ok((gl_program, warnings))
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
    unsafe { gl::DeleteProgram(program.id) }
  }

  fn update_uniforms<F>(program: &Self::Program, f: F) where F: Fn() {
    unsafe { gl::UseProgram(program.id) };
    f();
    unsafe { gl::UseProgram(0) };
  }

  fn update1_i32(program: &Self::Program, u: SemIndex, x: i32) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform1i(program.uni_sem_map[&u], x) }
  }

  fn update2_i32(program: &Self::Program, u: SemIndex, v: [i32; 2]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform2iv(program.uni_sem_map[&u], 1, &v as *const i32) }
  }

  fn update3_i32(program: &Self::Program, u: SemIndex, v: [i32; 3]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform3iv(program.uni_sem_map[&u], 1, &v as *const i32) }
  }

  fn update4_i32(program: &Self::Program, u: SemIndex, v: [i32; 4]) {
    unsafe { gl::Uniform4iv(program.uni_sem_map[&u], 1, &v as *const i32) }
  }

  fn update1_slice_i32(program: &Self::Program, u: SemIndex, v: &[i32]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform1iv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr()) }
  }

  fn update2_slice_i32(program: &Self::Program, u: SemIndex, v: &[[i32; 2]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform2iv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const i32) }
  }

  fn update3_slice_i32(program: &Self::Program, u: SemIndex, v: &[[i32; 3]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform3iv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const i32) }
  }

  fn update4_slice_i32(program: &Self::Program, u: SemIndex, v: &[[i32; 4]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform4iv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const i32) }
  }

  fn update1_u32(program: &Self::Program, u: SemIndex, x: u32) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform1ui(program.uni_sem_map[&u], x) }
  }

  fn update2_u32(program: &Self::Program, u: SemIndex, v: [u32; 2]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform2uiv(program.uni_sem_map[&u], 1, &v as *const u32) }
  }

  fn update3_u32(program: &Self::Program, u: SemIndex, v: [u32; 3]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform3uiv(program.uni_sem_map[&u], 1, &v as *const u32) }
  }

  fn update4_u32(program: &Self::Program, u: SemIndex, v: [u32; 4]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform4uiv(program.uni_sem_map[&u], 1, &v as *const u32) }
  }

  fn update1_slice_u32(program: &Self::Program, u: SemIndex, v: &[u32]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform1uiv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const u32) }
  }

  fn update2_slice_u32(program: &Self::Program, u: SemIndex, v: &[[u32; 2]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform2uiv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const u32) }
  }

  fn update3_slice_u32(program: &Self::Program, u: SemIndex, v: &[[u32; 3]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform3uiv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const u32) }
  }

  fn update4_slice_u32(program: &Self::Program, u: SemIndex, v: &[[u32; 4]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform4uiv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const u32) }
  }

  fn update1_f32(program: &Self::Program, u: SemIndex, x: f32) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform1f(program.uni_sem_map[&u], x) }
  }

  fn update2_f32(program: &Self::Program, u: SemIndex, v: [f32; 2]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform2fv(program.uni_sem_map[&u], 1, &v as *const f32) }
  }

  fn update3_f32(program: &Self::Program, u: SemIndex, v: [f32; 3]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform3fv(program.uni_sem_map[&u], 1, &v as *const f32) }
  }

  fn update4_f32(program: &Self::Program, u: SemIndex, v: [f32; 4]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform4fv(program.uni_sem_map[&u], 1, &v as *const f32) }
  }

  fn update1_slice_f32(program: &Self::Program, u: SemIndex, v: &[f32]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform1fv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const f32) }
  }

  fn update2_slice_f32(program: &Self::Program, u: SemIndex, v: &[[f32; 2]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform2fv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const f32) }
  }

  fn update3_slice_f32(program: &Self::Program, u: SemIndex, v: &[[f32; 3]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform3fv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const f32) }
  }

  fn update4_slice_f32(program: &Self::Program, u: SemIndex, v: &[[f32; 4]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform4fv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const f32) }
  }

  fn update22_f32(program: &Self::Program, u: SemIndex, m: M22) {
    assert!((u as usize) < program.uni_sem_map.len());
    Self::update22_slice_f32(program, u, &[m])
  }

  fn update33_f32(program: &Self::Program, u: SemIndex, m: M33) {
    assert!((u as usize) < program.uni_sem_map.len());
    Self::update33_slice_f32(program, u, &[m])
  }

  fn update44_f32(program: &Self::Program, u: SemIndex, m: M44) {
    assert!((u as usize) < program.uni_sem_map.len());
    Self::update44_slice_f32(program, u, &[m])
  }

  fn update22_slice_f32(program: &Self::Program, u: SemIndex, v: &[M22]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::UniformMatrix2fv(program.uni_sem_map[&u], v.len() as GLsizei, gl::FALSE, v.as_ptr() as *const f32) }
  }

  fn update33_slice_f32(program: &Self::Program, u: SemIndex, v: &[M33]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::UniformMatrix3fv(program.uni_sem_map[&u], v.len() as GLsizei, gl::FALSE, v.as_ptr() as *const f32) }
  }

  fn update44_slice_f32(program: &Self::Program, u: SemIndex, v: &[M44]) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::UniformMatrix4fv(program.uni_sem_map[&u], v.len() as GLsizei, gl::FALSE, v.as_ptr() as *const f32) }
  }

  fn update1_bool(program: &Self::Program, u: SemIndex, x: bool) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform1ui(program.uni_sem_map[&u], x as GLuint) }
  }

  fn update2_bool(program: &Self::Program, u: SemIndex, v: [bool; 2]) {
    assert!((u as usize) < program.uni_sem_map.len());
    let v = [v[0] as u32, v[1] as u32];
    unsafe { gl::Uniform2uiv(program.uni_sem_map[&u], 1, &v as *const u32) }
  }

  fn update3_bool(program: &Self::Program, u: SemIndex, v: [bool; 3]) {
    assert!((u as usize) < program.uni_sem_map.len());
    let v = [v[0] as u32, v[1] as u32, v[2] as u32];
    unsafe { gl::Uniform3uiv(program.uni_sem_map[&u], 1, &v as *const u32) }
  }

  fn update4_bool(program: &Self::Program, u: SemIndex, v: [bool; 4]) {
    assert!((u as usize) < program.uni_sem_map.len());
    let v = [v[0] as u32, v[1] as u32, v[2] as u32, v[3] as u32];
    unsafe { gl::Uniform4uiv(program.uni_sem_map[&u], 1,  &v as *const u32) }
  }

  fn update1_slice_bool(program: &Self::Program, u: SemIndex, v: &[bool]) {
    assert!((u as usize) < program.uni_sem_map.len());
    let v: Vec<_> = v.iter().map(|x| *x as u32).collect();
    unsafe { gl::Uniform1uiv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr()) }
  }

  fn update2_slice_bool(program: &Self::Program, u: SemIndex, v: &[[bool; 2]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    let v: Vec<_> = v.iter().map(|x| [x[0] as u32, x[1] as u32]).collect();
    unsafe { gl::Uniform2uiv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const u32) }
  }

  fn update3_slice_bool(program: &Self::Program, u: SemIndex, v: &[[bool; 3]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    let v: Vec<_> = v.iter().map(|x| [x[0] as u32, x[1] as u32, x[2] as u32]).collect();
    unsafe { gl::Uniform3uiv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const u32) }
  }

  fn update4_slice_bool(program: &Self::Program, u: SemIndex, v: &[[bool; 4]]) {
    assert!((u as usize) < program.uni_sem_map.len());
    let v: Vec<_> = v.iter().map(|x| [x[0] as u32, x[1] as u32, x[2] as u32, x[3] as u32]).collect();
    unsafe { gl::Uniform4uiv(program.uni_sem_map[&u], v.len() as GLsizei, v.as_ptr() as *const u32) }
  }

  fn update_texture_unit(program: &Self::Program, u: SemIndex, unit: u32) {
    assert!((u as usize) < program.uni_sem_map.len());
    unsafe { gl::Uniform1i(program.uni_sem_map[&u], unit as GLint) }
  }

  fn update_buffer_binding(program: &Self::Program, u: SemIndex, binding: u32) {
    assert!((u as usize) < program.ubo_sem_map.len());
    unsafe { gl::UniformBlockBinding(program.id, program.ubo_sem_map[&u] as GLuint, binding as GLuint) }
  }
}

enum Location {
  Uniform(GLint),
  UniformBlock(GLint),
}

// Retrieve the uniform location.
fn get_uniform_location(program: GLuint, name: &str, ty: Type, dim: Dim) -> (Location, Option<UniformWarning>) {
  let c_name = CString::new(name.as_bytes()).unwrap();
  let location = if ty == Type::BufferBinding {
    let index = unsafe { gl::GetUniformBlockIndex(program, c_name.as_ptr() as *const GLchar) };

    if index == gl::INVALID_INDEX {
      return (Location::UniformBlock(-1), Some(UniformWarning::Inactive(name.to_owned())));
    }

    Location::UniformBlock(index as GLint)
  } else {
    let location = unsafe { gl::GetUniformLocation(program, c_name.as_ptr() as *const GLchar) };

    if location == -1 {
      return (Location::Uniform(-1), Some(UniformWarning::Inactive(name.to_owned())));
    }

    Location::Uniform(location)
  };

  if let Some(err) = uniform_type_match(program, name, ty, dim) {
    return (location, Some(UniformWarning::TypeMismatch(name.to_owned(), err)));
  }

  (location, None)
}

// Return something if no match can be established.
fn uniform_type_match(program: GLuint, name: &str, ty: Type, dim: Dim) -> Option<String> {
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
    (Type::Integral, Dim::Dim1) if typ != gl::INT => Some("requested int doesn't match".to_owned()),
    (Type::Integral, Dim::Dim2) if typ != gl::INT_VEC2 => Some("requested ivec2 doesn't match".to_owned()),
    (Type::Integral, Dim::Dim3) if typ != gl::INT_VEC3 => Some("requested ivec3 doesn't match".to_owned()),
    (Type::Integral, Dim::Dim4) if typ != gl::INT_VEC4 => Some("requested ivec4 doesn't match".to_owned()),
    (Type::Unsigned, Dim::Dim1) if typ != gl::UNSIGNED_INT => Some("requested uint doesn't match".to_owned()),
    (Type::Unsigned, Dim::Dim2) if typ != gl::UNSIGNED_INT_VEC2 => Some("requested uvec2 doesn't match".to_owned()),
    (Type::Unsigned, Dim::Dim3) if typ != gl::UNSIGNED_INT_VEC3 => Some("requested uvec3 doesn't match".to_owned()),
    (Type::Unsigned, Dim::Dim4) if typ != gl::UNSIGNED_INT_VEC4 => Some("requested uvec4 doesn't match".to_owned()),
    (Type::Floating, Dim::Dim1) if typ != gl::FLOAT => Some("requested float doesn't match".to_owned()),
    (Type::Floating, Dim::Dim2) if typ != gl::FLOAT_VEC2 => Some("requested vec2 doesn't match".to_owned()),
    (Type::Floating, Dim::Dim3) if typ != gl::FLOAT_VEC3 => Some("requested vec3 doesn't match".to_owned()),
    (Type::Floating, Dim::Dim4) if typ != gl::FLOAT_VEC4 => Some("requested vec4 doesn't match".to_owned()),
    (Type::Floating, Dim::Dim22) if typ != gl::FLOAT_MAT2 => Some("requested mat2 doesn't match".to_owned()),
    (Type::Floating, Dim::Dim33) if typ != gl::FLOAT_MAT3 => Some("requested mat3 doesn't match".to_owned()),
    (Type::Floating, Dim::Dim44) if typ != gl::FLOAT_MAT4 => Some("requested mat4 doesn't match".to_owned()),
    (Type::Boolean, Dim::Dim1) if typ != gl::BOOL => Some("requested bool doesn't match".to_owned()),
    (Type::Boolean, Dim::Dim2) if typ != gl::BOOL_VEC2 => Some("requested bvec2 doesn't match".to_owned()),
    (Type::Boolean, Dim::Dim3) if typ != gl::BOOL_VEC3 => Some("requested bvec3 doesn't match".to_owned()),
    (Type::Boolean, Dim::Dim4) if typ != gl::BOOL_VEC4 => Some("requested bvec4 doesn't match".to_owned()),
    _ => None
  }
}

pub type Uniform<T> = program::Uniform<GL33, T>;
pub type Uniformable = program::Uniformable<GL33>;
