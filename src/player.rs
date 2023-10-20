use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_move);
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
