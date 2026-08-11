use ash::vk;
use crate::{
    data::{Color, VertexPosition},
    render::{
        err::{RenderTargetError, RenderError},
        ctx::DeviceContext
    }
};

pub use vk::PrimitiveTopology as Topology;

pub struct Size {
    pub width: f32,
    pub height: f32
}

impl From<vk::Extent2D> for Size {
    fn from(value: vk::Extent2D) -> Self {
        Self { 
            width: value.width as f32,
            height: value.height as f32
        }
    }
}

pub trait VertexShape {
    fn color(&self) -> Color;
    fn positions(&self, extent:&Size) -> Vec<VertexPosition>;
    fn topology(&self) -> Topology;
}

pub struct VertexData {
    pub color: Color,
    pub positions: Vec<VertexPosition>,
    pub topology: vk::PrimitiveTopology
}

impl VertexData {
    pub(crate) fn binding() -> [vk::VertexInputBindingDescription; 2] {
        [
            VertexPosition::binding(),
            Color::binding()
        ]
    }

    pub(crate) fn attribute() -> [vk::VertexInputAttributeDescription; 2] {
        [
            VertexPosition::attribute(),
            Color::attribute()
        ]
    }
}

pub struct GpuData {
    size: u64,
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    ctx: DeviceContext
}

impl GpuData {
    pub fn new(ctx:DeviceContext) -> Self {
        Self {
            size: 0,
            buffer: vk::Buffer::null(),
            memory: vk::DeviceMemory::null(),
            ctx
        }
    }

    pub fn update<'a, T: Sized>(&'a mut self, data:&[T]) -> Result<(), RenderError> {
        let needed_size = std::mem::size_of_val(data) as u64;

        if needed_size > self.size {
            self.resize(needed_size)?;
        }

        unsafe {
            let ptr = self.ctx.device.map_memory(self.memory, 0, self.size, vk::MemoryMapFlags::default())
                .map_err(|_|RenderTargetError::FailedToWriteToDeviceMemory)? as *mut T;
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                ptr,
                data.len()
            );

            self.ctx.device.unmap_memory(self.memory);
        }

        Ok(())
    }

    pub fn resize(&mut self, size:vk::DeviceSize) -> Result<(), RenderError>{
         if size <= self.size {
            return Ok(());
        }

        unsafe { self.destory() };
        
        let usage = vk::BufferUsageFlags::VERTEX_BUFFER;
        let properties = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;

        let buffer_create_info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

        self.buffer = unsafe { self.ctx.device.create_buffer(&buffer_create_info, None) }
            .map_err(|_|RenderTargetError::FailedToInitBuffer)?;

        let mem_requirements = unsafe { self.ctx.device.get_buffer_memory_requirements(self.buffer) };

        let mem_type = self.ctx.inner.find_memory_type(mem_requirements.memory_type_bits, properties)
            .ok_or_else(||RenderTargetError::MissingMemoryType)?;

        let alloc_info = vk::MemoryAllocateInfo::default()
            .allocation_size(mem_requirements.size)
            .memory_type_index(mem_type);

        self.memory = unsafe {
            self.ctx.device.allocate_memory(&alloc_info, None)
        }.map_err(|_|RenderTargetError::FailedToInitBufferMemory)?;

        unsafe {
            self.ctx.device.bind_buffer_memory(self.buffer, self.memory, 0)
        }.map_err(|_|RenderTargetError::FailedToBindBufferToMemory)?;

        self.size = size;
        Ok(())
    }

    fn empty(&self) -> bool {
        self.size == 0
    }

    pub unsafe fn destory(&mut self) {
        if self.buffer != vk::Buffer::null() {
            self.ctx.device.destroy_buffer(self.buffer, None);
            self.buffer = vk::Buffer::null();
        }

        if self.memory != vk::DeviceMemory::null() {
            self.ctx.device.free_memory(self.memory, None);
            self.memory = vk::DeviceMemory::null();
        }
    } 
}

pub struct GpuBatchData {
    positions:GpuData,
    colors:GpuData,
    pub commands: Vec<DrawCommand>,
    ctx: DeviceContext
}

impl GpuBatchData {
    pub fn new(ctx:DeviceContext) -> Self {
        Self {
            positions: GpuData::new(ctx.clone()),
            colors: GpuData::new(ctx.clone()),
            commands: Vec::new(),
            ctx: ctx.clone()
        }
    }

    pub fn empty(&self) -> bool {
        self.positions.empty() && self.colors.empty()
    }

    pub fn update(&mut self, shapes: &[VertexData]) -> Result<(), RenderError>{
        let size = shapes.len();
        let mut positions = Vec::with_capacity(size);
        let mut colors = Vec::with_capacity(size);
        let mut commands = Vec::with_capacity(size);

        let mut offset = 0;
        for (color_idx, shape) in shapes.iter().enumerate() {
            let vertex_count = shape.positions.len() as u32;

            positions.extend_from_slice(&shape.positions);
            colors.push(shape.color);

            commands.push(DrawCommand{
                vertex_count,
                first_vertex: offset,
                color_index: color_idx as u32,
                topology: shape.topology,
            });

            offset += vertex_count;
        }

        self.positions.update(&positions)?;
        self.colors.update( &colors)?;
        self.commands = commands;

        Ok(())
    }

    pub fn bind_buffers(&self) -> [vk::Buffer; 2] {
        [
            self.positions.buffer,
            self.colors.buffer
        ]
    }

    pub unsafe fn destory(&mut self) {
        self.positions.destory();
        self.colors.destory();
    }
}

pub struct DrawCommand {
    pub vertex_count: u32,
    pub first_vertex: u32,
    pub color_index: u32,
    pub topology: vk::PrimitiveTopology
}