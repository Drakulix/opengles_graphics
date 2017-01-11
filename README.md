# opengles_graphics [![Build Status](https://travis-ci.org/PistonDevelopers/opengl_graphics.svg)](https://travis-ci.org/Drakulix/opengles_graphics) [![Crates.io](https://img.shields.io/crates/v/opengles_graphics.svg)](https://crates.io/crates/opengles_graphics) [![Crates.io](https://img.shields.io/crates/l/opengles_graphics.svg)](https://github.com/Drakulix/opengles_graphics/blob/master/LICENSE)

An OpenGL ES 2D back-end for the Piston game engine

Maintainers: @drakulix
Regulary updated fork of [PistonDevelopers/opengl_graphics](https://github.com/PistonDevelopers/opengl_graphics)

### Important!

OpenGL ES needs to load function pointers before use.
If you are experiencing strange error messages like "X not loaded" this is likely the case.
To do this, see the README in [gl-rs](https://github.com/bjz/gl-rs)


The necessary GL interface is exposed under `opengles_graphics::gl`.
Currently only supportes GLSL Version 100 & GL ES Version 2.0.

### Contributions welcome
