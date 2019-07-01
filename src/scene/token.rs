#[derive(Clone, Debug)]
pub enum Token {
    String(String),
    Float(f32),

    Camera,
    Integrator,

    Surface,
    Light,
    BxDF,
 
    Sphere,
    Quad,
    Point,
    Mesh,

    Glazed,
    Mirror,
    Lambertian,
    Specular,
}
