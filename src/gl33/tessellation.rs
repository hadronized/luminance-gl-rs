use gl;
use gl::types::*;
use gl33::buffer::Buffer;
use gl33::token::GL33;
use luminance::rw::W;
use luminance::tessellation::{self, HasTessellation, Mode};
use luminance::vertex::{Dim, Type, Vertex, VertexComponentFormat, VertexFormat};
use std::ptr;

pub type Tessellation = tessellation::Tessellation<GL33>;

impl HasTessellation for GL33 {
  // closure taking the number of instances to render
  type Tessellation = Box<Fn(u32)>;

  fn new<T>(mode: Mode, vertices: &Vec<T>, indices: Option<&Vec<u32>>) -> Self::Tessellation where T: Vertex {
    let mut vao: GLuint = 0;
    let vert_nb = vertices.len();

    unsafe {
      gl::GenVertexArrays(1, &mut vao);

      gl::BindVertexArray(vao);

      // vertex buffer
      let vertex_buffer = Buffer::new(W, vert_nb);
      vertex_buffer.fill(&vertices);

      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer.repr.handle);
      set_vertex_pointers(&T::vertex_format());

      // in case of indexed render, create the required objects
      if let Some(indices) = indices {
        let ind_nb = indices.len();
        let index_buffer = Buffer::new(W, ind_nb);

        index_buffer.fill(&indices);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.repr.handle);

        gl::BindVertexArray(0);

        Box::new(move |instances| {
          gl::BindVertexArray(vao);

          if instances == 1 {
            gl::DrawElements(from_mode(mode), ind_nb as GLsizei, gl::UNSIGNED_INT, ptr::null());
          } else if instances > 1 {
            gl::DrawElementsInstanced(from_mode(mode), ind_nb as GLsizei, gl::UNSIGNED_INT, ptr::null(), instances as GLsizei);
          } else {
            panic!("cannot index-render 0 instance");
          }
        })
      } else {
        gl::BindVertexArray(0);

        Box::new(move |instances| {
          gl::BindVertexArray(vao);

          if instances == 1 {
            gl::DrawArrays(from_mode(mode), 0, vert_nb as GLsizei);
          } else if instances > 1 {
            gl::DrawArraysInstanced(from_mode(mode), 0, vert_nb as GLsizei, instances as GLsizei);
          } else {
            panic!("cannot render 0 instance");
          }
        })
      }
    }
  }

  fn render(tess: &Self::Tessellation, instances: u32) {
    tess(instances);
  }
}

fn set_vertex_pointers(formats: &VertexFormat) {
  let stride = vertex_weight(formats) as GLsizei;

  for (i, format) in formats.iter().enumerate() {
    set_component_format(i as u32, stride, format);
  }
}

fn set_component_format(i: u32, stride: GLsizei, cf: &VertexComponentFormat) {
  unsafe {
    gl::VertexAttribPointer(i as GLuint, from_dim(&cf.dim), from_type(&cf.component_type), gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(i as GLuint);
  }
}

fn from_dim(d: &Dim) -> GLint {
  match *d {
    Dim::Dim1 => 1,
    Dim::Dim2 => 2,
    Dim::Dim3 => 3,
    Dim::Dim4 => 4
  }
}

fn from_type(t: &Type) -> GLenum {
  match *t {
    Type::Integral => gl::INT,
    Type::Unsigned => gl::UNSIGNED_INT,
    Type::Floating => gl::FLOAT,
    Type::Boolean => gl::INT
  }
}

fn vertex_weight(formats: &VertexFormat) -> usize {
  let mut weight: usize = 0;

  for f in formats {
    weight += from_dim(&f.dim) as usize * component_type_weight(&f.component_type);
  }

  weight
}

fn component_type_weight(t: &Type) -> usize {
  match *t {
    Type::Integral => 4,
    Type::Unsigned => 4,
    Type::Floating => 4,
    Type::Boolean => 1
  }
}

fn from_mode(mode: Mode) -> GLenum {
  match mode {
    Mode::Point => gl::POINTS,
    Mode::Line => gl::LINES,
    Mode::LineStrip => gl::LINE_STRIP,
    Mode::Triangle => gl::TRIANGLES,
    Mode::TriangleFan => gl::TRIANGLE_FAN,
    Mode::TriangleStrip => gl::TRIANGLE_STRIP
  }
}
