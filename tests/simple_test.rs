use std::{thread, time};

extern crate rust_hawktracer;
use rust_hawktracer::*;
use std::fs;

fn do_work(){ 
    scoped_tracepoint!(_work);
    
    for _ in 0..3 {
        scoped_tracepoint!(_test);
        thread::sleep(time::Duration::from_millis(30));
    }

    {
        scoped_tracepoint!(_test);
        thread::sleep(time::Duration::from_millis(10));
    }
}

#[test]
fn tracing_test_to_file() {
    let file_name = "file_name.htdump";
    fs::remove_file(file_name);
    let _listener = create_hawktracer_listener(HawktracerListenerType::ToFile {
        file_path: file_name.into(),
        buffer_size: 4096,
    });

    do_work();

    #[cfg(feature = "profiling_enabled")]
    {
        assert!(std::path::Path::new(file_name).exists());
    }
}


#[test]
fn tracing_test_network() {
    let _listener = create_hawktracer_listener(HawktracerListenerType::TCP {
        port: 12345,
        buffer_size: 4096,
    });

    do_work();
}


#[test]
fn tracing_test_two_listeners() {
    let file_name = "file_name.htdump";
    fs::remove_file(file_name);
    let _file_listener = create_hawktracer_listener(HawktracerListenerType::ToFile {
        file_path: file_name.into(),
        buffer_size: 4096,
    });

    let _network_listener = create_hawktracer_listener(HawktracerListenerType::TCP {
        port: 1111,
        buffer_size: 4096,
    });

    do_work();

    #[cfg(feature = "profiling_enabled")]
    {
        assert!(std::path::Path::new(file_name).exists());
    }
}