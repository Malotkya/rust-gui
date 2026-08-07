use ash::vk;
use shaderc::{
    Error as ShadercError,
    ShaderKind
};

#[derive(Debug)]
pub enum ShaderError {
    ShadercError(ShadercError),
    CompileFailed
}

const VERTEX_SRC:&'static str = r#"
    #version 450
    layout(location = 0) in vec2 in_pos;
    layout(location = 1) in vec4 in_color;
    layout(location = 0) out vec4 frag_color;
    void main() {
        gl_Position = vec4(in_pos, 0.0, 1.0);
        frag_color = in_color;
    }
"#;

const FRAGMENT_SRC:&'static str = r#"
    #version 450
    layout(location = 0) in vec4 frag_color;
    layout(location = 0) out vec4 out_color;
    void main() {
        out_color = vec4(frag_color, 1.0);
    }
"#;

struct ShaderPart {
    raw_shader: Vec<u32>,
    module: vk::ShaderModule,
    
}

impl ShaderPart {
    fn new(device: &ash::Device, compiler:&shaderc::Compiler, src:&str, kind: ShaderKind, name:&str) ->Result<Self, ShaderError> {
        let compile_options = shaderc::CompileOptions::new().unwrap();
        let raw_shader = compiler
            .compile_into_spirv(src, kind, name, "main", Some(&compile_options))
            .map_err(|e|ShaderError::ShadercError(e))?
            .as_binary().to_vec();

        let create_info = vk::ShaderModuleCreateInfo::default()
            .code(&raw_shader);

        let module = unsafe { device.create_shader_module(&create_info, None)}
            .map_err(|_|ShaderError::CompileFailed)?;

        Ok(Self{raw_shader, module})
    }

    fn stage(&self, stage:vk::ShaderStageFlags) -> vk::PipelineShaderStageCreateInfo<'_> {
        vk::PipelineShaderStageCreateInfo::default()
            .module(self.module)
            .name(c"main")
            .stage(stage)
    }
}

pub struct Shader<'a> {
    vertex: ShaderPart,
    fragment: ShaderPart,
    device:&'a ash::Device
}

impl<'a> Shader<'a> {
    pub fn new(device: &'a ash::Device, compiler: &shaderc::Compiler) -> Result<Self, ShaderError> {
        Ok(Self {
            vertex:   ShaderPart::new(device, compiler, VERTEX_SRC,   ShaderKind::Vertex,   "vertex.glsl")?,
            fragment: ShaderPart::new(device, compiler, FRAGMENT_SRC, ShaderKind::Fragment, "fragment.glsl")?,
            device
        })
    }

    pub fn stages(&self) -> [vk::PipelineShaderStageCreateInfo<'_>; 2] {
        [
            self.vertex.stage(vk::ShaderStageFlags::VERTEX),
            self.fragment.stage(vk::ShaderStageFlags::FRAGMENT)
        ]
    }
}

impl<'a> Drop for Shader<'a> {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_shader_module(self.vertex.module, None);
            self.device.destroy_shader_module(self.fragment.module, None);
        }
    }
}