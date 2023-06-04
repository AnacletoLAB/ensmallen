use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub};
use std::str::FromStr;

// Types used to represent edges, nodes and their types.
/// Type used to index the Nodes.
pub type NodeT = u32;
/// Type used to index the Node Types.
pub type NodeTypeT = u16;
/// Type used to index the Edges.
pub type EdgeT = u64;
/// Type used to index the Edge Types.
pub type EdgeTypeT = u16;
/// Type used for the weights of the edges.
pub type WeightT = f32;
/// Type used for the parameters of the walk such as the return weight (p),
/// and the explore weight (q).
pub type ParamsT = WeightT;
/// Type used to save contexts used for Skipgram and CBOW.
pub type Contexts = Vec<Vec<NodeT>>;
/// Type used to save a group of words indices.
pub type Words = Vec<NodeT>;
/// Type used to save the frequencies of words
pub type Frequencies = Vec<f64>;
/// Triple of edge data
pub type Triple = (NodeT, NodeT, Option<EdgeTypeT>);
/// Quadruple of edge data
pub type Quadruple = (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>);
/// Quadrule of string edge data
pub type StringQuadruple = (String, String, Option<String>, WeightT);
/// Symbol reserved to unmapped nodes for algoritms such as connected components.
pub const NODE_NOT_PRESENT: NodeT = NodeT::MAX;
pub const INDEX_NOT_PRESENT: usize = usize::MAX;

pub type Result<T> = std::result::Result<T, String>;

/// Trait used for the Vocabulary class.
/// It represent an unsigned integer that can be converted to and from usize.
/// This allows us to save memory using indicies of smaller size than u64
/// and it has no effects on performance because it's optimized away during
/// compilaton.
pub trait ToFromUsize:
    Clone
    + Display
    + Ord
    + Copy
    + AddAssign
    + Add
    + Sub<Output = Self>
    + Hash
    + FromStr
    + Sync
    + Send
    + Debug
    + Add<Output = Self>
{
    /// create the type from a usize
    fn from_usize(v: usize) -> Self;
    /// create an usize from the type
    fn to_usize(v: Self) -> usize;
    /// Retrun the maximum encodable number
    fn get_max() -> Self;

    fn checked_add(self, rhs: Self) -> Option<Self>;
}

/// Automatically implement the methods needed to convert from and to usize
/// for the given numerical type.
macro_rules! macro_impl_to_from_usize {
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

                #[inline(always)]
                fn get_max() -> $ty {
                    (0 as $ty).wrapping_sub(1)
                }

                #[inline(always)]
                fn checked_add(self, rhs: $ty) -> Option<$ty> {
                    self.checked_add(rhs)
                }
            }
        )*
    }
}

macro_impl_to_from_usize!(u8 u16 u32 u64 usize);

pub struct ThreadDataRaceAware<T> {
    pub(crate) value: UnsafeCell<T>,
}

unsafe impl<T> Sync for ThreadDataRaceAware<T> {}

impl<T> ThreadDataRaceAware<T> {
    pub fn new(value: T) -> ThreadDataRaceAware<T> {
        ThreadDataRaceAware {
            value: std::cell::UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> *mut T {
        self.value.get()
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}
