use bevy::prelude::*;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[derive(Component)]
pub struct WindowData {
    pub cursor_lock: bool,
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(WindowData { cursor_lock: true });
}
