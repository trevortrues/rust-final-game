use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
#[allow(unused)]

const SKY_COLOR: Color = Color::rgb(135.0 / 255.0, 206.0 / 255.0, 250.0 / 255.0);

#[derive(Component)]
pub struct Player{
    pub speed: f32
}

#[derive(Resource)]
pub struct Coins(pub f32);




fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    
    camera.projection.scaling_mode = ScalingMode::AutoMin{
        min_width: 256.0,
        min_height:144.0
    };
   
    commands.spawn(camera);

    let texture = asset_server.load("pixil-frame-0.png");

    commands.spawn((
        SpriteBundle {
            texture,
            
            ..default()
        },
        Player {speed: 100.0},
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in characters.iter_mut() {
        let movement_amt = player.speed * time.delta_seconds();
        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amt;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amt;
        }
        if input.pressed(KeyCode::S) && transform.translation.y < 600.0 {
            transform.translation.y -= movement_amt;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amt;
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
