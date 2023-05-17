use ahash::AHasher;
use siphasher::sip::SipHasher24;
use std::hash::Hasher as _;
use xxhash_rust::xxh3::Xxh3;

#[derive(Clone)]
pub enum Hasher {
    Simple(u64),
    CommutativeSimple(u64),
    XorShift(u64),
    AHasher(AHasher),
    Xxh3(Xxh3),
    SipHash(SipHasher24),
}

impl Hasher {
    pub fn simple() -> Self {
        Hasher::Simple(0x88b0fa3d8539f266)
    }

    pub fn commutative_simple() -> Self {
        Hasher::CommutativeSimple(0)
    }

    pub fn digest(self) -> u64 {
        match self {
            Hasher::Simple(state) => state,
            Hasher::CommutativeSimple(state) => state,
            Hasher::XorShift(state) => state,
            Hasher::Xxh3(hasher) => hasher.digest(),
            Hasher::SipHash(hasher) => hasher.finish(),
            Hasher::AHasher(hasher) => hasher.finish(),
        }
    }
}

pub trait UpdateHash<T: ?Sized> {
    fn update(&mut self, value: &T);
}

impl UpdateHash<u8> for Hasher {
    fn update(&mut self, value: &u8) {
        match self {
            Hasher::Simple(state) => {
                *state = (*state ^ (*value as u64)).wrapping_add(0xed4e83c06c9fe588);
            }
            Hasher::CommutativeSimple(state) => {
                *state = *state | (1 << (value % 64));
            }
            Hasher::XorShift(state) => {
                *state = state.wrapping_mul(*value as u64 ^ 0x44d4c5a74c775ba0);
                *state ^= *state << 13;
                *state ^= *state >> 7;
                *state ^= *state << 17;
            }
            Hasher::Xxh3(hasher) => {
                hasher.update(&[0xe8, 0xa8, 0xef, 0x9d, 0xbe, 0xe1, 0x7c, 0x01]);
                hasher.update(&value.to_le_bytes());
            }
            Hasher::SipHash(hasher) => {
                hasher.write_u8(*value);
            }
            Hasher::AHasher(hasher) => {
                hasher.write_u8(*value);
            }
        }
    }
}

impl UpdateHash<u16> for Hasher {
    fn update(&mut self, value: &u16) {
        match self {
            Hasher::Simple(state) => {
                *state = (*state ^ (*value as u64)).wrapping_add(0xed4e83c06c9fe588);
            }
            Hasher::CommutativeSimple(state) => {
                *state = *state | (1 << (value % 64));
            }
            Hasher::XorShift(state) => {
                *state = state.wrapping_mul(*value as u64 ^ 0x44d4c5a74c775ba0);
                *state ^= *state << 13;
                *state ^= *state >> 7;
                *state ^= *state << 17;
            }
            Hasher::Xxh3(hasher) => {
                hasher.update(&[0xe8, 0xa8, 0xef, 0x9d, 0xbe, 0xe1, 0x7c, 0x01]);
                hasher.update(&value.to_le_bytes());
            }
            Hasher::SipHash(hasher) => {
                hasher.write_u16(*value);
            }
            Hasher::AHasher(hasher) => {
                hasher.write_u16(*value);
            }
        }
    }
}

impl UpdateHash<u32> for Hasher {
    fn update(&mut self, value: &u32) {
        match self {
            Hasher::Simple(state) => {
                *state = (*state ^ (*value as u64)).wrapping_add(0xf01d12535da3ac14);
            }
            Hasher::CommutativeSimple(state) => {
                *state = *state | (1 << (value % 64));
            }
            Hasher::XorShift(state) => {
                *state = state.wrapping_mul(*value as u64 ^ 0x45dc0d8545fc1901);
                *state ^= *state << 13;
                *state ^= *state >> 7;
                *state ^= *state << 17;
            }
            Hasher::Xxh3(hasher) => {
                hasher.update(&[0x67, 0x30, 0xf7, 0x12, 0x31, 0xc0, 0xa1, 0xd4]);
                hasher.update(&value.to_le_bytes());
            }
            Hasher::SipHash(hasher) => {
                hasher.write_u32(*value);
            }
            Hasher::AHasher(hasher) => {
                hasher.write_u32(*value);
            }
        }
    }
}

