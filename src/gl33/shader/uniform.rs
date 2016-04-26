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

  fn update3_i32(u: &Self::U, (x, y, z): [i32; 3]) {
    unsafe { gl::Uniform3i(*u, x, y, z) }
  }

  fn update4_i32(u: &Self::U, (x, y, z, w): [i32; 4]) {
    unsafe { gl::Uniform4i(*u, x, y, z, w) }
  }

  fn update1_slice_i32(u: &Self::U, v: &[i32]) {
    unsafe { gl::Uniform1iv(*u, v.len() as GLsizei, v) }
  }

  fn update2_slice_i32(u: &Self::U, v: &[[i32; 2]]) {
    unsafe { gl::Uniform2iv(*u, v.len() as GLsizei, v) }
  }

  fn update3_slice_i32(u: &Self::U, v: &Vec<(i32, i32, i32)>) {
    unsafe { gl::Uniform3iv(*u, v.len() as GLsizei, v.as_ptr() as *const i32) }
  }

  fn update4_slice_i32(u: &Self::U, v: &Vec<(i32, i32, i32, i32)>) {
    unsafe { gl::Uniform4iv(*u, v.len() as GLsizei, v.as_ptr() as *const i32) }
  }

  fn update1_u32(u: &Self::U, x: u32) {
    unsafe { gl::Uniform1ui(*u, x) }
  }

  fn update2_u32(u: &Self::U, (x, y): (u32, u32)) {
    unsafe { gl::Uniform2ui(*u, x, y) }
  }

  fn update3_u32(u: &Self::U, (x, y, z): (u32, u32, u32)) {
    unsafe { gl::Uniform3ui(*u, x, y, z) }
  }

  fn update4_u32(u: &Self::U, (x, y, z, w): (u32, u32, u32, u32)) {
    unsafe { gl::Uniform4ui(*u, x, y, z, w) }
  }

  fn update1_slice_u32(u: &Self::U, v: &Vec<u32>) {
    unsafe { gl::Uniform1uiv(*u, v.len() as GLsizei, v.as_ptr() as *const u32) }
  }

  fn update2_slice_u32(u: &Self::U, v: &Vec<(u32, u32)>) {
    unsafe { gl::Uniform2uiv(*u, v.len() as GLsizei, v.as_ptr() as *const u32) }
  }

  fn update3_slice_u32(u: &Self::U, v: &Vec<(u32, u32, u32)>) {
    unsafe { gl::Uniform3uiv(*u, v.len() as GLsizei, v.as_ptr() as *const u32) }
  }

  fn update4_slice_u32(u: &Self::U, v: &Vec<(u32, u32, u32, u32)>) {
    unsafe { gl::Uniform4uiv(*u, v.len() as GLsizei, v.as_ptr() as *const u32) }
  }

  fn update1_f32(u: &Self::U, x: f32) {
    unsafe { gl::Uniform1f(*u, x) }
  }

  fn update2_f32(u: &Self::U, (x, y): (f32, f32)) {
    unsafe { gl::Uniform2f(*u, x, y) }
  }

  fn update3_f32(u: &Self::U, (x, y, z): (f32, f32, f32)) {
    unsafe { gl::Uniform3f(*u, x, y, z) }
  }

  fn update4_f32(u: &Self::U, (x, y, z, w): (f32, f32, f32, f32)) {
    unsafe { gl::Uniform4f(*u, x, y, z, w) }
  }

  fn update1_slice_f32(u: &Self::U, v: &Vec<f32>) {
    unsafe { gl::Uniform1fv(*u, v.len() as GLsizei, v.as_ptr() as *const f32) }
  }

  fn update2_slice_f32(u: &Self::U, v: &Vec<(f32, f32)>) {
    unsafe { gl::Uniform2fv(*u, v.len() as GLsizei, v.as_ptr() as *const f32) }
  }

  fn update3_slice_f32(u: &Self::U, v: &Vec<(f32, f32, f32)>) {
    unsafe { gl::Uniform3fv(*u, v.len() as GLsizei, v.as_ptr() as *const f32) }
  }

  fn update4_slice_f32(u: &Self::U, v: &Vec<(f32, f32, f32, f32)>) {
    unsafe { gl::Uniform4fv(*u, v.len() as GLsizei, v.as_ptr() as *const f32) }
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

  fn update22_slice_f32(u: &Self::U, v: &Vec<M22>) {
    unsafe { gl::UniformMatrix2fv(*u, v.len() as GLsizei, gl::FALSE, v.as_ptr() as *const f32) }
  }

  fn update33_slice_f32(u: &Self::U, v: &Vec<M33>) {
    unsafe { gl::UniformMatrix3fv(*u, v.len() as GLsizei, gl::FALSE, v.as_ptr() as *const f32) }
  }

  fn update44_slice_f32(u: &Self::U, v: &Vec<M44>) {
    unsafe { gl::UniformMatrix4fv(*u, v.len() as GLsizei, gl::FALSE, v.as_ptr() as *const f32) }
  }

  fn update1_bool(u: &Self::U, x: bool) {
    unsafe { gl::Uniform1i(*u, x as i32) }
  }

  fn update2_bool(u: &Self::U, (x, y): (bool, bool)) {
    unsafe { gl::Uniform2i(*u, x as i32, y as i32) }
  }

  fn update3_bool(u: &Self::U, (x, y, z): (bool, bool, bool)) {
    unsafe { gl::Uniform3i(*u, x as i32, y as i32, z as i32) }
  }

  fn update4_bool(u: &Self::U, (x, y, z, w): (bool, bool, bool, bool)) {
    unsafe { gl::Uniform4i(*u, x as i32, y as i32, z as i32, w as i32) }
  }

  fn update1_slice_bool(u: &Self::U, v: &Vec<bool>) {
    unsafe { gl::Uniform1iv(*u, v.len() as GLsizei, v.as_ptr() as *const i32) }
  }

  fn update2_slice_bool(u: &Self::U, v: &Vec<(bool, bool)>) {
    unsafe { gl::Uniform2iv(*u, v.len() as GLsizei, v.as_ptr() as *const i32) }
  }

  fn update3_slice_bool(u: &Self::U, v: &Vec<(bool, bool, bool)>) {
    unsafe { gl::Uniform3iv(*u, v.len() as GLsizei, v.as_ptr() as *const i32) }
  }

  fn update4_slice_bool(u: &Self::U, v: &Vec<(bool, bool, bool, bool)>) {
    unsafe { gl::Uniform4iv(*u, v.len() as GLsizei, v.as_ptr() as *const i32) }
  }
}
