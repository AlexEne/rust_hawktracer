use std::{thread, time};

extern crate rust_hawktracer;
use rust_hawktracer::*;
use std::fs;

#[test]
fn tracing_test_to_file() {
    let file_name = "file_name.htdump";
    fs::remove_file(file_name);
    let _instance = create_hawktracer_instance(HawktracerInstanceType::ToFile {
        file_path: file_name.into(),
        buffer_size: 4096,
    });

    {
        for _ in 0..3 {
            scoped_tracepoint!(_test);
            thread::sleep(time::Duration::from_millis(30));
        }
    }
    {
        scoped_tracepoint!(_test);
        thread::sleep(time::Duration::from_millis(10));
    }


    #[cfg(feature = "profiling_enabled")]
    {
        assert!(std::path::Path::new(file_name).exists());
    }
}


#[test]
fn tracing_test_network() {
    let _instance = create_hawktracer_instance(HawktracerInstanceType::TCP {
        port: 35445,
        buffer_size: 4096,
    });

    {
        for _ in 0..3 {
            scoped_tracepoint!(_test);
            thread::sleep(time::Duration::from_millis(30));
        }
    }
    {
        scoped_tracepoint!(_test);
        thread::sleep(time::Duration::from_millis(10));
    }
}
