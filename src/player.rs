use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .add_systems(Startup, spawn_player)
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(Update, cursor_grab)
            .add_systems(Update, player_move)
            .add_systems(Update, player_look);
    }
}

#[derive(Component)]
struct Player;

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

#[derive(Resource)]
struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.0001,
            speed: 5.,
        }
    }
}

#[derive(Resource)]
struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_right: KeyCode,
    pub move_left: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
    pub toggle_grab_cursor: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::W,
            move_backward: KeyCode::S,
            move_right: KeyCode::D,
            move_left: KeyCode::A,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ShiftLeft,
            toggle_grab_cursor: KeyCode::Escape,
        }
    }
}

fn spawn_player(mut commands: Commands) {
    let player = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Player,
    );

    commands.spawn(player);
}

fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    };
}

fn initial_grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    keybindings: Res<KeyBindings>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(keybindings.toggle_grab_cursor) {
            toggle_grab_cursor(&mut window);
        };
    } else {
        warn!("Primary window not found for `cursor_grab`!")
    }
}

fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Player>>,
    settings: Res<MovementSettings>,
    keybindings: Res<KeyBindings>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        for mut transform in query.iter_mut() {
            let mut direction = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for key in keys.get_pressed() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let key = *key;
                        if key == keybindings.move_forward {
                            direction += forward;
                        } else if key == keybindings.move_backward {
                            direction -= forward;
                        } else if key == keybindings.move_right {
                            direction += right;
                        } else if key == keybindings.move_left {
                            direction -= right;
                        } else if key == keybindings.move_ascend {
                            direction += Vec3::Y;
                        } else if key == keybindings.move_descend {
                            direction -= Vec3::Y;
                        }
                    }
                }

                let movement =
                    direction.normalize_or_zero() * settings.speed * time.delta_seconds();
                transform.translation += movement;
            }
        }
    } else {
        warn!("Primary window not found for `player_move`!")
    }
}

fn player_look(
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<Player>>,
    settings: Res<MovementSettings>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.iter(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }
                pitch = pitch.clamp(-1.54, 1.54);
                let looking =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
                transform.rotation = looking;
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!")
    }
}
