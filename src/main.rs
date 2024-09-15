use bevy::prelude::*;
mod animation;
mod movement;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup::setup)
        .add_systems(Update, (animation::animate_sprite, movement::movement_system))
        .run();
}
