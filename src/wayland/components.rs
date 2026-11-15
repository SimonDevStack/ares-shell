use crate::render::draw::{self, Draw, Render, RenderData};
use smithay_client_toolkit::shell::wlr_layer::LayerShellHandler;
use smithay_client_toolkit::shm::Shm;
use smithay_client_toolkit::shm::slot::SlotPool;
use smithay_client_toolkit::{compositor::CompositorHandler, shell::wlr_layer::LayerSurface};
use wayland_client::protocol::wl_pointer;

struct Bar {
    shm: Shm,
    first_configure: bool,
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
impl Render for Bar {
    fn settings(&self) -> RenderData {
        RenderData {
            num: 1,
            width: self.width,
            height: self.heiht,
            surface: self.layer,
        }
    }

}
    impl LayerShellHandler for Bar {
        fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _layer: &LayerSurface) {
            self.exit = true;
        }

        fn configure(
            &mut self,
            _conn: &Connection,
            qh: &QueueHandle<Self>,
            _layer: &LayerSurface,
            configure: LayerSurfaceConfigure,
            _serial: u32,
        ) {
            self.width = NonZeroU32::new(configure.new_size.0).map_or(256, NonZeroU32::get);
            self.height = NonZeroU32::new(configure.new_size.1).map_or(256, NonZeroU32::get);

            if self.first_configure {
                self.first_configure = false;
                self.draw(qh);
            }
        }
    }
    delegate_registry!(Bar);

    impl ProvidesRegistryState for Bar {
        fn registry(&mut self) -> &mut RegistryState {
            &mut self.registry_state
        }
        registry_handlers![OutputState, SeatState];
    }

    smithay_client_toolkit::delegate_dispatch2!(Bar);
}
