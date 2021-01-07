use tobj;
use wgpu::util::DeviceExt;

#[derive(Debug)]
struct Mesh {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}
#[derive(Debug)]
pub struct Model {
    pub position: [f32; 3],
    mesh: Mesh,
    position_buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}
impl Model {
    pub fn new(device: &wgpu::Device, obj_path: &str) -> Model{
        let (models, _materials) = tobj::load_obj(obj_path, true).expect("Failed to load file");
        let position = [0.0, 0.0, 0.0];

        let position_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&position),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        );
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("PositionBindGroupLayout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None
                }
            ]
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("PositionBindGroup"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(position_buffer.slice(..))
                }
            ]
        });
        Model {
            position: position,
            mesh: Mesh {
                vertices: models[0].clone().mesh.positions,
                indices: models[0].clone().mesh.indices,
                vertex_buffer: device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(&models[0].mesh.positions),
                        usage: wgpu::BufferUsage::VERTEX,
                    }
                ),
                index_buffer: device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(&models[0].mesh.indices),
                        usage: wgpu::BufferUsage::INDEX,
                    }
                ),
            },
            position_buffer: position_buffer,
            bind_group_layout: bind_group_layout,
            bind_group: bind_group,
        }
    }
    pub fn update(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder){
        let staging_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&self.position),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_SRC,
            }
        );
        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.position_buffer,
            0,
            3*4,
        )
    }
    pub fn render<'a>(&'a self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh.index_buffer.slice(..));
        render_pass.draw_indexed(0..self.mesh.indices.len() as u32, 0, 0..1);
    }
}
