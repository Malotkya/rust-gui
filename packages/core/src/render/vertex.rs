use ash::vk;
use crate::{
    data::{Color, Position},
    render::{
        err::{RenderTargetError, RenderError},
        ctx::DeviceContext
    }
};

pub struct VertexShape {
    pub color: Color,
    pub positions: Vec<Position>,
    pub topology: vk::PrimitiveTopology
}

impl VertexShape {
    pub(crate) fn binding() -> [vk::VertexInputBindingDescription; 2] {
        [
            Position::binding(),
            Color::binding()
        ]
    }

    pub(crate) fn attribute() -> [vk::VertexInputAttributeDescription; 2] {
        [
            Position::attribute(),
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

    pub fn update<'a, T: Sized>(&'a mut self, data:&[T]) -> Result<UnmapRef<'a>, RenderError> {
        let needed_size = std::mem::size_of_val(data) as u64;

        if needed_size > self.size {
            if self.buffer != vk::Buffer::null() { unsafe {
                self.ctx.device.destroy_buffer(self.buffer, None);
                self.ctx.device.free_memory(self.memory, None);
            } }

            self.resize(needed_size)?;
        }

        unsafe {
            self.ctx.device.map_memory(self.memory, 0, self.size, vk::MemoryMapFlags::default())
                .map_err(|_|RenderTargetError::FailedToWriteToDeviceMemory)?;
        }

        Ok(UnmapRef {
            device: &self.ctx.device,
            memory: &self.memory
        })
    }

    pub fn resize(&mut self, size:vk::DeviceSize) -> Result<(), RenderError>{
         if size <= self.size {
            return Ok(());
        }

        if self.buffer != vk::Buffer::null() { unsafe {
            self.ctx.device.destroy_buffer(self.buffer, None);
            self.ctx.device.free_memory(self.memory, None);
        } }
        
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
}

impl Drop for GpuData {
    fn drop(&mut self) {
        if self.buffer != vk::Buffer::null() { unsafe {
            self.ctx.device.destroy_buffer(self.buffer, None);
            self.ctx.device.free_memory(self.memory, None);
        } }
    }
}

pub struct GpuBatchData {
    positions:GpuData,
    colors:GpuData,
    pub commands: Vec<DrawCommand>
}

impl GpuBatchData {
    pub fn new(ctx:DeviceContext) -> Self {
        Self {
            positions: GpuData::new(ctx.clone()),
            colors: GpuData::new(ctx.clone()),
            commands: Vec::new()
        }
    }

    pub fn empty(&self) -> bool {
        self.positions.empty() && self.colors.empty()
    }

    pub fn update(&mut self, shapes: &[VertexShape]) -> Result<(), RenderError>{
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

        let _ = self.positions.update(&positions)?;
        let _ = self.colors.update( &colors)?;
        self.commands = commands;

        Ok(())
    }

    pub fn bind_buffers(&self) -> [vk::Buffer; 2] {
        [
            self.positions.buffer,
            self.colors.buffer
        ]
    }
}

pub struct DrawCommand {
    pub vertex_count: u32,
    pub first_vertex: u32,
    pub color_index: u32,
    pub topology: vk::PrimitiveTopology
}

pub struct UnmapRef<'a> {
    device:&'a ash::Device,
    memory: &'a vk::DeviceMemory,
}

impl<'a> UnmapRef<'a> {
    pub fn unmap(&self) {
        unsafe {
            self.device.unmap_memory(*self.memory);
        }
    }
}

impl<'a> Drop for UnmapRef<'a> {
    fn drop(&mut self) {
        self.unmap()
    }
}