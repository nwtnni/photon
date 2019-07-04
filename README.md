# photon

Path tracer inspired by [PBRT][1] and [CS 4620][7]

# Features

- Basic progress bar
- Multithreading using [`rayon`][2]
- Vector operations using [macros to reduce boilerplate][3]
- Simple [bump allocator][4]
- OBJ triangle mesh support
- Scene description DSL

# Examples

### Triangle Mesh

![Triangle mesh][5]

### Cornell Box

![Cornell box][6]

[1]: https://www.pbrt.org/
[2]: https://github.com/rayon-rs/rayon
[3]: src/geometry/vec.rs
[4]: src/arena.rs
[5]: renders/dragon.png
[6]: renders/cornell-box.png
[7]: http://www.cs.cornell.edu/courses/cs4620/2018fa/
