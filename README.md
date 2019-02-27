# photon

A Rust implementation of [Ray Tracing in One Weekend][1], by Peter Shirley.

# Features

- Live preview using an [`sdl2`][5] canvas
- Multithreading using [`rayon`][6]
- Vector operations using [macros to reduce boilerplate][7]

# Examples

![Metal spheres][2]

![Glass spheres][3]

![Full scene][4]

[1]: https://github.com/petershirley/raytracinginoneweekend
[2]: renders/metal.ppm
[3]: renders/dielectric.ppm
[4]: renders/full.ppm
[5]: https://github.com/Rust-SDL2/rust-sdl2 
[6]: https://github.com/rayon-rs/rayon
[7]: src/geometry/vec.rs
