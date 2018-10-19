include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "profiling_enabled")]
pub struct ScopedTracepoint;

#[cfg(feature = "profiling_enabled")]
impl ScopedTracepoint {
    pub fn start_trace(name: *mut i8) {
        unsafe {
            ht_feature_callstack_start_string(ht_global_timeline_get(), name);
        }
    }
}

#[cfg(feature = "profiling_enabled")]
impl Drop for ScopedTracepoint {
    fn drop(&mut self) {
        unsafe {
            ht_feature_callstack_stop(ht_global_timeline_get());
        }
    }
}


#[cfg(not(feature = "profiling_enabled"))]
pub struct ScopedTracepoint;
