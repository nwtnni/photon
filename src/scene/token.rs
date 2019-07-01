#[derive(Clone, Debug)]
pub enum Token {
    String(String),
    Float(f32),

    Scene,
    Camera,
    Integrator,

    Surface,
    Light,
 
    Sphere,
    Quad,
    Point,
    Mesh,

    Glazed,
    Mirror,
    Lambertian,
    Specular,
}
