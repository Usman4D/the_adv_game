mod dual_contouring;
mod freecamera;
mod gamemanager;
mod player;
mod skybox;
mod terrain_generation;
mod terrain_material;
mod worldgen_ui;
use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings, texture::ImageSettings},
    window::{PresentMode, WindowMode},
};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use freecamera::FreeCameraPlugin;
use gamemanager::GameManagerPlugin;
use gamemanager::WindowData;
use player::PlayerData;
use player::PlayerPlugin;
use skybox::SkyboxPlugin;
use terrain_generation::TerrainGenerationPlugin;
use worldgen_ui::WorldGenUIPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "kairos".to_string(),
            resizable: false,
            present_mode: PresentMode::Fifo,
            mode: WindowMode::Fullscreen,
            ..Default::default()
        })
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .insert_resource(ImageSettings::default_linear())
        .insert_resource(ClearColor(Color::rgb(
            70.0 / 255.0,
            57.0 / 255.0,
            40.0 / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        //.add_plugin(WireframePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(WorldGenUIPlugin)
        .add_plugin(GameManagerPlugin)
        .add_startup_system(setup)
        .add_system(regennoise)
        .add_system(cursor_grab_system)
        .add_plugin(PlayerPlugin)
        // .add_plugin(FreeCameraPlugin)
        .add_plugin(TerrainGenerationPlugin)
        // .add_plugin(SkyboxPlugin)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: ResMut<Windows>,
) {
    for za in 0..2 {
        for ya in 0..2 {
            for xa in 0..2 {
                // cube
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(
                        (xa * 4) as f32,
                        (ya * 4) as f32,
                        (za * 4) as f32,
                    ),
                    ..default()
                });
            }
        }
    }

    let mut sun_tranform = Transform::from_xyz(0.0, 0.0, 0.0);
    sun_tranform.rotate_x(4.7);
    // directional light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: sun_tranform,
        ..Default::default()
    });
    // light
    //commands.spawn_bundle(PointLightBundle {
    //    point_light: PointLight {
    //        intensity: 1500.0,
    //        shadows_enabled: true,
    //        ..default()
    //    },
    //    transform: Transform::from_xyz(0.0, 5.0, 0.0),
    //    ..default()
    //});
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}
fn regennoise(key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::G) {
        // hot_lib::calc_noise();
    }
}
fn cursor_grab_system(
    mut windowdata_query: Query<&mut WindowData>,
    mut windows: ResMut<Windows>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();
    let mut windowsdata = windowdata_query.get_single_mut().unwrap();

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);

        windowsdata.cursor_lock = true;
    }

    if key.just_pressed(KeyCode::Q) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);

        windowsdata.cursor_lock = false;
    }
}
