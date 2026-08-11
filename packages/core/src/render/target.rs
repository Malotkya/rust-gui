use ash::{khr, vk};
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};
use std::{
    sync::Arc,
    rc::Rc, 
    ops::Deref
};
use crate::{
    render::{
        ctx::RenderContext,
        err::{
            ContextError,
            RenderError
        },
        pipeline::Pipeline,
        swapchain::Swapchain,
        vertex::{GpuBatchData, VertexData, Size}
    }
};

#[derive(Debug)]
pub enum RenderTargetError {
    FailedToAquireNextImage,
    WaitForFencesFailed,
    ResetFencesFailed,
    MissingMemoryType,
    FailedToInitBuffer,
    FailedToInitBufferMemory,
    FailedToBindBufferToMemory,
    FailedToWriteToDeviceMemory,
    ResetCommandBufferFailed,
    BeginCommandBufferFailed,
    EndCommandBufferFailed,
    SubmitToQueueFailed,
    SwapchainNeedsRecreation,
    GraphicPresentationFailed
}

#[derive(Clone)]
pub(crate) struct DeviceContext {
    pub(crate) inner: Rc<RenderContext>,
    pub(crate) device: Rc<ash::Device>
}

pub struct RenderTarget {
     _shader_compiler: shaderc::Compiler,
    current_frame: usize,
    max_frames_in_flight: usize,
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    command_buffers: Vec<vk::CommandBuffer>,
    command_pool: vk::CommandPool,
    pipelines: Vec<Pipeline>,
    framebuffers: Vec<vk::Framebuffer>,
    render_pass: vk::RenderPass,
    swapchain: Swapchain,
    surface: vk::SurfaceKHR,
    queue: vk::Queue,
    device: Rc<ash::Device>,
    ctx: Rc<RenderContext>,
    buffer_data: GpuBatchData
}

impl RenderTarget {
    /// Draw a frame given a list of vertices.
    pub fn draw(&mut self, clear_color:[f32; 4], vertices: &[VertexData]) -> Result<(), RenderError> {
        // Acquire next image
        let timeout = u64::MAX;
        let image_index = unsafe {
            match self.swapchain.loader.acquire_next_image(
                *self.swapchain,
                timeout,
                self.image_available_semaphores[self.current_frame],
                vk::Fence::null(),
            ) {
                Ok((idx, _)) => idx,
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                    // swapchain out of date, recreate (not implemented in full here)
                    return Ok(());
                }
                Err(_) => return Err(RenderTargetError::FailedToAquireNextImage.into()),
            }
        };

        // Ensure previous frame finished
        unsafe {
            self.device
                .wait_for_fences(&[self.in_flight_fences[self.current_frame]], true, u64::MAX)
                .map_err(|_|RenderTargetError::WaitForFencesFailed)?;
            self.device
                .reset_fences(&[self.in_flight_fences[self.current_frame]])
                .map_err(|_|RenderTargetError::ResetFencesFailed)?;
        }

        self.buffer_data.update(vertices)?;

