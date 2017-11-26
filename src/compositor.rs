
use wayland_server::{EventLoop, EventLoopHandle, Client, Global};
use wayland_server::protocol::{wl_compositor};

fn compositor_bind(evlh: &mut EventLoopHandle, _: &mut (),
                   _: &Client, compositor: wl_compositor::WlCompositor) {
    println!("call compositor_bind");
}

pub fn compositor_init(evl: &mut EventLoop) -> Global<wl_compositor::WlCompositor, ()> {
    evl.register_global(4, self::compositor_bind, ())
}

