use super::hawktracer_listener::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "profiling_enabled")]
impl HawktracerListener for HawktracerListenerTCP {}

#[cfg(feature = "profiling_enabled")]
pub struct HawktracerListenerTCP {
    listener: *mut HT_TCPListener,
}

#[cfg(feature = "profiling_enabled")]
impl HawktracerListenerTCP {
    pub fn new(port: u32, buffer_size: usize) -> HawktracerListenerTCP {
        use std;

        let listener = unsafe {
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

        HawktracerListenerTCP { listener: listener }
    }
}

#[cfg(feature = "profiling_enabled")]
impl Drop for HawktracerListenerTCP {
    fn drop(&mut self) {
        unsafe {
            ht_timeline_flush(ht_global_timeline_get());
            ht_timeline_unregister_all_listeners(ht_global_timeline_get());
            ht_tcp_listener_destroy(self.listener);
        }
    }
}