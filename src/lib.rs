#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! An OpenGL ES back-end for Rust-Graphics

extern crate shader_version;
extern crate shaders_graphics2d_gles as shaders;
extern crate image;
extern crate graphics;
extern crate rusttype;
extern crate texture as texture_lib;

pub use shader_version::OpenGL;
pub use back_end::GlGraphics;
pub use texture::Texture;
pub use texture_lib::*;

pub mod shader_utils;
pub mod glyph_cache;
pub mod error;

#[allow(non_upper_case_globals, missing_docs)]
pub mod gl;

mod back_end;
mod texture;
mod draw_state;
