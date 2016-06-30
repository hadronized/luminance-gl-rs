use gl;
use gl::types::*;
use gl33::token::GL33;
use luminance::buffer;
use std::cmp::Ordering::*;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::slice;

pub type Buffer<T> = buffer::Buffer<GL33, T>;

pub struct GLBuffer {
  pub handle: GLuint,
  pub bytes: usize
}

impl buffer::HasBuffer for GL33 {
  type ABuffer = GLBuffer;

  fn new(size: usize) -> Self::ABuffer {
    let mut buffer: GLuint = 0;

    unsafe {
      gl::GenBuffers(1, &mut buffer);
      gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
      gl::BufferData(gl::ARRAY_BUFFER, size as isize, ptr::null(), gl::STREAM_DRAW);
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    GLBuffer {
      handle: buffer,
      bytes: size
    }
  }

  fn free(buffer: &mut Self::ABuffer) {
    unsafe { gl::DeleteBuffers(1, &buffer.handle) }
  }

  fn write_whole<T>(buffer: &GLBuffer, values: &[T]) -> Result<(), buffer::BufferError> {
    let bytes = values.len() * mem::size_of::<T>();

    // generate warning and recompute the proper number of bytes to copy
    let (warning, bytes) = match bytes.cmp(&buffer.bytes) {
      Less => (Some(buffer::BufferError::TooFewValues), bytes),
      Greater => (Some(buffer::BufferError::TooManyValues), buffer.bytes),
      _ => (None, bytes)
    };

    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, buffer.handle);
      let ptr = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY);

      ptr::copy_nonoverlapping(values.as_ptr() as *const c_void, ptr, bytes);

      let _ = gl::UnmapBuffer(gl::ARRAY_BUFFER);
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    match warning {
      Some(w) => Err(w),
      None => Ok(())
    }
  }

  fn write<T>(buffer: &GLBuffer, off: usize, x: T) -> Result<(), buffer::BufferError> where T: Copy {
    if off >= buffer.bytes {
      return Err(buffer::BufferError::Overflow);
    }

    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, buffer.handle);
      let ptr = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY);

      *(ptr.offset(off as isize) as *mut T) = x;

      let _ = gl::UnmapBuffer(gl::ARRAY_BUFFER);
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    Ok(())
  }

  fn read_whole<T>(buffer: &GLBuffer, nb: usize) -> Vec<T> where T: Copy {
    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, buffer.handle);
      let ptr = gl::MapBuffer(gl::ARRAY_BUFFER, gl::READ_ONLY) as *const T;

      let values = Vec::from(slice::from_raw_parts(ptr, nb));

      let _ = gl::UnmapBuffer(gl::ARRAY_BUFFER);
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);

      values
    }
  }

  fn read<T>(buffer: &GLBuffer, off: usize) -> Option<T> where T: Copy {
    if off >= buffer.bytes {
      return None;
    }

    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, buffer.handle);
      let ptr = gl::MapBuffer(gl::ARRAY_BUFFER, gl::READ_ONLY);

      let x = &*(ptr.offset(off as isize) as *const T);

      let _ = gl::UnmapBuffer(gl::ARRAY_BUFFER);
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);

      Some(*x)
    }
  }
}
