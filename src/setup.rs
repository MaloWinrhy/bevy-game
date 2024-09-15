use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct InactivityTimer(Timer);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct IsAttacking(pub bool);

pub fn setup(
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
