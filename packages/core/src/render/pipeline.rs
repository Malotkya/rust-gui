use ash::vk;
use super::{
    err::RenderError,
    shader::Shader,
    VertexData
};
use std::{
    ops::Deref,
    rc::Rc
};

#[derive(Debug)]
pub enum PipelineError {
    InitPiplineLayoutFailed,
    InitPipelineFailed(PipelineType),
    InitShaderCompilerFailed,
}

pub type PipelineType = vk::PrimitiveTopology;

#[derive(Clone)]
pub struct Pipeline {
    layout: vk::PipelineLayout,
    inner: vk::Pipeline,
    device: Rc<ash::Device>,
    _type: PipelineType
}

impl Pipeline {
    pub fn new_group(device: &Rc<ash::Device>, render_pass: vk::RenderPass, extent: vk::Extent2D, compiler: &shaderc::Compiler) -> Result<Vec<Self>, RenderError> {
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
