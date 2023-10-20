use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(Update, cursor_grab)
            .add_systems(Update, player_move)
            .add_systems(Update, player_look);
    }
}

#[derive(Component)]
struct Player;

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
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            direction += transform.forward();
        };
        if keys.pressed(KeyCode::S) {
            direction += transform.back();
        };
        if keys.pressed(KeyCode::D) {
            direction += transform.right();
        };
        if keys.pressed(KeyCode::A) {
            direction += transform.left();
        };

        direction.y = 0.0;

        if keys.pressed(KeyCode::Space) {
            direction += transform.up();
        };
        if keys.pressed(KeyCode::ShiftLeft) {
            direction += transform.down();
        };

        let movement = direction.normalize_or_zero() * 5.0 * time.delta_seconds();
        transform.translation += movement;
    }
}

fn player_look(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        if keys.pressed(KeyCode::Left) {
            yaw += 1.0f32.to_radians();
        };

        if keys.pressed(KeyCode::Right) {
            yaw -= 1.0f32.to_radians();
        };

        if keys.pressed(KeyCode::Up) {
            pitch += 1.0f32.to_radians();
        };

        if keys.pressed(KeyCode::Down) {
            pitch -= 1.0f32.to_radians();
        };

        let looking = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        transform.rotation = looking;
    }
}
