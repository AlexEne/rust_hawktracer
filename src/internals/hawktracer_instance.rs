include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


pub struct HawktracerInstance {
}

impl HawktracerInstance {
    pub fn new() -> HawktracerInstance {
        use std;
        use std::os::raw::c_char;
        let p: *mut *mut c_char = std::ptr::null_mut();
        unsafe {
            ht_init(0, p);
        }
        HawktracerInstance {}      
    }
    
    pub fn touch(&self) {}
}

impl Drop for HawktracerInstance {
    fn drop(&mut self) {
        unsafe {
            ht_deinit();
        }
    }
}