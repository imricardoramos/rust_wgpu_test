use std::time::Instant;
use std::time::Duration;
use winit::window::Window;
use imgui::*;
use imgui_wgpu::{Renderer, RendererConfig};
use crate::engine::Engine;

pub struct Gui {
    pub imgui: imgui::Context,
    pub platform: imgui_winit_support::WinitPlatform,
    pub renderer: imgui_wgpu::Renderer,
    last_frame: Instant,
    pub demo_open: bool,
    pub last_cursor: Option<imgui::MouseCursor>,
}

impl Gui {
    pub fn init(window: &Window, engine: &Engine) -> Self{
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );
        imgui.set_ini_filename(None);
        let hidpi_factor = window.scale_factor();

        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let renderer_config = RendererConfig {
            texture_format: engine.sc_desc.format,
            ..Default::default()
        };

        let renderer = Renderer::new(&mut imgui, &engine.device, &engine.queue, renderer_config);

        let last_frame = Instant::now();
        let demo_open = true;
        Self {
            imgui: imgui,
            platform: platform,
            renderer: renderer,
            last_frame: last_frame,
            demo_open: demo_open,
            last_cursor: None,
        }
    }
    pub fn render(&mut self, window: &Window) -> Duration {
        let delta_s = self.last_frame.elapsed();
        let now = Instant::now();
        self.imgui.io_mut().update_delta_time(now - self.last_frame);
        self.last_frame = now;
        self.platform
            .prepare_frame(self.imgui.io_mut(), &window)
            .expect("Failed to prepare frame");
        delta_s
    }
}
