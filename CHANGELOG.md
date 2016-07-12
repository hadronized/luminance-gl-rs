## 0.5.0

- Fixed viewport issue.
- Removed the need of **core**.
- Removed `UniformName`.
- Fixed the `update_textures` function regarding **luminance**.
- Using `AsRef` for `update_textures`.
- Adapted mipmaps as `usize`.
- Panic if unknown pixel format.

### 0.4.3

- Implemented `Uniform` for `Texture`.

### 0.4.2

- Fixed `HasFramebuffer::free_framebuffer`.

### 0.4.1

- Crate fixed because of *0.4.0* being broken then yanked.

## 0.4.0

- Implemented existential quantification for `Pipeline`.
- Added travis CI support.

### 0.3.2

- Added `ProgramProxy` in the export list of lib.rs.

### 0.3.1

- Added `ProgramProxy` alias.

## 0.3.0

- `Program` now has its *uniform interface* tagged in the type.
- Added support for `luminance-0.4.0`.

### 0.2.1

- Removed `W` use from `Buffer` as it was removed in `luminance-0.3.0`.

## 0.2.0

- Added support for `luminance-0.2.0`.

## 0.1.0

- Initial revision.
