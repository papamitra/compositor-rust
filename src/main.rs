
extern crate wayland_server;
extern crate xcb;
extern crate nix;
extern crate libloading;
#[macro_use]
extern crate lazy_static;
extern crate wayland_sys;

use std::ffi::CStr;

use wayland_sys::server::*;

mod egl;

fn main() {
    unsafe {
        let display = wl_display_create();

        let socket_name_ptr = wl_display_add_socket_auto(display);
        let socket_name = CStr::from_ptr(socket_name_ptr).to_string_lossy().into_owned();
        println!("socket_name = {}", socket_name);

        egl::egl_init(display);

        wl_display_run(display);
    }
}