        // Record command buffer for this frame into the per-frame command buffer
        let cmd_buf = self.command_buffers[self.current_frame];
        unsafe {
            self.device
                .reset_command_buffer(cmd_buf, vk::CommandBufferResetFlags::empty())
                    .map_err(|_|RenderTargetError::ResetCommandBufferFailed)?;
            let begin_info = vk::CommandBufferBeginInfo::default();
            self.device.begin_command_buffer(cmd_buf, &begin_info)
                .map_err(|_|RenderTargetError::BeginCommandBufferFailed)?;

            let clear_values = [vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: clear_color
                },
            }];

            let render_pass_info = vk::RenderPassBeginInfo::default()
                .render_pass(self.render_pass)
                .framebuffer(self.framebuffers[image_index as usize])
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: self.swapchain.extent,
                })
                .clear_values(&clear_values);

            self.device.cmd_begin_render_pass(
                cmd_buf,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );

            self.record_commands(self.current_frame);

            self.device.cmd_end_render_pass(cmd_buf);
            self.device.end_command_buffer(cmd_buf)
                .map_err(|_|RenderTargetError::EndCommandBufferFailed)?;
        }

        // Submit
        let wait_semaphores = [self.image_available_semaphores[self.current_frame]];
        let signal_semaphores = [self.render_finished_semaphores[self.current_frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [cmd_buf];

        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        unsafe {
            self.device.queue_submit(
                self.queue,
                &[submit_info],
                self.in_flight_fences[self.current_frame],
            )
        }.map_err(|_|RenderTargetError::SubmitToQueueFailed)?;

        // Present
        let swapchains = [*self.swapchain];
        let image_indices = [image_index];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        let result = unsafe { self.swapchain.loader.queue_present(self.queue, &present_info) };
        match result {
            Ok(_) => {}
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) | Err(vk::Result::SUBOPTIMAL_KHR) => return Err(
                RenderTargetError::SwapchainNeedsRecreation.into()
            ),
            Err(_) => return Err(RenderTargetError::GraphicPresentationFailed.into()),
        }

        // Advance frame
        self.current_frame = (self.current_frame + 1) % self.max_frames_in_flight;
        Ok(())
    }

    unsafe fn record_commands(&self, frame:usize) {
        let buffers = self.buffer_data.bind_buffers();
        let offsets = [0, 0];

        self.device.cmd_bind_vertex_buffers(
            self.command_buffers[frame],
            0, // Start binding at slot 0
            &buffers,
            &offsets,
        );

        if self.buffer_data.empty() {
            return;
        }
        let mut active_pipeline = vk::Pipeline::null();
        let mut instance_offset = 0;

        for cmd in &self.buffer_data.commands {
            // Only swap pipelines if the next shape requires a different topology
            let required_pipeline = self.pipelines[cmd.topology.as_raw() as usize].deref().clone();
            
            if required_pipeline != active_pipeline {
                self.device.cmd_bind_pipeline(
                    self.command_buffers[frame],
                    vk::PipelineBindPoint::GRAPHICS,
                    required_pipeline,
                );

                active_pipeline = required_pipeline;
            }

            // 3. Issue the draw call
            self.device.cmd_draw(
                self.command_buffers[frame],
                cmd.vertex_count, // Number of positions in this specific shape
                1,                // Instance count (1 instance per shape)
                cmd.first_vertex, // Where this shape starts in the flat position buffer
                instance_offset,  // Instanced offset pointing to this shape's color index
            );

            instance_offset +=1;
        }
    }

    pub fn size(&self) -> Size {
        self.swapchain.extent.into()
    }
}

impl Drop for RenderTarget {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.wait_for_fences(&self.in_flight_fences, true, 5_000_000_000);
            let _ = self.device.device_wait_idle();
            
            drop(std::ptr::read(&self.buffer_data));

            for &f in &self.in_flight_fences {
                self.device.destroy_fence(f, None);
            }
            for &s in &self.image_available_semaphores {
                self.device.destroy_semaphore(s, None);
            }
            for &s in &self.render_finished_semaphores {
                self.device.destroy_semaphore(s, None);
            }

            self.device.free_command_buffers(self.command_pool, &self.command_buffers);
            self.device.destroy_command_pool(self.command_pool, None);

            for &fb in &self.framebuffers {
                self.device.destroy_framebuffer(fb, None);
            }
            
            while let Some(pipeline) = self.pipelines.pop() {
                drop(pipeline);
            }

            self.device.destroy_render_pass(self.render_pass, None);
            
           drop(std::ptr::read(&self.swapchain));

            self.ctx.surface_loader.destroy_surface(self.surface, None);
            self.device.destroy_device(None);

        }
    }
}

