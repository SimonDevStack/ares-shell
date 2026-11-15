use wayland_client::Connection;
use wayland_client::Dispatch;
use wayland_client::EventQueue;
use wayland_client::Proxy;
use wayland_client::QueueHandle;

pub fn connect() {
    let conn = Connection::connect_to_env().unwrap();

    let (globals, mut event_queue) = registry_queue_init(&conn).unwrap();
    let qh = event_queue.handle();

    let compositor = CompositorState::bind(&globals, &qh).expect("wl_compositor is not available");

    let layer_shell = LayerShell::bind(&globals, &qh).expect("layer shell is not available");
}
