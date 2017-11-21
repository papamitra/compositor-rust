
extern crate wayland_server;
extern crate xcb;
extern crate nix;
extern crate libloading;
#[macro_use]
extern crate lazy_static;

mod egl;

fn main() {
    let (mut display, mut event_loop) = wayland_server::create_display();

    let socket_name = display.add_socket_auto().unwrap();

    egl::egl_init();

    println!("Hello, wayland!: {:?}", socket_name);

    event_loop.run().unwrap();
}
