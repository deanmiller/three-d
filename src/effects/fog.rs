use crate::*;

pub struct FogEffect {
    gl: Gl,
    program: program::Program,
    pub color: Vec3,
    pub density: f32,
    pub animation: f32,
    full_screen_positions: VertexBuffer,
    full_screen_uvs: VertexBuffer
}

impl FogEffect {

    pub fn new(gl: &Gl) -> Result<FogEffect, effects::Error>
    {
        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/effect.vert"),
                                                    include_str!("shaders/fog.frag"))?;

        let positions = vec![
            -3.0, -1.0, 0.0,
            3.0, -1.0, 0.0,
            0.0, 2.0, 0.0
        ];
        let uvs = vec![
            -1.0, 0.0,
            2.0, 0.0,
            0.5, 1.5
        ];
        let full_screen_positions = VertexBuffer::new_with_static_f32(&gl, &positions).unwrap();
        let full_screen_uvs = VertexBuffer::new_with_static_f32(&gl, &uvs).unwrap();
        Ok(FogEffect {gl: gl.clone(), program, color: vec3(0.8, 0.8, 0.8), density: 0.2, animation: 0.1,
            full_screen_positions, full_screen_uvs})
    }

    pub fn apply(&self, time: f32, camera: &camera::Camera, depth_texture: &Texture2DArray) -> Result<(), effects::Error>
    {
        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, state::DepthTestType::None);
        state::cull(&self.gl,state::CullType::Back);
        state::blend(&self.gl, state::BlendType::SrcAlphaOneMinusSrcAlpha);

        self.program.use_texture(depth_texture, "depthMap")?;

        self.program.add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;

        self.program.add_uniform_vec3("fogColor", &self.color)?;
        self.program.add_uniform_float("fogDensity", &self.density)?;
        self.program.add_uniform_float("animation", &self.animation)?;
        self.program.add_uniform_float("time", &(0.001 * time))?;
        self.program.add_uniform_vec3("eyePosition", camera.position())?;

        self.program.use_attribute_vec3_float(&self.full_screen_positions, "position").unwrap();
        self.program.use_attribute_vec2_float(&self.full_screen_uvs, "uv_coordinate").unwrap();
        self.program.draw_arrays(3);
        Ok(())
    }

}