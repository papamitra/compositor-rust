
use wayland_server::{EventLoop, EventLoopHandle, Client, Global};
use wayland_server::protocol::{wl_compositor};

use wl_surface::*;
use surface::*;

fn wl_compositor_bind(evlh: &mut EventLoopHandle, _: &mut (),
                   _: &Client, compositor: wl_compositor::WlCompositor) {
    evlh.register(
        &compositor,
        wl_compositor_implementation(),
        (), None);
}

pub fn wl_compositor_init(evl: &mut EventLoop) -> Global<wl_compositor::WlCompositor, ()> {
    evl.register_global(4, self::wl_compositor_bind, ())
}

fn wl_compositor_implementation() -> wl_compositor::Implementation<()> {
    wl_compositor::Implementation {
        create_surface: |evlh, _, _, _, surface| {
            println!("call create_surface()");

            create_surface(&surface);

            evlh.register(
                &surface,
                wl_surface_implementation(),
                (), None);
        },

        create_region: |_evlh, _, _, _, _region| {
            println!("call create_region()");
        },

    }
}
