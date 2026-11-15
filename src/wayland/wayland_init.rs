use wayland_client::Connection;
use wayland_client::Dispatch;
use wayland_client::EventQueue;
use wayland_client::globals::GlobalList;
use wayland_client::globals::GlobalListContents;
use wayland_client::globals::registry_queue_init;
use wayland_client::protocol::wl_registry::WlRegistry;

pub fn connect() -> Connection {
    let conn = Connection::connect_to_env().unwrap();
    conn
}

fn spawn_queue<T: Dispatch<WlRegistry, GlobalListContents> + 'static>(
    conn: &Connection,
) -> (EventQueue<T>, GlobalList) {
    let (globals, event_queue) = registry_queue_init(&conn).unwrap();
    (event_queue, globals)
}
