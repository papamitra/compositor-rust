
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
#[macro_use]
extern crate glium;

mod backend;
mod wl_compositor;
mod wl_surface;
mod zxdg_shell_v6;
mod zxdg_surface_v6;
mod zxdg_toplevel_v6;
mod surface;
mod context;

use glium::Surface;
use glium::glutin::{self, GlContext};

fn main() {

    let (mut display, mut event_loop) = wayland_server::create_display();

    let socket_name = display.add_socket_auto().unwrap();
    println!("socket_name = {:?}", socket_name);

    let raw_display = unsafe { display.ptr() };
    let backend = backend::Backend::new(raw_display).unwrap();

    let context = context::Context::new(backend).unwrap();

    wl_compositor::wl_compositor_init(&mut event_loop);
    zxdg_shell_v6::zxdg_shell_init(&mut event_loop);

    loop {
        event_loop.dispatch(Some(16)).unwrap();
        display.flush_clients();

        context.redraw();
    }
}
