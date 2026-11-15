use smithay_client_toolkit::shm::Shm;
use wayland_client::QueueHandle;
pub trait Draw {
    fn get_buffer(&self) -> Shm;
    fn draw<T>(&self, queue: QueueHandle<T>);
}
