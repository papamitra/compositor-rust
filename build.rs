extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, StructGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(
        Api::Egl,
        (1, 5),
        Profile::Core,
        Fallbacks::All,
        [
            "EGL_EXT_platform_base",
            "EGL_EXT_platform_x11",
            "EGL_KHR_platform_x11",
            "EGL_KHR_platform_wayland",
            "EGL_EXT_platform_wayland",
            "EGL_EXT_platform_device",
            "EGL_WL_bind_wayland_display",
        ],
    ).write_bindings(StructGenerator, &mut file)
        .unwrap();

}
