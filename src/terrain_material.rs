use bevy::{
    pbr::Material,
    prelude::{Color, Handle, Image},
    reflect::TypeUuid,
    render::render_resource::*,
};

#[uuid = "2d9582a2-7f19-4f1b-9633-a9ab3276838e"]
#[derive(AsBindGroup, TypeUuid, Clone)]
pub struct TerrainMaterial {
    #[uniform(0)]
    pub base_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub color_tex_handle: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub normal_map_texture: Handle<Image>,
}

impl Material for TerrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain.wgsl".into()
    }
}
