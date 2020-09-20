use rand::Rng;
use std::path::Path;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789()*&^%$#@!~";
    
pub fn random_string(len: usize) -> String{    
    let mut rng = rand::thread_rng();
    
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn random_path() -> String {
    Path::new("/tmp").join(random_string(64)).to_str().unwrap().to_string()
}