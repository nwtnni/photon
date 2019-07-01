#[derive(Clone, Debug)]
pub enum Token {
    String(String),
    Int(i32),
    Float(f32),

    Width,
    Height,
    Samples,

    Camera,
    Integrator,

    Surface,
    Light,
    BxDF,

    Normal,
    Path,
 
    Sphere,
    Quad,
    Point,
    Mesh,

    Glazed,
    Mirror,
    Lambertian,
    Specular,
}
