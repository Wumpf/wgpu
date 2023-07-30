//! TODO: Describe

use std::{borrow::Cow, iter};

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    pos: [f32; 3],
    color: [f32; 4],
}

struct Example {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    depth_view: wgpu::TextureView,
}

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;

fn create_depth_view(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
    device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&wgpu::TextureViewDescriptor::default())
}

impl wgpu_example::framework::Example for Example {
    fn init(
        config: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(config.format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Greater,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // A bunch of quads, each drawn as triangle strip.
        let mut vertex_data = Vec::new();
        fn add_quad(
            vertices: &mut Vec<Vertex>,
            color: [f32; 4],
            depth: f32,
            top_left: [f32; 2],
            extent: [f32; 2],
        ) {
            vertices.extend_from_slice(&[
                Vertex {
                    pos: [top_left[0], top_left[1], depth],
                    color,
                },
                Vertex {
                    pos: [top_left[0] + extent[0], top_left[1], depth],
                    color,
                },
                Vertex {
                    pos: [top_left[0], top_left[1] - extent[1], depth],
                    color,
                },
                Vertex {
                    pos: [top_left[0] + extent[0], top_left[1] - extent[1], depth],
                    color,
                },
            ])
        }
        // Top left quarter in red.
        add_quad(
            &mut vertex_data,
            [1.0, 0.0, 0.0, 1.0],
            0.8,
            [-1.0, 1.0],
            [1.0, 1.0],
        );
        // Same again in pink, but behind, so completely hidden!
        add_quad(
            &mut vertex_data,
            [1.0, 0.0, 1.0, 1.0],
            0.5,
            [-1.0, 1.0],
            [1.0, 1.0],
        );
        // Entire screen in yellow, again behind the red one
        add_quad(
            &mut vertex_data,
            [0.0, 1.0, 0.0, 1.0],
            0.5,
            [-1.0, 1.0],
            [2.0, 2.0],
        );

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Example {
            pipeline,
            vertex_buffer,
            depth_view: create_depth_view(device, config.width, config.height),
        }
    }

    fn update(&mut self, _event: winit::event::WindowEvent) {}

    fn render(
        &mut self,
        view: &wgpu::TextureView,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _spawner: &wgpu_example::framework::Spawner,
    ) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations::default()),
                    stencil_ops: None,
                }),
            });

            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rpass.set_pipeline(&self.pipeline);

            rpass.draw(0..4, 0..1);
            rpass.draw(4..8, 0..1);
            rpass.draw(8..12, 0..1);
        }

        queue.submit(iter::once(encoder.finish()));
    }

    fn resize(
        &mut self,
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
        self.depth_view = create_depth_view(device, config.width, config.height);
    }
}

fn main() {
    wgpu_example::framework::run::<Example>("occlusion-query");
}

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[wasm_bindgen_test::wasm_bindgen_test]
fn occlusion_query() {
    // TODO:
    wgpu_example::framework::test::<Example>(wgpu_example::framework::FrameworkRefTest {
        image_path: "/examples/msaa-line/screenshot.png",
        width: 1024,
        height: 768,
        optional_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
        base_test_parameters: wgpu_test::TestParameters::default()
            // AMD seems to render nothing on DX12 https://github.com/gfx-rs/wgpu/issues/3838
            .specific_failure(Some(wgpu::Backends::DX12), Some(0x1002), None, false),
        // There's a lot of natural variance so we check the weighted median too to differentiate
        // real failures from variance.
        comparisons: &[
            wgpu_test::ComparisonType::Mean(0.065),
            wgpu_test::ComparisonType::Percentile {
                percentile: 0.5,
                threshold: 0.29,
            },
        ],
    });
}
