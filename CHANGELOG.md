## 0.13.1

- **Deprecated.**

# 0.13

- Vertices (`Vertex`) are now aligned based on what decides the Rust compiler. This is very
  important, especially because of the version 0.15.0 adding non-32-bit vertex components: alignment
  and padding is now completely handled for you and you have nothing to care about.
- Changed the meaning of the semantic maps (uniforms). It is now required to provide a `Uniform` to
  build a new `Sem`. This is an improvement in the sense that the *unsafe* zone is restricted to the
  declaration of uniforms for a given program. This *unsafe* zone will be covered in a next release
  by a macro to make it safe.

# 0.12

- Support for luminance-0.15.0 (yeah, lazy changelog line, sorry :D).

# 0.11

- `UniformWarning::TypeMismatch` now includes the name of the uniform which type mismatches with the
  requested on.

# 0.10

- Changed the pipeline workflow by introducing `Pipe` objects.
- Removed strong typing in shader programs (`Program<T>` is now `Program`).
- Removed strong typing in shader stages (`Stage<T>` is now `Stage`).

## 0.9.1

- Fixed segfault when a Tessellation with no bound buffer gets dropped.

# 0.9

- Added attribute-less tessellations.
- Enhanced shader-related documentation.
- Removed `Slot`.

# 0.8

- Support of texture / uniform buffer sets.

## 0.7.1

# 0.7

- Textures support in shaders.

## 0.6.2

- Replaced some internal references to `Vec` by slices.

## 0.6.1

- Fixed memory corruption in new_shader / new_program.

# 0.6

- Uniform warnings.

## 0.5.6

- Fixed runtime reification of uniforms.

## 0.5.5

- Support for runtime reification of uniforms.

## 0.5.4

- Added support for getting textures’ texels.

## 0.5.3

- Support for raw texture uploads.

## 0.5.2

- Added documentation link.

## 0.5.1

- Fixed vertex input offsets. That issue makes all prior versions fail when trying to handle
  multi-attributes vertices. You are very advised to upgrade to this version then.

# 0.5

- Fixed viewport issue.
- Removed the need of **core**.
- Removed `UniformName`.
- Fixed the `update_textures` function regarding **luminance**.
- Using `AsRef` for `update_textures`.
- Adapted mipmaps as `usize`.
- Panic if unknown pixel format.

## 0.4.3

- Implemented `Uniform` for `Texture`.

## 0.4.2

- Fixed `HasFramebuffer::free_framebuffer`.

## 0.4.1

- Crate fixed because of *0.4.0* being broken then yanked.

# 0.4

- Implemented existential quantification for `Pipeline`.
- Added travis CI support.

## 0.3.2

- Added `ProgramProxy` in the export list of lib.rs.

## 0.3.1

- Added `ProgramProxy` alias.

# 0.3

- `Program` now has its *uniform interface* tagged in the type.
- Added support for `luminance-0.4.0`.

## 0.2.1

- Removed `W` use from `Buffer` as it was removed in `luminance-0.3.0`.

# 0.2

- Added support for `luminance-0.2.0`.

# 0.1

- Initial revision.
