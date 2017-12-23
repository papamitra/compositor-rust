
extern crate wayland_server;
extern crate wayland_protocols;
extern crate xcb;
extern crate nix;
extern crate libloading;
#[macro_use]
extern crate lazy_static;
extern crate wayland_sys;
#[macro_use]
extern crate error_chain;

mod egl;
mod wl_compositor;
mod wl_surface;
mod zxdg_shell_v6;
mod zxdg_surface_v6;
mod zxdg_toplevel_v6;
mod surface;

fn main() {

    let (mut display, mut event_loop) = wayland_server::create_display();

    let socket_name = display.add_socket_auto().unwrap();
    println!("socket_name = {:?}", socket_name);

    let raw_display = unsafe { display.ptr() };
    let conn = egl::egl_init(raw_display).unwrap();

    wl_compositor::wl_compositor_init(&mut event_loop);
    zxdg_shell_v6::zxdg_shell_init(&mut event_loop);

    event_loop.run().unwrap();
}
