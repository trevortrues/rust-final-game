use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
#[allow(unused)]
#[allow(dead_code)]

const SKY_COLOR: Color = Color::rgb(135.0 / 255.0, 206.0 / 255.0, 250.0 / 255.0);

#[derive(Debug, Component, Clone, Copy)]
struct HitBox(Vec2);

#[derive(Component)]
pub struct Player{
    pub speed: f32
}

#[derive(Resource)]
pub struct Coins(pub f32);




fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
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
        Player{speed: 100.0},
        Grounded(true),
    ));
}

const FALL_SPEED: f32 = 98.0;

#[derive(Component)]
struct Jump(f32);


fn player_jump(
    mut commands: Commands, time: Res<Time>,
    mut player: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
) {
    let Ok((player, mut transform, mut jump)) = player.get_single_mut() else {return;};
    let jump_power = (time.delta_seconds() * FALL_SPEED *2.).min(jump.0);
    jump.0 -= jump_power;
    transform.translation.y += jump_power;
    if jump.0 == 0. {commands.entity(player).remove::<Jump>();}
}
fn player_fall(
    mut player: Query<&mut Transform, (With<Player>, Without<Jump>)>,
    time: Res<Time>,
) {
    let Ok(mut player) = player.get_single_mut() else {return;};
    if player.translation.y > 0.0 {
        player.translation.y -= time.delta_seconds() * FALL_SPEED;
        if player.translation.y < 0.0 {
            player.translation.y = 0.0
        }
    }
}


fn character_movement(
    mut commands: Commands,
    mut characters: Query<(&mut Transform, &Player, Option<&Jump>, Entity)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player, jump, entity) in characters.iter_mut() {
        let movement_amt = player.speed * time.delta_seconds();
        if input.just_pressed(KeyCode::W) && jump.is_none() {
            commands.entity(entity).insert(Jump(25.0));
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

fn check_hit(hitbox: HitBox, offset: Vec3, other_hitbox: HitBox, other_offset: Vec3) -> bool {
    let h_size = hitbox.0.y /2.;
    let oh_size = other_hitbox.0.y /2.;
    let w_size = hitbox.0.x /2.;
    let ow_size = other_hitbox.0.x /2.;

    offset.x + w_size > other_offset.x - ow_size && offset.x - w_size < other_offset.x + ow_size &&
    offset.y + h_size > other_offset.y - oh_size && offset.y - h_size < other_offset.y + oh_size
}

#[derive(Component)]
struct Grounded(bool);

fn ground_detection(
    mut player: Query<(&Transform, &mut Grounded), With<Player>>,
    mut last: Local<Transform>,
) {
    let (pos,mut on_ground) = player.single_mut();
    let current = if pos.translation.y == last.translation.y {
        true
    } else {
        false
    };
    if current != on_ground.0 {
        on_ground.0 = current;
    }
    *last = *pos;
}


use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    
    if query == "ty" {
        println!("hii");
    }else if query == "trevor" {
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
            .add_systems(Update, player_jump)
            .add_systems(Update, player_fall)
            .add_systems(Update, ground_detection)
            .run();
    }
}
