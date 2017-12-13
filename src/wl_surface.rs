
use wayland_server::protocol::{wl_surface};
use wayland_server::Resource;

use surface::*;

pub fn wl_surface_implementation() -> wl_surface::Implementation<()> {
    wl_surface::Implementation {
        destroy: |_evlh, _data, _client, _surface| {
            println!("call wl_surface.destroy()");
        },

        attach: |_evlh, _data, _client, surface, mut buffer, _x, _y| unsafe {
            println!("call wl_surface.attach()");
            let ptr = surface.get_user_data();
            let surface = &mut *(ptr as *mut Surface);

            if let Some(ref mut buffer) = buffer {
                surface.set_buffer(buffer.clone_unchecked());
            }
        },

        damage: |_evlh, _data, _client, _surface, _x, _y, _width, _height| {
            println!("call wl_surface.damage()");
        },

        frame: |_evlh, _data, _client, surface, callback| unsafe {
            println!("call wl_surface.frame()");

            let ptr = surface.get_user_data();
            let surface = &mut *(ptr as *mut Surface);
            surface.set_callback(callback);
        },

        set_opaque_region: |_evlh, _data, _client, _surface, _region| {
            println!("call wl_surface.set_opaque_region()");
        },

        set_input_region: |_evlh, _data, _client, _surface, _region| {
            println!("call wl_surface.set_input_region()");
        },

        commit: |_, _, _, surface| unsafe {
            println!("call wl_surface.commit()");
            let ptr = surface.get_user_data();
            let surface = &mut *(ptr as *mut Surface);

            surface.callback_done();
            surface.buffer_release();
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

