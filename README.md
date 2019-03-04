# photon

A Rust implementation of [Ray Tracing in One Weekend][1], by Peter Shirley.

# Features

- Live preview using an [`sdl2`][2] canvas
- Basic progress bar
- Multithreading using [`rayon`][3]
- Vector operations using [macros to reduce boilerplate][4]
- Simple [bump allocator][10]
- OBJ triangle mesh support

# Examples

Note: all images in the `render` directory except `full.png` were converted from .ppm to .png,
since they were rendered before I switched to using [`lodepng`][5] for PNG encoding.

### Metal

![Metal spheres][6]

### Glass

![Glass spheres][7]

### Depth of Field

![Depth of field][8]

### Full Scene

![Full scene][9]

### Motion Blur

![Motion blur][11]

### Triangle Mesh

![Triangle mesh][12]

### Translation

![Bunnies][13]

[1]: https://github.com/petershirley/raytracinginoneweekend
[2]: https://github.com/Rust-SDL2/rust-sdl2 
[3]: https://github.com/rayon-rs/rayon
[4]: src/geometry/vec.rs
[5]: https://github.com/kornelski/lodepng-rust 
[6]: renders/metal.png
[7]: renders/dielectric.png
[8]: renders/focus.png
[9]: renders/full.png
[10]: src/arena.rs
[11]: renders/motion.png
[12]: renders/dragon.png
[13]: renders/bunnies.png
