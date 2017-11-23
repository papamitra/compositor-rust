
use std::ffi::{CStr};
use std::ptr;

use wayland_sys::server::*;

error_chain!{}

pub struct Display {
    raw_display: *mut wl_display
}

impl Display {
    pub fn create_display() -> Display {
        unsafe {
            Display { raw_display: wl_display_create() }
        }
    }

    pub fn add_socket_auto(&self) -> Result<String> {
        unsafe {
            let socket_name_ptr = wl_display_add_socket_auto(self.raw_display);
            if socket_name_ptr == ptr::null() {
                bail!("wl_display_add_socket_auto failed")
            }

            let socket_name = CStr::from_ptr(socket_name_ptr).to_string_lossy().into_owned();

            Ok(socket_name)
        }
    }

    pub fn run(&self) {
        unsafe {
            wl_display_run(self.raw_display);
        }
    }

    pub fn get_raw_display(&mut self) -> *mut wl_display {
        self.raw_display
    }
}
