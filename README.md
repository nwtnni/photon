# photon

Path tracer inspired by [PBRT][1] and [CS 4620][2].
Initially based off of [Ray Tracing in One Weekend][3], but now modified extensively.

# Features

- Basic progress bar
- Multithreading using [`rayon`][4]
- Vector operations using [macros to reduce boilerplate][5]
- Simple [bump allocator][6]
- OBJ triangle mesh support
- [Bounding volume hierarchy][7] constructed with [SAH heuristic][8] for intersection acceleration
- Scene description DSL

# Examples

### Cornell Box

![Cornell box][9]

[1]: https://www.pbrt.org/
[2]: http://www.cs.cornell.edu/courses/cs4620/2018fa/
[3]: https://github.com/petershirley/raytracinginoneweekend 
[4]: https://github.com/rayon-rs/rayon
[5]: src/geometry/vec.rs
[6]: src/arena.rs
[7]: src/bvh/tree.rs
[8]: https://medium.com/@bromanz/how-to-create-awesome-accelerators-the-surface-area-heuristic-e14b5dec6160
[9]: renders/cornell-box.png
