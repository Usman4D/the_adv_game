use crate::{gamemanager::WindowData, terrain_generation::TerrainSeersTag};
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::{na::Vector3, prelude::*};
pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerData {
    pub cursor_lock: bool,
    pub velocity: Vec3,
    pub speed: f32,
    pub acceleration: f32,

    pub surface_normal: Vec3,
}
#[derive(Component)]
pub struct PlayerTag;
#[derive(Component)]
pub struct PlayerRootTag;
#[derive(Component)]
pub struct PlayerCameraTag;

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            cursor_lock: true,
            velocity: Vec3::ZERO,
            speed: 10f32,
            acceleration: 1f32,
            surface_normal: Vec3::ZERO,
        }
    }
}
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(player_look)
            .add_system(player_freefall);
    }
}
fn spawn_player(mut commands: Commands) {
    /* commands.spawn().insert(PlayerTag).insert(Transform::from_xyz(-5.0,2.0,-5.0))
    .with_children(|parent| {
        // child cube
        parent.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 2.0)/* .looking_at(Vec3::ZERO, Vec3::Y) */,
            ..default()
        });
    }); */
    commands
        .spawn()
        .insert(PlayerTag)
        .insert(PlayerData {cursor_lock : true, ..Default::default()})
        .insert(PlayerRootTag)
        .insert(TerrainSeersTag)
        .insert_bundle(SpatialBundle::default())
        .insert(Transform::from_xyz(-10.0, 0.0, 0.0))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(PlayerCameraTag)
                .insert_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 2.0, 0.0), /* .looking_at(Vec3::ZERO, Vec3::Y) */
                    ..default()
                });
        });
}
fn player_freefall(
    mut query: Query<(&mut Transform, &mut PlayerData), With<PlayerRootTag>>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (mut transform, mut player_data) in query.iter_mut() {
        let ray_pos = transform.translation
            + Vec3 {
                x: 0f32,
                y: 2f32,
                z: 0f32,
            };
        let ray_dir = Vec3::new(0.0, -1.0, 0.0);
        let max_toi = 10.0;
        let solid = true;
        let groups = InteractionGroups::all();
        let filter = QueryFilter::new();

        if let Some((entity, ray)) =
            rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_toi, solid, filter)
        {
            // The first collider hit has the entity `entity` and it hit after
            // the ray travelled a distance equal to `ray_dir * toi`.
            let toi = ray.toi;
            player_data.surface_normal = ray.normal;
            // println!("Entity {:?} hit at point {}", entity, hit_point);

            if toi > 2.0 {
                transform.translation += ray_dir * time.delta_seconds() * 5.0;
            } else if toi < 1.8 {
                transform.translation -= ray_dir * time.delta_seconds() * 5.0;
            }
        }
    }
}
fn player_movement(
    mut player_query: Query<(Entity, &mut PlayerData), With<PlayerRootTag>>,
    mut transform_query: Query<&mut Transform>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (player, mut player_data) in player_query.iter_mut() {
        let mut move_vec = Vec3::ZERO;
        if let Ok(transform) = transform_query.get(player) {
            if keyboard_input.pressed(KeyCode::W) {
                let forw_vec = transform.right().cross(player_data.surface_normal);
                move_vec -= forw_vec;
            } else {
            }
            if keyboard_input.pressed(KeyCode::S) {
                let forw_vec = transform.right().cross(player_data.surface_normal);
                move_vec += forw_vec;
            }
            if keyboard_input.pressed(KeyCode::A) {
                let right_vec = transform.forward().cross(player_data.surface_normal);
                move_vec -= right_vec;
            }
            if keyboard_input.pressed(KeyCode::D) {
                let right_vec = transform.forward().cross(player_data.surface_normal);
                move_vec += right_vec;
            }
        }

        if !(keyboard_input.pressed(KeyCode::W)
            && keyboard_input.pressed(KeyCode::S)
            && keyboard_input.pressed(KeyCode::A)
            && keyboard_input.pressed(KeyCode::D))
        {
            let vel = player_data.velocity.clone();
            player_data.velocity -= vel * time.delta_seconds() * 5f32;
        }
        if let Ok(mut transform) = transform_query.get_mut(player) {
            let mut speed = 0.2f32;
            if keyboard_input.pressed(KeyCode::LShift) {
                speed = 0.4f32;
                if player_data.velocity.length() <= 0.4f32 {
                    player_data.velocity += move_vec * time.delta_seconds() * speed;
                }
            } else {
                if player_data.velocity.length() <= 0.2f32 {
                    player_data.velocity += move_vec * time.delta_seconds() * speed;
                }
            }
            transform.translation += player_data.velocity.clone();
        }
    }
}
fn player_look(
    mut set: ParamSet<(
        Query<(&mut Transform), With<PlayerCameraTag>>,
        Query<&mut Transform, With<PlayerRootTag>>,
    )>,
    window_data: Query<&WindowData>,
    mut mouse_evr: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let cursor_lock: bool = window_data.get_single().unwrap().cursor_lock;
    if !cursor_lock {
        return;
    }

    let mut delta = Vec2::ZERO;
    for ev in mouse_evr.iter() {
        delta += ev.delta * time.delta_seconds() * 0.4;
    }
    for mut transform in set.p0().iter_mut() {
        let pitch = Quat::from_rotation_x(-delta.y);
        transform.rotation = transform.rotation * pitch;
    }
    for mut transform in set.p1().iter_mut() {
        let yaw = Quat::from_rotation_y(-delta.x);
        transform.rotation = yaw * transform.rotation; // rotate around global y axis
    }
}
// fn player_debug(
//     query: Quey<GlobalTransform, With<PlayerCameraTag>>,
// ){
//     for global_transform in query.iter() {
//         //global_transform.com
//     }
// }
