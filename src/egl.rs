use xcb;
use libloading::Library;
use nix::libc::{c_long, c_void, int32_t, uint64_t};
use std::ffi::{CStr, CString};
use std::ptr;
use std::os;

// for bindings.rs (generated by gl-generator)
pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
pub type khronos_uint64_t = uint64_t;
pub type khronos_ssize_t = c_long;
pub type EGLint = int32_t;
pub type EGLNativeDisplayType = NativeDisplayType;
pub type EGLNativePixmapType = NativePixmapType;
pub type EGLNativeWindowType = NativeWindowType;
pub type NativeDisplayType = *const c_void;
pub type NativePixmapType = *const c_void;
pub type NativeWindowType = *const c_void;

mod egl {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

lazy_static! {
    static ref LIBEGL: Library = Library::new("libEGL.so.1").unwrap();
}

fn load_egl(lib: &Library) -> egl::Egl {
    egl::Egl::load_with(|sym| {
        let name = CString::new(sym).unwrap();
        unsafe {
            let symbol = lib.get::<*mut c_void>(name.as_bytes());
            match symbol {
                Ok(x) => {
                    *x as *const _
                },
                Err(_) => ptr::null(),
            }
        }
    })
}

pub fn egl_init() {
    let (conn, screen_num) = xcb::Connection::connect_with_xlib_display().unwrap();

    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let foreground = conn.generate_id();
    xcb::create_gc(
        &conn,
        foreground,
        screen.root(),
        &[
            (xcb::GC_FOREGROUND, screen.black_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
        ],
    );

    let win = conn.generate_id();
    xcb::create_window(
        &conn,
        xcb::COPY_FROM_PARENT as u8,
        win,
        screen.root(),
        0,
        0,
        150,
        150,
        10,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &[
            (xcb::CW_BACK_PIXEL, screen.white_pixel()),
            (
                xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS,
            ),
        ],
    );

    xcb::map_window(&conn, win);
    conn.flush();

    let p = load_egl(&LIBEGL);
    unsafe {
        let egl_display = p.GetPlatformDisplay(
            egl::PLATFORM_X11_KHR,
            conn.get_raw_dpy() as *mut _,
            ptr::null(),
        );

        {
            let mut major : EGLint = 0;
            let mut minor : EGLint = 0;

            let ret = p.Initialize(egl_display, &mut major, &mut minor);
            assert!(ret == true as u32);
        }

        
    }
}
