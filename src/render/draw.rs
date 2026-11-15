use smithay_client_toolkit::{shell::wlr_layer::LayerSurface, shm::Shm};
use wayland_client::QueueHandle;

pub trait Render {
    // this allows for draw auto impl
    fn settings(&self) -> Settings;
    // currently CPU only
    fn draw<T>(&self, queue: &QueueHandle<T>) {
        let settings = self.settings();
        let width = settings.width;
        let height = settings.height;
        let surface = settings.surface;
        // multi buff support in future
        let num = settings.num;

        let stride = width * 4;

        let (buffer, canvas) = self
            .pool
            .create_buffer(
                width as i32,
                height as i32,
                stride as i32,
                wl_shm::Format::Argb8888,
            )
            .expect("create buffer");
        // todo: add actual render logic; might come tommorow
    }
}
pub struct RenderData {
    pub num: u8,
    pub width: u32,
    pub height: u32,
    pub surface: LayerSurface,
}
