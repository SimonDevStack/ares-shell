use crate::render::draw::{self, Draw};
use smithay_client_toolkit::shm::Shm;
use smithay_client_toolkit::shm::slot::SlotPool;
use smithay_client_toolkit::{compositor::CompositorHandler, shell::wlr_layer::LayerSurface};
use wayland_client::protocol::wl_pointer;
struct Bar {
    shm: Shm,
    exit: bool,
    width: u32,
    heiht: u32,
    layer: LayerSurface,
    pointer: Option<wl_pointer::WlPointer>,
    buffer: SlotPool,
    _opacity: f32,
}

impl CompositorHandler for Bar {
    fn scale_factor_changed(
        &mut self,
        conn: &wayland_client::Connection,
        qh: &wayland_client::QueueHandle<Self>,
        surface: &wayland_client::protocol::wl_surface::WlSurface,
        new_factor: i32,
    ) {
    }
    fn transform_changed(
        &mut self,
        conn: &wayland_client::Connection,
        qh: &wayland_client::QueueHandle<Self>,
        surface: &wayland_client::protocol::wl_surface::WlSurface,
        new_transform: wayland_client::protocol::wl_output::Transform,
    ) {
    }
    fn frame(
        &mut self,
        conn: &wayland_client::Connection,
        qh: &wayland_client::QueueHandle<Self>,
        surface: &wayland_client::protocol::wl_surface::WlSurface,
        time: u32,
    ) {
        draw()
    }
    fn surface_enter(
        &mut self,
        conn: &wayland_client::Connection,
        qh: &wayland_client::QueueHandle<Self>,
        surface: &wayland_client::protocol::wl_surface::WlSurface,
        output: &wayland_client::protocol::wl_output::WlOutput,
    ) {
    }
    fn surface_leave(
        &mut self,
        conn: &wayland_client::Connection,
        qh: &wayland_client::QueueHandle<Self>,
        surface: &wayland_client::protocol::wl_surface::WlSurface,
        output: &wayland_client::protocol::wl_output::WlOutput,
    ) {
    }
}
