use bevy::{ecs::system::Resource, prelude::*};
use bevy_inspector_egui::bevy_egui::{egui, EguiContext, EguiPlugin};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Fbm, Perlin};
#[hot_lib_reloader::hot_module(dylib = "hotlib")]
mod hot_lib {
    hot_functions_from_file!("hotlib/src/lib.rs");
}
pub struct WorldGenUIPlugin;

impl Plugin for WorldGenUIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NoiseMapImage { ..default() })
            .add_startup_system(setup)
            .add_system(noiseimage_reload)
            .add_system(ui_example.after(noiseimage_reload));
    }
}
#[derive(Default)]
struct NoiseMapImage {
    is_initialized: bool,
    image: Handle<Image>,
}
fn setup(mut asset_server: ResMut<AssetServer>, mut noisemap: ResMut<NoiseMapImage>) {
    hot_lib::calc_noise();
    noisemap.image = asset_server.load("../example_images/texture_wood_planar.png");
    noisemap.is_initialized = false;
}

fn noiseimage_reload(
    mut asset_server: ResMut<AssetServer>,
    mut noisemap_image: ResMut<NoiseMapImage>,
    key: Res<Input<KeyCode>>,
) {
    if key.just_pressed(KeyCode::G) {
        hot_lib::calc_noise();
        asset_server.reload_asset("../example_images/texture_wood_planar.png");
    }
}
fn ui_example(
    mut egui_context: ResMut<EguiContext>,
    mut rendered_texture_id: Local<egui::TextureId>,
    mut noisemap: ResMut<NoiseMapImage>,
) {
    if !noisemap.is_initialized {
        noisemap.is_initialized = true;
        *rendered_texture_id = egui_context.add_image(noisemap.image.clone_weak());
    }

    egui::Window::new("WorldGenTool").show(egui_context.ctx_mut(), |ui| {
        ui.add(egui::widgets::Image::new(
            *rendered_texture_id,
            [256.0, 256.0],
        ));
        ui.label("Noisemap");
    });
}
