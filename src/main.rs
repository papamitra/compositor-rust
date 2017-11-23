
extern crate wayland_server;
extern crate xcb;
extern crate nix;
extern crate libloading;
#[macro_use]
extern crate lazy_static;
extern crate wayland_sys;

mod egl;
mod display;

fn main() {

    let mut display = display::Display::create_display();

    let socket_name = display.add_socket_auto().unwrap();
    println!("socket_name = {}", socket_name);

    egl::egl_init(display.get_raw_display());

    display.run();
}
