
#[derive(Clone, Copy, Send)]
pub enum Hasher {
    Simple(u64),
    XorShift(u64),
    CRC32(),
    XXH3(),
    SipHash3(),
}

impl Hasher {
    pub fn new(hasher_name: &str) -> Result<Self> {
        match hasher_name {
            "simple" => Ok(Hasher::Simple(0x88b0fa3d8539f266)),
            "xorshift" => Ok(Hasher::XorShift(0x88b0fa3d8539f266)),
            "crc32" => Ok(Hasher::CRC32()),
            "xxh3" => Ok(Hasher::XXH3()),
            "siphash3" => Ok(Hasher::SipHash3()),
            _ => Err(),
        }
    }

    pub fn digest(self) -> u32 {
        match self {
            Hasher::Simple(state) => state as u32,
            Hasher::XorShift(state) => state as u32,
            _ => todo!(),
        }
    }
}

pub trait UpdateHash<T> {
    fn update(&mut self, value: T);
}

impl UpdateHash<u32> for Hasher {
    fn update(&mut self, value: u32) {
        match self {
            Hasher::Simple(state) => {
                *state = (state ^ (value as u64)).wrapping_add(0xf01d12535da3ac14);
            }
            Hasher::XorShift(state) => {
                *state = state.wrapping_mul(value as u64);
                *state ^= state << 7;
                *state ^= state >> 13;
                *state ^= state << 17;
            }
            _ => todo!(),
        }
    }
}

impl UpdateHash<u64> for Hasher {
    fn update(&mut self, value: u64) {
        match self {
            Hasher::Simple(state) => {
                *state = (state ^ value).wrapping_add(0x5d3612daf380e1b7);
            }
            Hasher::XorShift(state) => {
                *state = state ^ value;
                *state ^= state << 7;
                *state ^= state >> 13;
                *state ^= state << 17;
            }
            _ => todo!(),
        }
    }
}