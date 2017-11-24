
extern crate wayland_server;
extern crate xcb;
extern crate nix;
extern crate libloading;
#[macro_use]
extern crate lazy_static;
extern crate wayland_sys;
#[macro_use]
extern crate error_chain;

mod egl;

fn main() {

    let (mut display, mut event_loop) = wayland_server::create_display();

    let socket_name = display.add_socket_auto().unwrap();
    println!("socket_name = {:?}", socket_name);

    egl::egl_init(display.get_raw_display());

    event_loop.run().unwrap();
}
