use gl;
use gl::types::*;
use luminance::tess::{self, HasTess, Mode};
use luminance::vertex::{Dim, Type, Vertex, VertexComponentFormat, VertexFormat};
use std::mem;
use std::ptr;

use gl33::buffer::{Buffer, GLBuffer};
use gl33::token::GL33;

pub type Tess = tess::Tess<GL33>;

pub struct GLTess {
  // closure taking the point / line size and the number of instances to render
  pub render: Box<Fn(Option<f32>, u32)>,
  vao: GLenum,
  vbo: Option<GLBuffer>,
  ibo: Option<GLBuffer>,
  vertex_format: VertexFormat,
  vert_nb: usize
}

impl HasTess for GL33 {
  type Tess = GLTess;

  fn new_tess<T>(mode: Mode, vertices: &[T], indices: Option<&[u32]>) -> Self::Tess where T: Vertex {
    let mut vao: GLuint = 0;
    let vert_nb = vertices.len();

    unsafe {
      gl::GenVertexArrays(1, &mut vao);

      gl::BindVertexArray(vao);

      // vertex buffer
      let vertex_buffer = Buffer::new(vert_nb);
      vertex_buffer.fill(vertices);

      // once the vertex buffer is filled, we get its internal representation and we leak it so that
      // it’s not dropped at the end of the scope
      let vbo = vertex_buffer.repr.clone();
      mem::forget(vertex_buffer);

      gl::BindBuffer(gl::ARRAY_BUFFER, vbo.handle);
      set_vertex_pointers(&T::vertex_format());

      // in case of indexed render, create the required objects
      if let Some(indices) = indices {
        let ind_nb = indices.len();
        let index_buffer = Buffer::new(ind_nb);
        index_buffer.fill(indices);

        // same than vertex buffer, once the index buffer is filled, we leak it to the void
        let ibo = index_buffer.repr.clone();
        mem::forget(index_buffer);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo.handle);

        gl::BindVertexArray(0);

        GLTess {
          render: Box::new(move |size, instances| {
            gl::BindVertexArray(vao);

            set_point_line_size(mode, size);

            if instances == 1 {
              gl::DrawElements(opengl_mode(mode), ind_nb as GLsizei, gl::UNSIGNED_INT, ptr::null());
            } else if instances > 1 {
              gl::DrawElementsInstanced(opengl_mode(mode), ind_nb as GLsizei, gl::UNSIGNED_INT, ptr::null(), instances as GLsizei);
            } else {
              panic!("cannot index-render 0 instance");
            }
          }),
          vao: vao,
          vbo: Some(vbo),
          ibo: Some(ibo),
          vertex_format: T::vertex_format(),
          vert_nb: vert_nb
        }
      } else {
        gl::BindVertexArray(0);

        GLTess {
          render: Box::new(move |size, instances| {
            gl::BindVertexArray(vao);

            set_point_line_size(mode, size);

            if instances == 1 {
              gl::DrawArrays(opengl_mode(mode), 0, vert_nb as GLsizei);
            } else if instances > 1 {
              gl::DrawArraysInstanced(opengl_mode(mode), 0, vert_nb as GLsizei, instances as GLsizei);
            } else {
              panic!("cannot render 0 instance");
            }
          }),
          vao: vao,
          vbo: Some(vbo),
          ibo: None,
          vertex_format: T::vertex_format(),
          vert_nb: vert_nb
        }
      }
    }
  }

  fn destroy_tess(tess: &mut Self::Tess) {
    // delete vertex array and all bound buffers
    unsafe {
      gl::DeleteVertexArrays(1, &tess.vao);

      if let &Some(ref vbo) = &tess.vbo {
        gl::DeleteBuffers(1, &vbo.handle);
      }

      if let &Some(ref ibo) = &tess.ibo {
        gl::DeleteBuffers(1, &ibo.handle);
      }
    }
  }

  fn attributeless(mode: Mode, vert_nb: usize) -> Self::Tess {
    let mut vao = 0;

    unsafe {
      gl::GenVertexArrays(1, &mut vao);

      gl::BindVertexArray(vao);
      gl::BindVertexArray(0);

      GLTess {
        render: Box::new(move |size, instances| {
          gl::BindVertexArray(vao);

          set_point_line_size(mode, size);

          if instances == 1 {
            gl::DrawArrays(opengl_mode(mode), 0, vert_nb as GLsizei);
          } else if instances > 1 {
            gl::DrawArraysInstanced(opengl_mode(mode), 0, vert_nb as GLsizei, instances as GLsizei);
          } else {
            panic!("cannot render 0 instance");
          }
        }),
        vao: vao,
        vbo: None,
        ibo: None,
        vertex_format: Vec::new(),
        vert_nb: vert_nb
      }
    }
  }

  fn vertex_format(tesse: &Self::Tess) -> &VertexFormat {
    &tesse.vertex_format
  }

  fn get_vertex_buffer_ref_mut(tess: &mut Self::Tess) -> Option<(&mut Self::ABuffer, usize)> {
    let vert_nb = tess.vert_nb;
    tess.vbo.as_mut().map(|vbo| (vbo, vert_nb))
  }
}

