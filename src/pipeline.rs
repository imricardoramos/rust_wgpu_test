use crate::vertex::Vertex;

pub fn create_pipeline(
    device: &wgpu::Device,
    vs_module: wgpu::ShaderModule,
    fs_module: wgpu::ShaderModule,
    render_pipeline_layout: wgpu::PipelineLayout,
    sc_desc: &wgpu::SwapChainDescriptor,
    ) -> wgpu::RenderPipeline {

    let pipeline_descriptor = &wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main", // 1.
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor { // 2.
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(
            wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }
        ),
        color_states: &[
            wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            },
        ],
        primitive_topology: wgpu::PrimitiveTopology::TriangleList, // 1.
        depth_stencil_state: None, // 2.
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint32, // 3.
            vertex_buffers: &[
                wgpu::VertexBufferDescriptor {
                    stride: 4*3, // 1.
                    step_mode: wgpu::InputStepMode::Vertex, // 2.
                    attributes: &[ // 3.
                        wgpu::VertexAttributeDescriptor {
                            offset: 0, // 4.
                            shader_location: 0, // 5.
                            format: wgpu::VertexFormat::Float3, // 6.
                        },
                    ]
                }
            ], // 4.
        },
        sample_count: 1, // 5.
        sample_mask: !0, // 6.
        alpha_to_coverage_enabled: false, // 7.
    };
    device.create_render_pipeline(pipeline_descriptor)
}
