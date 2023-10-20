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
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(KeyCode::Escape) {
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
                        if key == KeyCode::W {
                            direction += forward;
                        } else if key == KeyCode::S {
                            direction -= forward;
                        } else if key == KeyCode::D {
                            direction += right;
                        } else if key == KeyCode::A {
                            direction -= right;
                        } else if key == KeyCode::Space {
                            direction += Vec3::Y;
                        } else if key == KeyCode::ShiftLeft {
                            direction -= Vec3::Y;
                        }
                    }
                }

                let movement = direction.normalize_or_zero() * 5.0 * time.delta_seconds();
                transform.translation += movement;
            }
        }
    } else {
        warn!("Primary window not found for `player_move`!")
    }
}

fn player_look(
    // keys: Res<Input<KeyCode>>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.iter(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let window_scale = window.height().min(window.width());
                        print!("{}", window_scale);
                        pitch -= (0.0001f32 * ev.delta.y * window_scale).to_radians();
                        yaw -= (0.0001f32 * ev.delta.x * window_scale).to_radians();
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
