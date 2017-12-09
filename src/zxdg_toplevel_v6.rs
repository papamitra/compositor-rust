
use wayland_protocols::unstable::xdg_shell::v6::server::{zxdg_toplevel_v6};

pub(crate) fn zxdg_toplevel_v6_implementation() -> zxdg_toplevel_v6::Implementation<()> {
    zxdg_toplevel_v6::Implementation {
        destroy: |_, _, _, _| {},

        set_parent: |_, _, _, _, _| {
            println!("call zxdg_toplevel_v6.set_parent()");
        },

        set_title: |_, _, _, _, _| {
            println!("call zxdg_toplevel_v6.set_title()");
        },

        set_app_id: |_, _, _, _, _| {
            println!("call zxdg_toplevel_v6.set_app_id()");
        },

        show_window_menu: |_, _, _, _, _, _, _, _| {
            println!("call zxdg_toplevel_v6.show_window_menu()");
        },

        move_: |_, _, _, _, _, _| {
            println!("call zxdg_toplevel_v6.move_()");
        },

        resize: |_,_,_,_,_,_,_| {
            println!("call zxdg_toplevel_v6.resize()");
        },

        set_max_size: |_, _, _, _, _, _| {
            println!("call zxdg_toplevel_v6.set_max_size()");
        },

        set_min_size: |_, _, _, _, _, _| {
            println!("call zxdg_toplevel_v6.set_min_size()");
        },

        set_maximized: |_, _, _, _| {
            println!("call zxdg_toplevel_v6.set_maximized()");
        },

        unset_maximized: |_, _, _, _| {
            println!("call zxdg_toplevel_v6.unset_maximized()");
        },

        set_fullscreen: |_, _, _, _, _| {
            println!("call zxdg_toplevel_v6.set_fullscreen()");
        },

        unset_fullscreen: |_, _, _, _| {
            println!("call zxdg_toplevel_v6.unset_fullscreen()");
        },

        set_minimized: |_, _, _, _| {
            println!("call zxdg_toplevel_v6.set_minimized()");
        },
    }
}
