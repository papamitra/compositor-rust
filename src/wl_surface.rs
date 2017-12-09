
use wayland_server::protocol::{wl_surface};

pub fn wl_surface_implementation() -> wl_surface::Implementation<()> {
    wl_surface::Implementation {
        destroy: |_evlh, _data, _client, _surface| {
            println!("call wl_surface.destroy()");
        },

        attach: |_evlh, _data, _client, _surface, _buffer, _x, _y| {
            println!("call wl_surface.attach()");
        },

        damage: |_evlh, _data, _client, _surface, _x, _y, _width, _height| {
            println!("call wl_surface.damage()");
        },

        frame: |_evlh, _data, _client, _surface, _callback| {
            println!("call wl_surface.frame()");
        },

        set_opaque_region: |_evlh, _data, _client, _surface, _region| {
            println!("call wl_surface.set_opaque_region()");
        },

        set_input_region: |_evlh, _data, _client, _surface, _region| {
            println!("call wl_surface.set_input_region()");
        },

        commit: |_, _, _, _| {
            println!("call wl_surface.commit()");
        },

        set_buffer_transform: |_, _, _, _, _| {
            println!("call wl_surface.set_buffer_transform()");
        },

        set_buffer_scale: |_, _, _, _, _| {
            println!("call wl_surface.set_buffer_scale()");
        },

        damage_buffer: |_, _, _, _, _, _, _, _| {
            println!("call wl_surface.damage_buffer()");
        },
    }
}

