use bevy::prelude::*;
#[allow(unused)]

const SKY_COLOR: Color = Color::rgb(135.0 / 255.0, 206.0 / 255.0, 250.0 / 255.0);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("pixil-frame-0.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        texture,
        ..default()
    });
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in characters.iter_mut() {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 200.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 200.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) && transform.translation.y < 600.0 {
            transform.translation.y -= 200.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 200.0 * time.delta_seconds();
        }
    }
}



fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "rustfinal".into(),
                        resolution: (1200.0, 800.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
            })
            .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}