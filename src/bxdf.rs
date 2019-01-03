pub trait BxDF {

}

pub enum Transfer {
    Reflect,
    Transmit,
    Scatter,
}

pub enum Material {
    Diffuse,
    Glossy,
    Specular,
}