impl RenderContext {
    pub fn create_target(self: &Rc<Self>, window:&Arc<Window>) -> Result<RenderTarget, RenderError> {
        unsafe {
            let display_handle = window.display_handle()
                .map_err(|e|ContextError::DisplayHandleError(e))?
                .as_raw();

            let window_handle = window.window_handle()
                .map_err(|e|ContextError::WindowHandleError(e))?
                .as_raw();

            let surface = ash_window::create_surface(&self.entry, &self.instance, display_handle, window_handle, None)
                .map_err(|_|ContextError::InitSurfaceFailed)?;

            let queue_priorities = [1.0f32];
            let queue_info = [vk::DeviceQueueCreateInfo::default()
                .queue_family_index(self.queue_family_index)
                .queue_priorities(&queue_priorities)
            ];

            let device_exts = [khr::swapchain::NAME.as_ptr()];
            let device_create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_info)
                .enabled_extension_names(&device_exts);

            let device = Rc::new(
                self.instance.create_device(self.physical_device, &device_create_info, None)
                    .map_err(|_|ContextError::InitLogicDeviceFailed)?
            );
            let queue = device.get_device_queue(self.queue_family_index, 0);      

            let swapchain = Swapchain::new(
                DeviceContext{
                    inner: self.clone(),
                    device: device.clone()
                },
                &surface,
                window.inner_size()
            )?;


            // Render pass
            let color_attachment = vk::AttachmentDescription::default()
                .format(swapchain.image_format)
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

            let color_attachment_ref = vk::AttachmentReference {
                attachment: 0,
                layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            };

            let subpass = vk::SubpassDescription::default()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .color_attachments(std::slice::from_ref(&color_attachment_ref));

            let dependency = vk::SubpassDependency::default()
                .src_subpass(vk::SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags::empty())
                .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

            let attachments = [color_attachment];
            let subpasses = [subpass];
            let dependencies = [dependency];
            let rp_info = vk::RenderPassCreateInfo::default()
                .attachments(&attachments)
                .subpasses(&subpasses)
                .dependencies(&dependencies);

            let render_pass = device.create_render_pass(&rp_info, None)
                .map_err(|_|ContextError::InitRenderPassFailed)?;


            let shader_compiler = shaderc::Compiler::new()
                .map_err(|_|ContextError::InitShaderCompilerFailed)?;

            let pipelines = Pipeline::new_group(&device, render_pass, swapchain.extent, &shader_compiler)?;

            let vk::Extent2D{width, height} = swapchain.extent;

            let mut framebuffers = Vec::with_capacity(swapchain.image_views.len());
            for (i, view) in swapchain.image_views.iter().enumerate() {
                let attachments = [*view];
                let fb_info = vk::FramebufferCreateInfo::default()
                    .render_pass(render_pass)
                    .attachments(&attachments)
                    .width(width)
                    .height(height)
                    .layers(1);

                framebuffers.push(
                    device.create_framebuffer(&fb_info, None)
                        .map_err(|_|ContextError::InitFrameBufferFailed(i))?
                );
            }

            let max_frames_in_flight = 2usize;
            let command_pool = device.create_command_pool(
                &vk::CommandPoolCreateInfo::default().queue_family_index(self.queue_family_index),
                None
            ).map_err(|_|ContextError::InitCommandPoolFailed)?;

            let alloc_info = vk::CommandBufferAllocateInfo::default()
                    .command_pool(command_pool)
                    .level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(max_frames_in_flight as u32);

            let command_buffers = device.allocate_command_buffers(&alloc_info)
                .map_err(|_|ContextError::InitCommandBufferFailed)?;

            // sync objects
            let (image_available_semaphores, render_finished_semaphores, in_flight_fences) =
                create_sync_objects(&device, max_frames_in_flight)?;

            Ok(RenderTarget{
                buffer_data: GpuBatchData::new(DeviceContext {
                    inner: self.clone(),
                    device: device.clone()
                }),
                image_available_semaphores,
                render_finished_semaphores,
                in_flight_fences,
                _shader_compiler: shader_compiler,
                current_frame: 0,
                max_frames_in_flight,
                command_buffers,
                command_pool,
                pipelines,
                framebuffers,
                render_pass,
                swapchain,
                surface,
                queue,
                device,
                ctx: self.clone()
            })
        }
    }
}

unsafe fn create_sync_objects(device: &ash::Device, max_frames_in_flight: usize) -> Result<(Vec<vk::Semaphore>, Vec<vk::Semaphore>, Vec<vk::Fence>), RenderError> {
    let mut image_available_semaphores = Vec::with_capacity(max_frames_in_flight);
    let mut render_finished_semaphores = Vec::with_capacity(max_frames_in_flight);
    let mut in_flight_fences = Vec::with_capacity(max_frames_in_flight);

    let semaphore_info = vk::SemaphoreCreateInfo::default();
    let fence_info = vk::FenceCreateInfo::default()
        .flags(vk::FenceCreateFlags::SIGNALED);

    for _ in 0..max_frames_in_flight {
        let image_available = device.create_semaphore(&semaphore_info, None)
            .map_err(|_|ContextError::InitSemaphoreFailed)?;
        let render_finished = device.create_semaphore(&semaphore_info, None)
            .map_err(|_|ContextError::InitSemaphoreFailed)?;
        let fence = device.create_fence(&fence_info, None)
            .map_err(|_|ContextError::InitFenceFailed)?;

        image_available_semaphores.push(image_available);
        render_finished_semaphores.push(render_finished);
        in_flight_fences.push(fence);
    }

    Ok((image_available_semaphores, render_finished_semaphores, in_flight_fences))
}