use futures::executor::block_on;
use winit::{
    window::Window,
};
use crate::{
    engine::Engine,
    model::Model,
    gui::Gui
};
use imgui::*;

pub struct Scene {
    engine: Engine,
    pub gui: Gui,
    model: Model,
}

impl Scene {
    pub fn new(window: &Window) -> Self {
        let engine = block_on(Engine::init(window));
        let gui = Gui::init(&window, &engine);
        let model = Model::new(&engine.device,"./models/icosahedron.obj");
        println!("{:#?}", model);
        Self {
            engine,
            gui,
            model
        }
    }
    pub fn render(&mut self, window: &Window) -> Result<(), wgpu::SwapChainError>{
        let frame = self.engine.swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;
        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor{
            label: Some("MyEncoder")
        };
        let mut command_encoder = self.engine.device.create_command_encoder(&command_encoder_descriptor);

        let render_pipeline = self.engine.create_render_pipeline(&[&self.model.bind_group_layout]);

        let delta_s = self.gui.render(&window);
        let ui = self.gui.imgui.frame();

        {
            let model = &mut self.model;
            let window = imgui::Window::new(im_str!("Scene Editor"));
            window
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(&ui, || {
                    imgui::Slider::<f32>::new(im_str!("asdf"))
                        .range(-1.0..=1.0)
                        .build_array(&ui, &mut model.position);
                });

            ui.show_demo_window(&mut self.gui.demo_open);
        }
        println!("{:?}", self.model);
        if self.gui.last_cursor != ui.mouse_cursor() {
            self.gui.last_cursor = ui.mouse_cursor();
            self.gui.platform.prepare_render(&ui, &window);
        }
        self.model.update(&self.engine.device, &mut command_encoder);
        {
            let render_pass_descriptor = &wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: true
                    },
                }],
                depth_stencil_attachment: None
            };
            let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
            render_pass.set_pipeline(&render_pipeline);
            self.model.render(&self.engine.device, &mut render_pass);
            self.gui.renderer
                .render(ui.render(), &self.engine.queue, &self.engine.device, &mut render_pass)
                .expect("Rendering failed");
        }
        self.engine.queue.submit(std::iter::once(command_encoder.finish()));
        Ok(())
    }
    pub fn redraw(&mut self, new_size: winit::dpi::PhysicalSize<u32>){
        self.engine.recreate_swap_chain(new_size)
    }
}
