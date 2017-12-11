
use wayland_server::{EventLoop, EventLoopHandle, Client, Global};
use wayland_protocols::unstable::xdg_shell::v6::server::{zxdg_shell_v6};

use zxdg_surface_v6::*;

fn zxdg_shell_bind(evlh: &mut EventLoopHandle, _: &mut (),
                   _: &Client, zxdg_shell: zxdg_shell_v6::ZxdgShellV6) {
    evlh.register(
        &zxdg_shell,
        zxdg_shell_implementation(),
        (), None);
}

pub fn zxdg_shell_init(evl: &mut EventLoop) -> Global<zxdg_shell_v6::ZxdgShellV6, ()> {
    evl.register_global(1, self::zxdg_shell_bind, ())
}

fn zxdg_shell_implementation() -> zxdg_shell_v6::Implementation<()> {
    zxdg_shell_v6::Implementation {
        destroy: |_, _, _, _| {},
        create_positioner: |_evlh, _, _, _, _positioner| {
            println!("call create_positioner()");
        },

        get_xdg_surface: |evlh, _, _, _, xdg_surface, _surface| {
            println!("call get_xdg_surface()");
            evlh.register(
                &xdg_surface,
                zxdg_surface_v6_implementation(),
                (), None);

            xdg_surface.configure(1);
        },

        pong: |_, _, _, _, _sefial| {
            println!("call pong()");
        },
    }
}
