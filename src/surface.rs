
use wayland_server::protocol::{wl_surface, wl_callback, wl_buffer};
use wayland_server::Resource;

pub struct Surface {
    callback: Option<wl_callback::WlCallback>,
    buffer: Option<wl_buffer::WlBuffer>
}

impl Surface {
    pub fn new() -> Surface {
        Surface{callback: None, buffer: None}
    }

    pub fn print(&self) {
        println!("call Surface.print()");
    }

    pub fn set_callback(&mut self, callback: wl_callback::WlCallback) {
        self.callback = Some(callback);
    }

    pub fn set_buffer(&mut self, buffer: wl_buffer::WlBuffer) {
        self.buffer = Some(buffer);
    }

    pub fn callback_done(&mut self) {
        if let Some(ref callback) = self.callback {
            callback.done(0);
        }
        self.callback = None;
    }

    pub fn buffer_release(&mut self) {
        if let Some(ref buffer) = self.buffer {
            buffer.release();
        }
        self.buffer = None;
    }
}

pub(crate) fn create_surface(surface: &wl_surface::WlSurface) {
    unsafe {
        surface.set_user_data(Box::into_raw(Box::new(Surface::new())) as *mut _);
    }
}
