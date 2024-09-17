# Bevy App

This is a simple Bevy application demonstrating sprite animation and movement systems. The app is built using the [Bevy engine](https://bevyengine.org/), a fast, simple, and flexible game engine written in Rust.

## Features

- **Sprite Animation**: Implements a basic system for sprite animation using Bevy's ECS (Entity Component System).
- **Movement System**: Provides a system to handle the movement of entities in the game.
- **Modular Setup**: The code is modularized into different systems (setup, animation, and movement) for better structure and scalability.

## Prerequisites

To run this app, you will need:

- [Rust](https://www.rust-lang.org/) installed on your system (use `rustup` for installation)
- The `bevy` crate, which will be installed automatically through Cargo

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/bevy-app.git
   cd bevy-app
   ```

2. Build and run the application:

   ```bash
   cargo run
   ```

Cargo will automatically download and compile Bevy and its dependencies on the first run.

## Code Structure

- **`main.rs`**: The entry point of the application, where the `App` is set up and systems are registered.
- **`setup.rs`**: Contains the setup system, which is run at the start of the application to initialize the game entities.
- **`animation.rs`**: Implements the `animate_sprite` system, responsible for animating the sprites in the game.
- **`movement.rs`**: Implements the `movement_system`, handling the movement of entities.

### `main.rs` Overview

```rust
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
```

- **`add_plugins`**: Adds the default plugins, with a modification to use nearest-neighbor scaling for images.
- **`add_systems(Startup, setup::setup)`**: Runs the `setup` system at startup to set up initial game state.
- **`add_systems(Update, ...)`**: Runs the `animate_sprite` and `movement_system` during the update phase of the game loop.

## How It Works

- **Setup**: When the app starts, the setup system is triggered to initialize the game environment (e.g., loading sprites, placing entities in the world).
- **Animation System**: Every game tick, the `animate_sprite` system updates the sprites, allowing for smooth animations.
- **Movement System**: The `movement_system` handles the input and moves entities accordingly, making the game interactive.
