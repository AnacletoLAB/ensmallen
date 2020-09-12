use derive_getters::Getters;
use std::collections::HashSet;

// Types used to represent edges, nodes and their types.
pub type NodeT = usize;
pub type EdgeT = usize;
pub type WeightT = f64;
pub type ParamsT = f64;
pub type NodeTypeT = u16;
pub type EdgeTypeT = u16;

#[derive(Debug, Clone, Getters, PartialEq)]
pub(crate) struct EdgeMetadata {
    pub(crate) edge_id: EdgeT,
    pub(crate) edge_types: HashSet<EdgeTypeT>,
}

pub(crate) struct ConstructorEdgeMetadata {
    pub(crate) edge_types: Vec<EdgeTypeT>,
    pub(crate) weights: Vec<WeightT>,
}

pub(crate) impl ConstructorEdgeMetadata {
    pub(crate) fn new() -> ConstructorEdgeMetadata {
        ConstructorEdgeMetadata {
            edge_types: Vec::new(),
            weights: Vec::new(),
        }
    }
}

pub(crate) trait ToFromUsize {
    fn from_usize(v: usize) -> Self;
    fn to_usize(v: Self) -> usize;
}

macro_rules! impl_to_from_usize {
    ($($ty:ty)*) => {
        $(
            impl ToFromUsize for $ty {
                #[inline(always)]
                fn from_usize(v: usize) -> $ty {
                    v as $ty
                }
                #[inline(always)]
                fn to_usize(v: $ty) -> usize {
                    v as usize
                }
            }
        )*
    }
}

impl_to_from_usize!(u8 u16 u32 u64 usize);
