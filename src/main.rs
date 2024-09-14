use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_sprite, movement_system))
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct InactivityTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct IsAttacking(bool); // Indicateur pour savoir si le joueur est en train d'attaquer

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut InactivityTimer,
        &mut IsAttacking
    ), With<Player>>,
) {
    for (mut transform, mut indices, mut timer, mut inactivity_timer, mut is_attacking) in &mut query {
        let mut direction = Vec3::ZERO;

        if is_attacking.0 {
            timer.unpause();
            if timer.finished() {
                is_attacking.0 = false;
            }
            return;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            indices.first = 36;
            indices.last = 41;
            timer.reset();
            is_attacking.0 = true;
            timer.unpause();
            return;
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            transform.scale.x = -transform.scale.x.abs();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            transform.scale.x = transform.scale.x.abs();
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        if direction != Vec3::ZERO {
            transform.translation += direction * time.delta_seconds() * 200.0;
            inactivity_timer.reset();

            if direction.x != 0.0 {
                if indices.first != 24 || indices.last != 29 {
                    indices.first = 24;
                    indices.last = 29;
                    timer.reset();
                }
            } else if direction.y > 0.0 {
                if indices.first != 30 || indices.last != 35 {
                    indices.first = 30;
                    indices.last = 35;
                    timer.reset();
                }
            } else if direction.y < 0.0 {
                if indices.first != 18 || indices.last != 23 {
                    indices.first = 18;
                    indices.last = 23;
                    timer.reset();
                }
            }

            timer.unpause();
        } else {
            inactivity_timer.tick(time.delta());
            if inactivity_timer.finished() {
                if indices.first != 0 || indices.last != 5 {
                    indices.first = 0;
                    indices.last = 5;
                    timer.reset();
                }
                timer.unpause();
            } else {
                timer.pause();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("player/player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 5 };

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(6.0)),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        InactivityTimer(Timer::from_seconds(2.0, TimerMode::Once)),
        IsAttacking(false),
        Player,
    ));
}
