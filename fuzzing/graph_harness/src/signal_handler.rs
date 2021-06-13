use super::*;
use libc::{sighandler_t, signal};
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;

type SigNum = i32;

lazy_static! {
    static ref SIG_HANDLER_DATA: Mutex<HashMap<u64, FromVecHarnessParams>> =
        { Mutex::new(HashMap::new()) };
}

pub fn abrt_handler(sig_num: SigNum) {
    let data_map = SIG_HANDLER_DATA
        .lock()
        .expect("Cannot get the lock for the sig handlers data");

    match data_map.get(&u64::from(thread::current().id().as_u64())) {
        Some(data) => {
            let _ = handle_panics_from_vec(None, data.clone(), Some(sig_num));
        }
        None => {}
    }
}

pub fn register_handler(sig_num: SigNum, handler: fn(SigNum), data: FromVecHarnessParams) {
    let mut data_map = SIG_HANDLER_DATA
        .lock()
        .expect("Cannot get the lock for the sig handlers data");
    data_map.insert(u64::from(thread::current().id().as_u64()), data);

    unsafe {
        signal(
            sig_num,
            std::mem::transmute::<fn(SigNum), sighandler_t>(handler),
        )
    };
}
