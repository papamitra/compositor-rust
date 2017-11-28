
use wayland_server::{EventLoop, EventLoopHandle, Client, Global};
use wayland_server::protocol::{wl_compositor};

fn compositor_bind(evlh: &mut EventLoopHandle, _: &mut (),
                   _: &Client, compositor: wl_compositor::WlCompositor) {
    evlh.register(
        &compositor,
        compositor_implementation(),
        (), None);
}

pub fn compositor_init(evl: &mut EventLoop) -> Global<wl_compositor::WlCompositor, ()> {
    evl.register_global(4, self::compositor_bind, ())
}

fn compositor_implementation() -> wl_compositor::Implementation<()> {
    wl_compositor::Implementation {
        create_surface: |_evlh, _, _, _, _surface| {
            println!("call create_surface()");
        },

        create_region: |_evlh, _, _, _, _region| {
            println!("call create_region()");
        },

    }
}
