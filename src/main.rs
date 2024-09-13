use bevy::prelude::*;
use bevy_light_2d::prelude::*;

// Composants
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// Plugin principal
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup); // Ajouter la configuration de la caméra et de la lumière
           //.add_systems(Startup, add_people) // Ajouter les entités "personnes"
           //.add_systems(Update, hello_world) // Système qui affiche un message au monde
          // .add_systems(Update, update_people) // Met à jour les noms
           //.add_systems(Update, greet_people); // Système qui salue les personnes
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}


fn hello_world() {
    println!("Hello world !");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

// Ajout des entités "personnes"
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

// Mise à jour des noms des personnes
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            radius: 100.0,
            intensity: 3.0,
            ..default()
        },
        ..default()
    });
}
