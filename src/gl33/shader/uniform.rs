use gl;
use gl::types::*;
use gl33::token::GL33;
use luminance::linear::*;
use luminance::shader::uniform;

pub type Uniform<T> = uniform::Uniform<GL33, T>;

impl uniform::HasUniform for GL33 {
  type U = GLint;

  fn update1_i32(u: &Self::U, x: i32) {
    unsafe { gl::Uniform1i(*u, x) }
  }

  fn update2_i32(u: &Self::U, v: [i32; 2]) {
    unsafe { gl::Uniform2iv(*u, 1, v) }
  }

  fn update3_i32(u: &Self::U, v: [i32; 3]) {
    unsafe { gl::Uniform3iv(*u, 1, v) }
  }

  fn update4_i32(u: &Self::U, v: [i32; 4]) {
    unsafe { gl::Uniform4iv(*u, 1, v) }
  }

  fn update1_slice_i32(u: &Self::U, v: &[i32]) {
    unsafe { gl::Uniform1iv(*u, v.len() as GLsizei, v) }
  }

  fn update2_slice_i32(u: &Self::U, v: &[[i32; 2]]) {
    unsafe { gl::Uniform2iv(*u, v.len() as GLsizei, v) }
  }

  fn update3_slice_i32(u: &Self::U, v: &[[i32; 3]]) {
    unsafe { gl::Uniform3iv(*u, v.len() as GLsizei, v) }
  }

  fn update4_slice_i32(u: &Self::U, v: &[[i32; 4]]) {
    unsafe { gl::Uniform4iv(*u, v.len() as GLsizei, v) }
  }

  fn update1_u32(u: &Self::U, x: u32) {
    unsafe { gl::Uniform1ui(*u, x) }
  }

  fn update2_u32(u: &Self::U, v: [u32; 2]) {
    unsafe { gl::Uniform2uiv(*u, 1, v) }
  }

  fn update3_u32(u: &Self::U, v: [u32; 3]) {
    unsafe { gl::Uniform3uiv(*u, 1, v) }
  }

  fn update4_u32(u: &Self::U, v: [u32; 4]) {
    unsafe { gl::Uniform4uiv(*u, 1, v) }
  }

  fn update1_slice_u32(u: &Self::U, v: &[u32]) {
    unsafe { gl::Uniform1uiv(*u, v.len() as GLsizei, v) }
  }

  fn update2_slice_u32(u: &Self::U, v: &[[u32; 2]]) {
    unsafe { gl::Uniform2uiv(*u, v.len() as GLsizei, v) }
  }

  fn update3_slice_u32(u: &Self::U, v: &[[u32; 3]]) {
    unsafe { gl::Uniform3uiv(*u, v.len() as GLsizei, v) }
  }

  fn update4_slice_u32(u: &Self::U, v: &[[u32; 4]]) {
    unsafe { gl::Uniform4uiv(*u, v.len() as GLsizei, v) }
  }

  fn update1_f32(u: &Self::U, x: f32) {
    unsafe { gl::Uniform1f(*u, x) }
  }

  fn update2_f32(u: &Self::U, v: [f32; 2]) {
    unsafe { gl::Uniform2fv(*u, 1, v) }
  }

  fn update3_f32(u: &Self::U, v: [f32; 3]) {
    unsafe { gl::Uniform3fv(*u, 1, v) }
  }

  fn update4_f32(u: &Self::U, v: [f32; 4]) {
    unsafe { gl::Uniform4fv(*u, 1, v) }
  }

  fn update1_slice_f32(u: &Self::U, v: &[f32]) {
    unsafe { gl::Uniform1fv(*u, v.len() as GLsizei, v) }
  }

  fn update2_slice_f32(u: &Self::U, v: &[[f32; 2]]) {
    unsafe { gl::Uniform2fv(*u, v.len() as GLsizei, v) }
  }

  fn update3_slice_f32(u: &Self::U, v: &[[f32; 3]]) {
    unsafe { gl::Uniform3fv(*u, v.len() as GLsizei, v) }
  }

  fn update4_slice_f32(u: &Self::U, v: &[[f32; 4]]) {
    unsafe { gl::Uniform4fv(*u, v.len() as GLsizei, v) }
  }

  fn update22_f32(u: &Self::U, m: M22) {
    Self::update22_slice_f32(u, &vec![m])
  }

  fn update33_f32(u: &Self::U, m: M33) {
    Self::update33_slice_f32(u, &vec![m])
  }

  fn update44_f32(u: &Self::U, m: M44) {
    Self::update44_slice_f32(u, &vec![m])
  }

  fn update22_slice_f32(u: &Self::U, v: &[M22]) {
    unsafe { gl::UniformMatrix2fv(*u, v.len() as GLsizei, gl::FALSE, v) }
  }

  fn update33_slice_f32(u: &Self::U, v: &[M33]) {
    unsafe { gl::UniformMatrix3fv(*u, v.len() as GLsizei, gl::FALSE, v) }
  }

  fn update44_slice_f32(u: &Self::U, v: &[M44]) {
    unsafe { gl::UniformMatrix4fv(*u, v.len() as GLsizei, gl::FALSE, v) }
  }

  fn update1_bool(u: &Self::U, x: bool) {
    unsafe { gl::Uniform1i(*u, x as i32) }
  }

  fn update2_bool(u: &Self::U, v: [bool; 2]) {
    unsafe { gl::Uniform2iv(*u, 1, v.map(|x| x as i32)) }
  }

  fn update3_bool(u: &Self::U, v: [bool; 3]) {
    unsafe { gl::Uniform3iv(*u, 1, v.map(|x| x as i32)) }
  }

  fn update4_bool(u: &Self::U, v: [bool; 4]) {
    unsafe { gl::Uniform4iv(*u, 1, v.map(|x| x as i32)) }
  }

  fn update1_slice_bool(u: &Self::U, v: &[bool]) {
    unsafe { gl::Uniform1iv(*u, v.len() as GLsizei, v.map(|x| x as i32)) }
  }

  fn update2_slice_bool(u: &Self::U, v: &[[bool; 2]]) {
    unsafe { gl::Uniform2iv(*u, v.len() as GLsizei, v.map(|x| x.map(|y| y as i32))) }
  }

  fn update3_slice_bool(u: &Self::U, v: &[[bool; 3]]) {
    unsafe { gl::Uniform3iv(*u, v.len() as GLsizei, v.map(|x| x.map(|y| y as i32))) }
  }

  fn update4_slice_bool(u: &Self::U, v: &[[bool; 4]]) {
    unsafe { gl::Uniform4iv(*u, v.len() as GLsizei, v.map(|x| x.map(|y| y as i32))) }
  }
}
