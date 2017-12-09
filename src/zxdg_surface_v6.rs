
use wayland_protocols::unstable::xdg_shell::v6::server::{zxdg_surface_v6};

pub(crate) fn zxdg_surface_v6_implementation() -> zxdg_surface_v6::Implementation<()> {
    zxdg_surface_v6::Implementation {
        destroy: |_, _, _, _| {
        },

        get_toplevel: |_, _, _, _, _| {
            println!("call zxdg_surface_v6.get_toplevel()");
        },

        get_popup: |_,_,_,_,_,_,_| {
            println!("call zxdg_surface_v6.get_popup()");
        },

        set_window_geometry: |_, _, _, _, _, _, _, _| {
            println!("call zxdg_surface_v6.set_window_geometory()");
        },

        ack_configure: |_, _, _, _, _| {
            println!("call zxdg_surface_v6.ack_configure()");
        },

    }
}
