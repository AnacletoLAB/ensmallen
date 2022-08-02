#![feature(atomic_from_mut)]

pub mod atomic;

pub mod prelude {
    pub use super::atomic::*;
}