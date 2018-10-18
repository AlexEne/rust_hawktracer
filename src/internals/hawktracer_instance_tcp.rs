#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::base_instance::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "profiling_enabled")]
impl HawktracerInstance for HawktracerInstanceTCP {}

#[cfg(feature = "profiling_enabled")]
pub struct HawktracerInstanceTCP {
    listener: *mut HT_TCPListener,
}

#[cfg(feature = "profiling_enabled")]
impl HawktracerInstanceTCP {
    pub fn new(port: u32, buffer_size: usize) -> HawktracerInstanceTCP {
        use std;
        use std::os::raw::c_char;

        let p: *mut *mut c_char = std::ptr::null_mut();
        let listener = unsafe {
            ht_init(0, p);
            let listener = ht_tcp_listener_create(
                port as i32,
                buffer_size,
                std::ptr::null_mut() as _,
            );

            ht_timeline_register_listener(
                ht_global_timeline_get(),
                Some(ht_tcp_listener_callback),
                listener as _,
            );

            listener
        };

        HawktracerInstanceTCP { listener: listener }
    }
}

#[cfg(feature = "profiling_enabled")]
impl Drop for HawktracerInstanceTCP {
    fn drop(&mut self) {
        unsafe {
            ht_timeline_flush(ht_global_timeline_get());
            ht_timeline_unregister_all_listeners(ht_global_timeline_get());
            ht_tcp_listener_destroy(self.listener);
            ht_deinit();
        }
    }
}