use wayland_client::Connection;
use wayland_client::Dispatch;
use wayland_client::EventQueue;
use wayland_client::Proxy;
use wayland_client::QueueHandle;
use wayland_client::protocol::wl_display::WlDisplay;

pub fn connect() -> (Connection, WlDisplay) {
    let connection = Connection::connect_to_env().unwrap();
    let display = connection.display();
    (connection, display)
}

pub fn spawn_queue<T>(connection: &Connection) -> (EventQueue<T>, QueueHandle<T>) {
    let event_queue = connection.new_event_queue();
    let qh = event_queue.handle();
    (event_queue, qh)
}
