extern crate rust_hawktracer;
use rust_hawktracer::*;
use std::fs;
mod utils;
use utils::*;

#[test]
fn tracing_test_to_file() {
    let file_name = "file_name.htdump";
    fs::remove_file(file_name);
    let hawktracer_instance = HawktracerInstance::new();
    let _listener = hawktracer_instance.create_listener(HawktracerListenerType::ToFile {
        file_path: file_name.into(),
        buffer_size: 4096,
    });

    do_work();

    #[cfg(feature = "profiling_enabled")]
    {
        assert!(std::path::Path::new(file_name).exists());
        fs::remove_file(file_name);
    }
}