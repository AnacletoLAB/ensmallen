
use xxhash_rust::xxh3::Xxh3;

#[derive(Clone, Send)]
pub enum Hasher {
    Simple(u64),
    XorShift(u64),
    Crc32(),
    Xxh3(Xxh3),
    SipHash3(),
}

impl Hasher {
    pub fn new(hasher_name: &str) -> Result<Self> {
        match hasher_name {
            "simple" => Ok(Hasher::Simple(0x88b0fa3d8539f266)),
            "xorshift" => Ok(Hasher::XorShift(0x88b0fa3d8539f266)),
            "crc32" => Ok(Hasher::Crc32()),
            "xxh3" => Ok(Hasher::Xxh3(Xxh3::new())),
            "siphash3" => Ok(Hasher::SipHash3()),
            _ => Err(),
        }
    }

    pub fn digest(self) -> u32 {
        match self {
            Hasher::Simple(state) => state as u32,
            Hasher::XorShift(state) => state as u32,
            Hasher::Xxh3(hasher) => hasher.digest() as u32,
            _ => todo!(),
        }
    }
}

pub trait UpdateHash<T> {
    fn update(&mut self, value: &T);
}

impl UpdateHash<u32> for Hasher {
    fn update(&mut self, value: &u32) {
        match self {
            Hasher::Simple(state) => {
                *state = (state ^ (value as u64)).wrapping_add(0xf01d12535da3ac14);
            },
            Hasher::XorShift(state) => {
                *state = state.wrapping_mul(value as u64 ^ 0x45dc0d8545fc1901);
                *state ^= state << 13;
                *state ^= state >> 7;
                *state ^= state << 17;
            },
            Hasher::Xxh3(hasher) => {
                hasher.update();
                hasher.update(state);
            },
            _ => todo!(),
        }
    }
}

impl UpdateHash<u64> for Hasher {
    fn update(&mut self, value: &u64) {
        match self {
            Hasher::Simple(state) => {
                *state = (state ^ value).wrapping_add(0x5d3612daf380e1b7);
            }
            Hasher::XorShift(state) => {
                *state = state.wrapping_mul(value ^ 0x0c72cf2867062df2);
                *state ^= state << 13;
                *state ^= state >> 7;
                *state ^= state << 17;
            }
            _ => todo!(),
        }
    }
}

impl<'a, T> UpdateHash<Option<T>> for Hasher 
where 
    Self: UpdateHash<T>
{
    fn update(&mut self, value: &Option<T>) {
        match value {
            None => self.update(0x2be836c6d40bb19f),
            Some(val) => {
                self.update(0x0ec2e2c6b5ee9393);
                self.update(val);
            }
        }
    }
}

impl<T> UpdateHash<[T]> for Hasher 
where 
    Self: UpdateHash<T>
{
    fn update(&mut self, value: &[T]) {
        self.update(0xd97a1905a8a4ef70);
        value.iter().for_each(|val| {
            self.update(val);
        });
    }
}

impl<T> UpdateHash<(T,)> for Hasher 
where 
    Self: UpdateHash<T>
{
    fn update(&mut self, value: &(T,)) {
        self.update(0x1b3e4e28bb12f61d);
        self.update(value.0);
    }
}

impl<T1, T2> UpdateHash<(T1, T2)> for Hasher 
where 
    Self: UpdateHash<T1> + UpdateHash<T2>
{
    fn update(&mut self, value: &(T1, T2)) {
        self.update(0x9a77696fa75a0413);
        self.update(value.0);
        self.update(value.1);
    }
}

impl<T1, T2, T3> UpdateHash<(T1, T2, T3)> for Hasher 
where 
    Self: UpdateHash<T1> + UpdateHash<T2> + UpdateHash<T3>
{
    fn update(&mut self, value: &(T1, T2, T3)) {
        self.update(0xdb34310d1e8ba528);
        self.update(value.0);
        self.update(value.1);
        self.update(value.2);
    }
}