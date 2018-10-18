pub mod base_instance;
pub mod scoped_tracepoint;

#[cfg(feature = "profiling_enabled")]
pub mod hawktracer_instance_file;

#[cfg(feature = "profiling_enabled")]
pub mod hawktracer_instance_tcp;