// Give OpenGL types information on the content of the VBO by setting vertex formats and pointers
// to buffer memory.
fn set_vertex_pointers(formats: &[VertexComponentFormat]) {
  let offsets = aligned_offsets(formats);
  let vertex_weight = offset_based_vertex_weight(formats, &offsets) as GLsizei;

  for (i, (format, off)) in formats.iter().zip(offsets).enumerate() {
    set_component_format(i as u32, vertex_weight, off, format);
  }
}

fn set_component_format(i: u32, stride: GLsizei, off: usize, f: &VertexComponentFormat) {
  match f.comp_type {
    Type::Floating => {
      unsafe {
        gl::VertexAttribPointer(i as GLuint, dim_as_size(&f.dim), opengl_sized_type(&f), gl::FALSE, stride, ptr::null::<GLvoid>().offset(off as isize));
      }
    },
    Type::Integral | Type::Unsigned | Type::Boolean => {
      unsafe {
        gl::VertexAttribIPointer(i as GLuint, dim_as_size(&f.dim), opengl_sized_type(&f), stride, ptr::null::<GLvoid>().offset(off as isize));
      }
    }
  }

  unsafe {
    gl::EnableVertexAttribArray(i as GLuint);
  }
}

fn dim_as_size(d: &Dim) -> GLint {
  match *d {
    Dim::Dim1 => 1,
    Dim::Dim2 => 2,
    Dim::Dim3 => 3,
    Dim::Dim4 => 4
  }
}

fn opengl_sized_type(f: &VertexComponentFormat) -> GLenum {
  match (f.comp_type, f.unit_size) {
    (Type::Integral, 1) => gl::BYTE,
    (Type::Integral, 2) => gl::SHORT,
    (Type::Integral, 4) => gl::INT,
    (Type::Unsigned, 1) | (Type::Boolean, 1) => gl::UNSIGNED_BYTE,
    (Type::Unsigned, 2) => gl::UNSIGNED_SHORT,
    (Type::Unsigned, 4) => gl::UNSIGNED_INT,
    (Type::Floating, 4) => gl::FLOAT,
    _ => panic!("unsupported vertex component format: {:?}", f)
  }
}

// Compute offsets for all the vertex components according to the alignments provided.
fn aligned_offsets(formats: &[VertexComponentFormat]) -> Vec<usize> {
  let mut offsets = Vec::with_capacity(formats.len());
  let mut off = 0;

  // compute offsets
  for f in formats {
    off = off_align(off, f.align); // keep the current component format aligned
    offsets.push(off);
    off += component_weight(f); // increment the offset by the pratical size of the component
  }

  offsets
}

// Weight in bytes of a single vertex, taking into account padding so that the vertex stay correctly
// aligned.
fn offset_based_vertex_weight(formats: &[VertexComponentFormat], offsets: &[usize]) -> usize {
  if formats.is_empty() || offsets.is_empty() {
    return 0;
  }

  off_align(offsets[offsets.len() - 1] + component_weight(&formats[formats.len() - 1]), formats[0].align)
}

// Weight in bytes of a vertex component.
fn component_weight(f: &VertexComponentFormat) -> usize {
  dim_as_size(&f.dim) as usize * f.unit_size
}

fn opengl_mode(mode: Mode) -> GLenum {
  match mode {
    Mode::Point => gl::POINTS,
    Mode::Line => gl::LINES,
    Mode::LineStrip => gl::LINE_STRIP,
    Mode::Triangle => gl::TRIANGLES,
    Mode::TriangleFan => gl::TRIANGLE_FAN,
    Mode::TriangleStrip => gl::TRIANGLE_STRIP
  }
}

fn set_point_line_size(mode: Mode, size: Option<f32>) {
  let computed = size.unwrap_or(1.);

  match mode {
    Mode::Point => unsafe { gl::PointSize(computed) },
    Mode::Line | Mode::LineStrip => unsafe { gl::LineWidth(computed) },
    _ => {}
  }
}

// Align an offset.
#[inline]
fn off_align(off: usize, align: usize) -> usize {
  let a = align - 1;
  (off + a) & !a
}
