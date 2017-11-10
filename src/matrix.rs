/// 2x2 Matrix
#[derive(Debug, Clone, Copy)]
pub struct M22 {
    
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
}

impl M22 {
    // constants

    pub const IDENTITY: M22 = M22 { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0 };
    // methods

    
    // associated functions

    pub fn new(m11: f32, m12: f32, m21: f32, m22: f32) -> Self {
        M22 { m11, m12, m21, m22 }
    }
}