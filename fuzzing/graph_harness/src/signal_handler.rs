use super::*;
use std::thread;
use libc::{signal, sighandler_t};
use std::sync::Mutex;
use std::collections::HashMap;

type sig_num = i32;

lazy_static! {
    static ref SIG_HANDLER_DATA: Mutex<HashMap<u64, FromVecHarnessParams>> = {
        Mutex::new(HashMap::new())
    }; 
}

pub(crate) fn abrt_handler(sig_num: sig_num) {
    let mut data_map = SIG_HANDLER_DATA.lock().expect("Cannot get the lock for the sig handlers data");
    let data = data_map.get(&u64::from(thread::current().id().as_u64())).unwrap();

    handle_panics_from_vec(None, data.clone(), Some(sig_num));
}


pub(crate) fn register_handler(sig_num: sig_num, handler: fn(sig_num), data: FromVecHarnessParams) {
    unsafe{
        signal(sig_num, 
            std::mem::transmute::<fn(sig_num), sighandler_t>(handler)
        )
    };

    let mut data_map = SIG_HANDLER_DATA.lock().expect("Cannot get the lock for the sig handlers data");    
    let data = data_map.insert(u64::from(thread::current().id().as_u64()), data);
}