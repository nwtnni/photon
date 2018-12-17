use noisy_float::prelude::*;

pub struct Transform {
    m: [[N32; 4]; 4],
    m_inv: [[N32; 4]; 4],
}

impl Transform {

}

impl Default for Transform {
    fn default() -> Self {
        let identity = [
            [n32(1.0), n32(0.0), n32(0.0), n32(0.0)],
            [n32(0.0), n32(1.0), n32(0.0), n32(0.0)],
            [n32(0.0), n32(0.0), n32(1.0), n32(0.0)],
            [n32(0.0), n32(0.0), n32(0.0), n32(1.0)],
        ];
        Transform {
            m: identity.clone(),
            m_inv: identity,
        }
    }
}
