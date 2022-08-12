use express_measures::Coerced;

use crate::f16;

impl Coerced<f32> for f16 {
    fn coerce_into(self) -> f32 {
        self.to_f32()
    }
    fn coerce_from(other: f32) -> Self {
        f16::from_f32(other)
    }
}

impl Coerced<f64> for f16 {
    fn coerce_into(self) -> f64 {
        self.to_f64()
    }
    fn coerce_from(other: f64) -> Self {
        f16::from_f64(other)
    }
}
