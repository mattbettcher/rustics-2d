use {M22, V2};

#[allow(dead_code)]
pub struct MPR {}

impl MPR {
    #[allow(dead_code)]
    pub fn detect<T>(
        _support_a: T,
        _support_b: T,
        _position_a: &V2,
        _position_b: &V2,
        _orientation_a: &M22,
        _orienation_b: &M22,
    ) -> (V2, V2, f32) {
        unimplemented!()
    }
}