impl UpdateHash<u64> for Hasher {
    fn update(&mut self, value: &u64) {
        match self {
            Hasher::Simple(state) => {
                *state = (*state ^ value).wrapping_add(0x5d3612daf380e1b7);
            }
            Hasher::CommutativeSimple(state) => {
                *state = *state | (1 << (value % 64));
            }
            Hasher::XorShift(state) => {
                *state = state.wrapping_mul(value ^ 0x0c72cf2867062df2);
                *state ^= *state << 13;
                *state ^= *state >> 7;
                *state ^= *state << 17;
            }
            Hasher::Xxh3(hasher) => {
                hasher.update(&[0xec, 0xef, 0x7c, 0xae, 0x90, 0x60, 0xb2, 0x6f]);
                hasher.update(&value.to_le_bytes());
            }
            Hasher::SipHash(hasher) => {
                hasher.write_u64(*value);
            }
            Hasher::AHasher(hasher) => {
                hasher.write_u64(*value);
            }
        }
    }
}

impl<T> UpdateHash<Option<T>> for Hasher
where
    Self: UpdateHash<T>,
{
    fn update(&mut self, value: &Option<T>) {
        match value {
            None => {
                <Self as UpdateHash<u64>>::update(self, &0x2be836c6d40bb19f_u64);
            }
            Some(val) => {
                <Self as UpdateHash<u64>>::update(self, &0x0ec2e2c6b5ee9393_u64);
                self.update(val);
            }
        }
    }
}

impl<T> UpdateHash<[T]> for Hasher
where
    Self: UpdateHash<T>,
{
    fn update(&mut self, value: &[T]) {
        <Self as UpdateHash<u64>>::update(self, &0xd97a1905a8a4ef70_u64);
        value.iter().for_each(|val| {
            self.update(val);
        });
    }
}

impl<'a, T> UpdateHash<&'a [T]> for Hasher
where
    Self: UpdateHash<T>,
{
    fn update(&mut self, value: &&'a [T]) {
        <Self as UpdateHash<u64>>::update(self, &0xd97a1905a8a4ef70_u64);
        value.iter().for_each(|val| {
            self.update(val);
        });
    }
}

impl<T> UpdateHash<(T,)> for Hasher
where
    Self: UpdateHash<T>,
{
    fn update(&mut self, value: &(T,)) {
        <Self as UpdateHash<u64>>::update(self, &0x1b3e4e28bb12f61d_u64);
        <Self as UpdateHash<T>>::update(self, &value.0);
    }
}

impl<T1, T2> UpdateHash<(T1, T2)> for Hasher
where
    Self: UpdateHash<T1> + UpdateHash<T2>,
{
    fn update(&mut self, value: &(T1, T2)) {
        <Self as UpdateHash<u64>>::update(self, &0x9a77696fa75a0413_u64);
        <Self as UpdateHash<T1>>::update(self, &value.0);
        <Self as UpdateHash<T2>>::update(self, &value.1);
    }
}

impl<T1, T2, T3> UpdateHash<(T1, T2, T3)> for Hasher
where
    Self: UpdateHash<T1> + UpdateHash<T2> + UpdateHash<T3>,
{
    fn update(&mut self, value: &(T1, T2, T3)) {
        <Self as UpdateHash<u64>>::update(self, &0xdb34310d1e8ba528_u64);
        <Self as UpdateHash<T1>>::update(self, &value.0);
        <Self as UpdateHash<T2>>::update(self, &value.1);
        <Self as UpdateHash<T3>>::update(self, &value.2);
    }
}
