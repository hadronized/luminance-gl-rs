use gl;

pub fn debug_gl() {
  let e = unsafe { gl::GetError() };

  match e {
    gl::NO_ERROR => println!("no error"),
    gl::INVALID_ENUM => println!("invalid enum"),
    gl::INVALID_VALUE => println!("invalid value"),
    gl::INVALID_OPERATION => println!("invalid operation"),
    gl::INVALID_FRAMEBUFFER_OPERATION => println!("invalid frameuffer operation"),
    gl::OUT_OF_MEMORY => println!("out of memory"),
    _ => println!("unknown error: {}", e)
  }
}
