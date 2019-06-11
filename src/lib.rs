/// Memory allocation
pub mod arena;

/// Bounding volume hierarchy
pub mod bvh;

/// Camera model
pub mod camera;

/// Algorithms for shading surfaces
pub mod integrator;

/// Lighting primitives
pub mod light;

/// Scattering models
pub mod bxdf;

/// External models
pub mod model;

/// Progress bar
pub mod progress;

/// Geometric primitives
pub mod math;

/// Full scene information
pub mod scene;

/// Intersection and shape models
pub mod geom;

/// Statistic tracking
pub mod stats;

/// Coloring textures
pub mod texture;

pub mod prelude {
    pub use crate::bxdf::BxDF;
    pub use crate::integrator::Integrator;
    pub use crate::light::Light;
    pub use crate::geom::Surface;
    pub use crate::texture::Texture;
}
