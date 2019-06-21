use std::path::{Path, PathBuf};

use failure::Error;
use gfx_hal::{
    command::ClearValue,
    format::Format,
    pso::{DepthStencilDesc, Element, ElemStride, VertexInputRate},
};
use lazy_static;
use rendy::{
    command::{QueueId, RenderPassEncoder},
    factory::{Config, Factory},
    graph::{
        render::{PrepareResult, RenderGroupBuilder, SimpleGraphicsPipeline, SimpleGraphicsPipelineDesc},
        GraphBuilder, GraphContext, NodeBuffer, NodeImage,
    },
    memory::{Dynamic, MemoryUsageValue},
    mesh::{AsVertex, Color, PosColor, Position},
    resource::{Buffer, BufferInfo, DescriptorSetLayout, Escape, Handle},
    shader::{FileShaderInfo, ShaderKind, ShaderSet, ShaderSetBuilder, SourceLanguage},
    wsi::winit::{Event, EventsLoop, WindowBuilder, WindowEvent},
};

#[cfg(feature = "dx12")]
type Backend = rendy::dx12::Backend;
#[cfg(feature = "metal")]
type Backend = rendy::metal::Backend;
#[cfg(feature = "vulkan")]
type Backend = rendy::vulkan::Backend;

const VERTEX_DATA: [PosColor; 3] = [
    PosColor {
        position: Position([-0.5, 0.5, 0.0]),
        color: Color([1.0, 0.0, 0.0, 1.0]),
    },
    PosColor {
        position: Position([0.0, -0.5, 0.0]),
        color: Color([0.0, 1.0, 0.0, 1.0]),
    },
    PosColor {
        position: Position([0.5, 0.5, 0.0]),
        color: Color([0.0, 0.0, 1.0, 1.0]),
    },
];

lazy_static::lazy_static! {
    static ref VERTEX_SHADER_PATH: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "examples",
        "01-up-and-running",
        "shaders",
        "vert.glsl",
    ].iter().collect::<PathBuf>();

    static ref FRAGMENT_SHADER_PATH: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "examples",
        "01-up-and-running",
        "shaders",
        "frag.glsl",
    ].iter().collect::<PathBuf>();
}

#[derive(Debug, Default)]
struct TriangleRenderPipelineDesc;

impl<B, T> SimpleGraphicsPipelineDesc<B, T> for TriangleRenderPipelineDesc
where
    B: gfx_hal::Backend,
    T: ?Sized,
{
    type Pipeline = TriangleRenderPipeline<B>;

    fn load_shader_set(&self, factory: &mut Factory<B>, _aux: &T) -> ShaderSet<B> {
        ShaderSetBuilder::default()
            .with_vertex(&FileShaderInfo::new(
                VERTEX_SHADER_PATH.as_path(),
                ShaderKind::Vertex,
                SourceLanguage::GLSL,
                "main",
            ))
            .expect("Vertex shader compilation failed.")
            .with_fragment(&FileShaderInfo::new(
                FRAGMENT_SHADER_PATH.as_path(),
                ShaderKind::Fragment,
                SourceLanguage::GLSL,
                "main",
            ))
            .expect("Fragment shader compilation failed.")
            .build(factory, Default::default())
            .expect("Shader set creation failed.")
    }

    fn depth_stencil(&self) -> Option<DepthStencilDesc> { None }

    fn vertices(&self) -> Vec<(Vec<Element<Format>>, ElemStride, VertexInputRate)> {
        vec![PosColor::vertex().gfx_vertex_input_desc(VertexInputRate::Vertex)]
    }

    fn build(
        self,
        _ctx: &GraphContext<B>,
        _factory: &mut Factory<B>,
        _queue: QueueId,
        _aux: &T,
        buffers: Vec<NodeBuffer>,
        images: Vec<NodeImage>,
        set_layouts: &[Handle<DescriptorSetLayout<B>>],
    ) -> Result<Self::Pipeline, Error> {
        assert!(buffers.is_empty());
        assert!(images.is_empty());
        assert!(set_layouts.is_empty());

        Ok(TriangleRenderPipeline { vertex_buffer: None })
    }
}

#[derive(Debug)]
struct TriangleRenderPipeline<B> where B: gfx_hal::Backend {
    vertex_buffer: Option<Escape<Buffer<B>>>,
}

impl<B, T> SimpleGraphicsPipeline<B, T> for TriangleRenderPipeline<B>
where
    B: gfx_hal::Backend,
    T: ?Sized,
{
    type Desc = TriangleRenderPipelineDesc;

    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        _set_layouts: &[Handle<DescriptorSetLayout<B>>],
        _index: usize,
        _aux: &T,
    ) -> PrepareResult {
        if self.vertex_buffer.is_none() {
            println!("Creating vertex buffer");

            let buf_info = BufferInfo {
                size: PosColor::vertex().stride as u64 * VERTEX_DATA.len() as u64,
                usage: gfx_hal::buffer::Usage::VERTEX,
            };

            println!("{:?}", buf_info);

            let mut vertex_buffer = factory
                .create_buffer(
                    buf_info,
                    Dynamic,
                )
                .expect("Vertex buffer creation failed.");

            println!("Uploading vertex buffer");
            unsafe {
                factory
                    .upload_visible_buffer(&mut vertex_buffer, 0, &VERTEX_DATA)
                    .expect("Vertex data upload failed.");
            }

            self.vertex_buffer = Some(vertex_buffer);
        }

        PrepareResult::DrawReuse
    }

    fn draw(
        &mut self,
        _layout: &<B as gfx_hal::Backend>::PipelineLayout,
        mut encoder: RenderPassEncoder<B>,
        _index: usize,
        _aux: &T,
    ) {
        let vb = self.vertex_buffer.as_ref().unwrap();
        unsafe {
            encoder.bind_vertex_buffers(0, Some((vb.raw(), 0)));
            encoder.draw(0..3, 0..1);
        }
    }

    fn dispose(self, factory: &mut Factory<B>, aux: &T) {}
}

#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
fn main() {
    env_logger::init();

    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello, Rendy!")
        .with_dimensions((800, 600).into())
        .build(&events_loop)
        .expect("Window creation failed.");


    let config: Config = Default::default();
    let (mut factory, mut families): (Factory<Backend>, _) =
        rendy::factory::init(config).expect("Factory creation failed.");

    let surface = factory.create_surface(&window);

    let mut graph_builder = GraphBuilder::<Backend, ()>::new();

    graph_builder.add_node(
        TriangleRenderPipeline::builder()
            .into_subpass()
            .with_color_surface()
            .into_pass()
            .with_surface(
                surface,
                Some(gfx_hal::command::ClearValue::Color([0.0, 0.0, 0.0, 1.0].into()))
            ),
    );

    let mut graph = graph_builder
        .build(&mut factory, &mut families, &())
        .unwrap();

    let mut close = false;
    while !close {
        factory.maintain(&mut families);
        graph.run(&mut factory, &mut families, &());

        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event: w, .. } => match w {
                WindowEvent::CloseRequested => close = true,
                _ => (),
            },
            _ => (),
        });
    }

    graph.dispose(&mut factory, &());
}

#[cfg(not(any(feature = "dx12", feature = "metal", feature = "vulkan")))]
fn main() {
    println!("Please enable one of the backend features: dx12, metal, vulkan");
}
