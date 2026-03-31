use crate::config::retro_effect::RetroEffectConfig;
use crate::gl;
use crate::gl::types::*;
use crate::renderer::shader::{ShaderProgram, ShaderVersion};

const POST_VERT_SHADER: &str = include_str!("../../res/post.v.glsl");
const POST_FRAG_SHADER: &str = include_str!("../../res/post.f.glsl");

pub struct PostProcessor {
    program: ShaderProgram,
    texture: GLuint,
    vao: GLuint,
    u_resolution: GLint,
    u_cell_height: GLint,
    u_scanline_intensity: GLint,
    u_glow_intensity: GLint,
    u_scanline_thickness: GLint,
    u_scanline_spacing: GLint,
    width: i32,
    height: i32,
}

impl PostProcessor {
    pub fn new(width: u32, height: u32) -> Result<Self, crate::renderer::Error> {
        let program =
            ShaderProgram::new(ShaderVersion::Glsl3, None, POST_VERT_SHADER, POST_FRAG_SHADER)?;

        let u_resolution = program.get_uniform_location(c"resolution")?;
        let u_cell_height = program.get_uniform_location(c"cellHeight")?;
        let u_scanline_intensity = program.get_uniform_location(c"scanlineIntensity")?;
        let u_glow_intensity = program.get_uniform_location(c"glowIntensity")?;
        let u_scanline_thickness = program.get_uniform_location(c"scanlineThickness")?;
        let u_scanline_spacing = program.get_uniform_location(c"scanlineSpacing")?;

        unsafe {
            gl::UseProgram(program.id());
            let scene_tex_loc = program.get_uniform_location(c"sceneTex")?;
            gl::Uniform1i(scene_tex_loc, 0);
            gl::UseProgram(0);
        }

        let mut texture = 0;
        let mut vao = 0;

        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::GenVertexArrays(1, &mut vao);
        }

        let mut pp = Self {
            program,
            texture,
            vao,
            u_resolution,
            u_cell_height,
            u_scanline_intensity,
            u_glow_intensity,
            u_scanline_thickness,
            u_scanline_spacing,
            width: 0,
            height: 0,
        };

        pp.resize(width, height);

        Ok(pp)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.width = width as i32;
        self.height = height as i32;

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as GLint,
                self.width,
                self.height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    /// Capture the current framebuffer content into our texture via
    /// `glCopyTexSubImage2D`, then draw the post-processing effect back.
    pub fn draw(&self, config: &RetroEffectConfig, cell_height: f32) {
        let w = self.width;
        let h = self.height;

        unsafe {
            let mut saved_viewport: [GLint; 4] = [0; 4];
            gl::GetIntegerv(gl::VIEWPORT, saved_viewport.as_mut_ptr());

            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::CopyTexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, 0, 0, w, h);
            gl::BindTexture(gl::TEXTURE_2D, 0);

            gl::Viewport(0, 0, w, h);

            gl::UseProgram(self.program.id());

            gl::Uniform2f(self.u_resolution, w as f32, h as f32);
            gl::Uniform1f(self.u_cell_height, cell_height);
            gl::Uniform1f(self.u_scanline_intensity, config.scanline_intensity.as_f32());
            gl::Uniform1f(self.u_glow_intensity, config.glow_intensity.as_f32());
            gl::Uniform1f(self.u_scanline_thickness, config.scanline_thickness);
            gl::Uniform1f(self.u_scanline_spacing, config.scanline_spacing);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);

            gl::Disable(gl::BLEND);

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);

            gl::Enable(gl::BLEND);

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::UseProgram(0);

            gl::Viewport(
                saved_viewport[0],
                saved_viewport[1],
                saved_viewport[2],
                saved_viewport[3],
            );
        }
    }
}

impl Drop for PostProcessor {
    fn drop(&mut self) {
        unsafe {
            if self.texture != 0 {
                gl::DeleteTextures(1, &self.texture);
            }
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }
        }
    }
}

impl std::fmt::Debug for PostProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PostProcessor")
            .field("texture", &self.texture)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
