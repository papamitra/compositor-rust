
extern crate wayland_server;

fn main() {
    let (_display, mut event_loop) = wayland_server::create_display();

    println!("Hello, wayland!");

    event_loop.run().unwrap();
}
