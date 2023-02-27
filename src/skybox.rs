use bevy::{
    asset::LoadState,
    input::mouse::MouseMotion,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupError, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
            OwnedBindingResource, PreparedBindGroup, RenderPipelineDescriptor, SamplerBindingType,
            ShaderRef, ShaderStages, SpecializedMeshPipelineError, TextureSampleType,
            TextureViewDescriptor, TextureViewDimension,
        },
        renderer::RenderDevice,
        texture::{CompressedImageFormats, FallbackImage},
    },
};


pub struct SkyboxPlugin;
struct Cubemap{
    is_loaded : bool,
    index: usize,
    image_handle: Handle<Image>,
}
impl Plugin for SkyboxPlugin{
    fn build(&self, app : &mut App){
        app.add_plugin(MaterialPlugin::<CubemapMaterial>::default())
        .add_startup_system(setup)
        .add_system(asset_loaded);
    }
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
    )
{
    let skybox_handle = asset_server.load("textures/skybox/Ryfjallet_cubemap.png");

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}
fn asset_loaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cubemap_materials: ResMut<Assets<CubemapMaterial>>,
    mut cubemap: ResMut<Cubemap>,
    ) {

    if !cubemap.is_loaded
        && asset_server.get_load_state(cubemap.image_handle.clone_weak()) == LoadState::Loaded
    {
        let mut image = images.get_mut(&cubemap.image_handle.clone_weak()).unwrap();
        if image.texture_descriptor.array_layer_count() == 1 {
                image.reinterpret_stacked_2d_as_array(
                    image.texture_descriptor.size.height / image.texture_descriptor.size.width,
                );
                image.texture_view_descriptor = Some(TextureViewDescriptor {
                    dimension: Some(TextureViewDimension::Cube),
                    ..default()
                });
            }

        commands.spawn_bundle(MaterialMeshBundle::<CubemapMaterial> {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 10000.0 })),
                    material: cubemap_materials.add(CubemapMaterial {
                        base_color_texture: Some(cubemap.image_handle.clone_weak()),
                    }),
                    ..default()
                });

        //commands.insert_resource(Cubemap {
        //    is_loaded: false,
        //    index: 0,
        //    image_handle: skybox_handle,
        //});
        cubemap.is_loaded = true;
    }
}
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "9509a0f8-3c05-48ee-a13e-a93226c7f488"]
struct CubemapMaterial {
    base_color_texture: Option<Handle<Image>>,
}
impl Material for CubemapMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
       "shaders/cubemap_unlit.wgsl".into() 
    }

    fn specialize(
            pipeline: &bevy::pbr::MaterialPipeline<Self>,
            descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
            layout: &bevy::render::mesh::MeshVertexBufferLayout,
            key: bevy::pbr::MaterialPipelineKey<Self>,
        ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
impl AsBindGroup for CubemapMaterial {
    type Data = ();

    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        images: &RenderAssets<Image>,
        _fallback_image: &FallbackImage,
    ) -> Result<PreparedBindGroup<Self>, AsBindGroupError> {
        let base_color_texture = self
            .base_color_texture
            .as_ref()
            .ok_or(AsBindGroupError::RetryNextUpdate)?;
        let image = images
            .get(base_color_texture)
            .ok_or(AsBindGroupError::RetryNextUpdate)?;
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&image.texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&image.sampler),
                },
            ],
            label: Some("cubemap_texture_material_bind_group"),
            layout,
        });

        Ok(PreparedBindGroup {
            bind_group,
            bindings: vec![
                OwnedBindingResource::TextureView(image.texture_view.clone()),
                OwnedBindingResource::Sampler(image.sampler.clone()),
            ],
            data: (),
        })
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                // Cubemap Base Color Texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::Cube,
                    },
                    count: None,
                },
                // Cubemap Base Color Texture Sampler
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: None,
        })
    }
}
