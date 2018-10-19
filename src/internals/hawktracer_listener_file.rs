use super::hawktracer_listener::*;
#[allow(unused_imports)]
use std::path::PathBuf;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "profiling_enabled")]
pub struct HawktracerListenerFile {
    listener: *mut _HT_FileDumpListener,
}

#[cfg(feature = "profiling_enabled")]
impl<'a> HawktracerListener<'a> for HawktracerListenerFile {}

#[cfg(feature = "profiling_enabled")]
impl HawktracerListenerFile {
    pub fn new(file_path: PathBuf, buffer_size: usize) -> HawktracerListenerFile {
        let string_path = file_path.into_os_string().into_string().unwrap();
        let file_path = std::ffi::CString::new(string_path).unwrap();
        use std;
        let listener = unsafe {
            let listener = ht_file_dump_listener_create(
                file_path.as_ptr(),
                buffer_size,
                std::ptr::null_mut() as _,
            );

            ht_timeline_register_listener(
                ht_global_timeline_get(),
                Some(ht_file_dump_listener_callback),
                listener as _,
            );

            listener
        };

        HawktracerListenerFile { listener: listener }
    }
}

#[cfg(feature = "profiling_enabled")]
impl Drop for HawktracerListenerFile {
    fn drop(&mut self) {
        unsafe {
            ht_timeline_flush(ht_global_timeline_get());
            ht_timeline_unregister_all_listeners(ht_global_timeline_get());
            ht_file_dump_listener_destroy(self.listener);
        }
    }
}