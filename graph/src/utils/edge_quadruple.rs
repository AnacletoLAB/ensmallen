/// Quadrule of string edge data
use crate::*;

#[derive(Clone, Debug)]
pub(crate) struct EdgeQuadruple(pub String, pub String, pub Option<String>, pub WeightT);

impl PartialEq for EdgeQuadruple {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) && (self.1 == other.1) && (self.2 == other.2) && (self.3.total_cmp(&other.3).is_eq())
    }
}

impl Eq for EdgeQuadruple {}

impl PartialOrd for EdgeQuadruple {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(
            self.0.partial_cmp(&other.0)?
                .then(self.1.partial_cmp(&other.1)?)
                .then(self.2.partial_cmp(&other.2)?)
                .then(self.3.total_cmp(&other.3))
        )
    }
}

impl Ord for EdgeQuadruple {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
            .then(self.1.cmp(&other.1))
            .then(self.2.cmp(&other.2))
            .then(self.3.total_cmp(&other.3))
    }
}