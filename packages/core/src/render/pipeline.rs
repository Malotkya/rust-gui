use ash::vk;
use super::{
    Device,
    err::RenderError,
    shader::Shader,
    VertexData
};
use std::{
    fmt,
    ops::Deref
};

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum PipelineError {
    InitPiplineLayoutFailed,
    InitPipelineFailed(PipelineType),
    InitShaderCompilerFailed,
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitPiplineLayoutFailed => write!(f, "Failed to initalize PipelineLayout!"),
            Self::InitShaderCompilerFailed => write!(f, "Failed to initalize Shader Compiler!"),
            Self::InitPipelineFailed(topology) => write!(f, "Failed to initalize {} Pipeline!",
                match topology {
                    &PipelineType::POINT_LIST => "point list".to_string(),
                    &PipelineType::LINE_LIST => "line list".to_string(),
                    &PipelineType::LINE_STRIP => "line strip".to_string(),
                    &PipelineType::TRIANGLE_LIST => "triangle list".to_string(),
                    &PipelineType::TRIANGLE_STRIP => "traingle strip".to_string(),
                    &PipelineType::TRIANGLE_FAN => "traingle fan".to_string(),
                    &PipelineType::LINE_LIST_WITH_ADJACENCY => "line list with adjacency".to_string(),
                    &PipelineType::LINE_STRIP_WITH_ADJACENCY => "line strip with adjacency".to_string(),
                    &PipelineType::TRIANGLE_LIST_WITH_ADJACENCY => "triangle list with adjacency".to_string(),
                    &PipelineType::TRIANGLE_STRIP_WITH_ADJACENCY => "traingle strip with adjacency".to_string(),
                    &n => format!("unknown({})", n.as_raw())
                }
            )
        }
    }
}

pub type PipelineType = vk::PrimitiveTopology;

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Pipeline {
    layout: vk::PipelineLayout,
    inner: vk::Pipeline,
    device: Device,
    _type: PipelineType
}

impl Pipeline {
    pub fn new_group(device: &Device, render_pass: vk::RenderPass, extent: vk::Extent2D, compiler: &shaderc::Compiler) -> Result<Vec<Self>, RenderError> {
        let shader = Shader::new(device, compiler)?;

        let shader_stages = shader.stages();
        let vertex_bindings = VertexData::binding();
        let vertex_attributes = VertexData::attribute();
        
        let vertex_input_info: vk::PipelineVertexInputStateCreateInfo<'_> = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&vertex_bindings)
            .vertex_attribute_descriptions(&vertex_attributes);

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: extent.width as f32,
            height: extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent,
        };
        let viewport_state: vk::PipelineViewportStateCreateInfo<'_> = vk::PipelineViewportStateCreateInfo::default()
            .viewports(std::slice::from_ref(&viewport))
            .scissors(std::slice::from_ref(&scissor));

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .depth_bias_enable(false);

        let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false); //Posibly true?

        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .attachments(std::slice::from_ref(&color_blend_attachment));

        let mut pipelines:Vec<Self> = Vec::with_capacity(10);
        for i in 0..10 { //Skip: vk::PrimitiveTopology::PATCH_LIST = 10
            let topology = PipelineType::from_raw(i);

            let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default();
            let layout = unsafe { device.create_pipeline_layout(&pipeline_layout_info, None) }
                .map_err(|_|PipelineError::InitPiplineLayoutFailed)?;

            let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
                .primitive_restart_enable(false)
                .topology(topology);

            let pipeline_info: vk::GraphicsPipelineCreateInfo<'_> = vk::GraphicsPipelineCreateInfo::default()
                .stages(&shader_stages)
                .vertex_input_state(&vertex_input_info)
                .input_assembly_state(&input_assembly)
                .viewport_state(&viewport_state)
                .rasterization_state(&rasterizer)
                .multisample_state(&multisampling)
                .color_blend_state(&color_blending)
                .layout(layout)
                .render_pass(render_pass)
                .subpass(0);

            let pipeline = unsafe { device.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None) }
                .map_err(|_|PipelineError::InitPipelineFailed(topology))?;

            pipelines.push(Self {
                layout,
                device: device.clone(),
                inner: pipeline[0],
                _type: topology
            });

        }

        Ok(pipelines)
    }

    pub unsafe fn destory(&mut self) {
        if self.inner != vk::Pipeline::null() { 
            self.device.destroy_pipeline(self.inner, None);
            self.inner = vk::Pipeline::null();
        }

        if self.layout != vk::PipelineLayout::null() {
            self.device.destroy_pipeline_layout(self.layout, None);
            self.layout = vk::PipelineLayout::null()
        }
    }
}

impl Deref for Pipeline {
    type Target = vk::Pipeline;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
