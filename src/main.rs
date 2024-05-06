use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

const SKY_COLOR: Color = Color::rgb(135.0 / 255.0, 206.0 / 255.0, 250.0 / 255.0);
const MOVE_SPEED: f32 = 100.0;
const FALL_SPEED: f32 = 220.0;

#[derive(Debug, Component, Clone, Copy)]
struct HitBox(Vec2);

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
struct Coin;

#[derive(Component)]
struct Block;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 512.0,
        min_height: 288.0,
    };
    commands.spawn(camera);

    let player_texture = asset_server.load("pixil-frame-0.png");
    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            transform: Transform {
                translation: Vec3::new(-240.0, 0.0, 1.0),
                scale: Vec3::splat(1.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::splat(32.0)),
                ..default()
            },
            ..default()
        },
        Player { speed: 200.0 },
        Grounded(true),
        HitBox(Vec2::new(32.0, 32.0)),
    ));

    let block_texture = asset_server.load("brick.png");
    let block_positions = vec![
        Vec3::new(-240.0, -32.0, 0.0),
        Vec3::new(-160.0, 0.0, 0.0),
        Vec3::new(-89.0, 15.0, 0.0),
        Vec3::new(4.0, -32.0, 0.0),
        Vec3::new(94.0, -32.0, 0.0),
        Vec3::new(172.0, 19.0, 0.0),
        Vec3::new(240.0, 49.0, 0.0),
    ];

    for position in block_positions {
        commands.spawn((
            SpriteBundle {
                texture: block_texture.clone(),
                transform: Transform {
                    translation: position,
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(32.0)),
                    ..default()
                },
                ..default()
            },
            HitBox(Vec2::new(32.0, 32.0)),
            Block,
        ));
    }

    let coin_texture = asset_server.load("oldbrick.png");
    let coin_positions = vec![
        Vec3::new(-200.0, 60.0, 0.0),
        Vec3::new(-120.0, -40.0, 0.0),
        Vec3::new(40.0, 80.0, 0.0),
        Vec3::new(200.0, 0.0, 0.0),
    ];

    for position in coin_positions {
        commands.spawn((
            SpriteBundle {
                texture: coin_texture.clone(),
                transform: Transform {
                    translation: position,
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
                ..default()
            },
            Coin,
        ));
    }
}

#[derive(Component)]
struct Jump(f32);

fn player_jump(
    mut commands: Commands,
    time: Res<Time>,
    mut player: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
) {
    let Ok((player, mut transform, mut jump)) = player.get_single_mut() else {
        return;
    };
    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.0);
    jump.0 -= jump_power;
    transform.translation.y += jump_power + 10.0;
    // transform.translation.x += 3.0;
    if jump.0 == 0. {
        commands.entity(player).remove::<Jump>();
    }
}

fn player_fall(
    mut player_query: Query<(&mut Transform, &Player, &HitBox), Without<Jump>>,
    blocks_query: Query<(&HitBox, &Transform), Without<Player>>,
    time: Res<Time>,
) {
    let camera_height = 288.0;
    let camera_bottom = -camera_height / 2.0;

    for (mut transform, _, player_hitbox) in player_query.iter_mut() {
        let fall_amount = time.delta_seconds() * FALL_SPEED;
        let potential_new_position = Vec3::new(
            transform.translation.x,
            transform.translation.y - fall_amount,
            transform.translation.z,
        );

        if let Some(collision_y) =
            check_landing_collision(potential_new_position, player_hitbox, &blocks_query)
        {
            transform.translation.y = collision_y;
        } else {
            transform.translation.y -= fall_amount;
            if transform.translation.y < camera_bottom - 10.0 {
                transform.translation = Vec3::new(-240.0, 0.0, 1.0);
            }
        }
    }
}

fn check_landing_collision(
    new_position: Vec3,
    player_hitbox: &HitBox,
    blocks: &Query<(&HitBox, &Transform), Without<Player>>,
) -> Option<f32> {
    let player_bottom = new_position.y - player_hitbox.0.y / 2.0;

    for (block_hitbox, block_transform) in blocks.iter() {
        let block_top = block_transform.translation.y + block_hitbox.0.y / 2.0;
        let block_bottom = block_transform.translation.y - block_hitbox.0.y / 2.0;

        if player_bottom <= block_top
            && player_bottom >= block_bottom
            && check_horizontal_overlap(new_position, player_hitbox, block_transform, block_hitbox)
        {
            return Some(block_top + player_hitbox.0.y / 2.0);
        }
    }

    None
}

fn check_horizontal_overlap(
    player_position: Vec3,
    player_hitbox: &HitBox,
    block_transform: &Transform,
    block_hitbox: &HitBox,
) -> bool {
    let player_left = player_position.x - player_hitbox.0.x / 2.0;
    let player_right = player_position.x + player_hitbox.0.x / 2.0;
    let block_left = block_transform.translation.x - block_hitbox.0.x / 2.0;
    let block_right = block_transform.translation.x + block_hitbox.0.x / 2.0;

    player_right > block_left && player_left < block_right
}

fn character_movement(
    mut commands: Commands,
    mut player: Query<(
        &mut Transform,
        &Player,
        Option<&Jump>,
        Entity,
        &HitBox,
        &Grounded,
    )>,
    blocks: Query<(&HitBox, &Transform), Without<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player, jump, entity, player_hitbox, grounded) in player.iter_mut() {
        let movement_amt = player.speed * time.delta_seconds();
        let mut delta_x = 0.0;
        let mut delta_y = 0.0;

        if input.pressed(KeyCode::A) {
            delta_x = -movement_amt;
        }
        if input.pressed(KeyCode::D) {
            delta_x = movement_amt;
        }
        if input.pressed(KeyCode::S) && transform.translation.y < 600.0 {
            delta_y = -movement_amt;
        }

        let horizontal_pos = transform.translation + Vec3::X * delta_x;
        if !is_colliding(&horizontal_pos, player_hitbox, &blocks) {
            transform.translation.x = horizontal_pos.x;
        }

        let vertical_pos = transform.translation + Vec3::Y * delta_y;
        if !is_colliding(&vertical_pos, player_hitbox, &blocks) {
            transform.translation.y = vertical_pos.y;
        }

        if input.just_pressed(KeyCode::W) && jump.is_none() && grounded.0 {
            commands.entity(entity).insert(Jump(25.0));
        }
    }
}

fn is_colliding(
    new_position: &Vec3,
    player_hitbox: &HitBox,
    blocks: &Query<(&HitBox, &Transform), Without<Player>>,
) -> bool {
    for (block_hitbox, block_transform) in blocks.iter() {
        if check_hit(
            *player_hitbox,
            *new_position,
            *block_hitbox,
            block_transform.translation,
        ) {
            return true;
        }
    }
    false
}

fn check_hit(hitbox: HitBox, offset: Vec3, other_hitbox: HitBox, other_offset: Vec3) -> bool {
    let h_size = hitbox.0.y / 2.;
    let oh_size = other_hitbox.0.y / 2.;
    let w_size = hitbox.0.x / 2.;
    let ow_size = other_hitbox.0.x / 2.;

    offset.x + w_size > other_offset.x - ow_size
        && offset.x - w_size < other_offset.x + ow_size
        && offset.y + h_size > other_offset.y - oh_size
        && offset.y - h_size < other_offset.y + oh_size
}

#[derive(Component)]
struct Grounded(bool);

fn ground_detection(
    mut player: Query<(&Transform, &mut Grounded), With<Player>>,
    mut last: Local<Transform>,
) {
    let (pos, mut on_ground) = player.single_mut();
    let current = pos.translation.y == last.translation.y;
    if current != on_ground.0 {
        on_ground.0 = current;
    }
    *last = *pos;
}


use std::cmp::max;

fn coin_pickup(
    mut commands: Commands,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let coin_noise = asset_server.load("coin.ogg");
    let player_pos = player_query.single();
    for (coin, coin_pos) in coin_query.iter() {
        if coin_pos.translation.distance(player_pos.translation) <= 20.0 {
            commands.spawn(AudioBundle {
                source: coin_noise.clone(),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Once,
                    ..default()
                },
            });
            commands.entity(coin).despawn();
        }
    }
}

use std::env;

mod buffer;
mod canvas;
mod colors;

use canvas::Canvas;
use colors::Colors;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    
    if query == "ty" {
        // use linalg::vec2::Vec2;
        // use linalg::{matrix::Mat4x4, vec4::Vec4};
        use minifb::{Key, Window, WindowOptions};
        use rand::Rng;

        use mki::{bind_key, Action, InhibitEvent, Keyboard, Sequence};

        const WIDTH: usize = 640;
        const HEIGHT: usize = 360;


        let mut canvas = Canvas::new(WIDTH, HEIGHT);

        let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_millis(100)));

        let mut player_array_pos:(isize, isize) = (10, 10);

        let mut x_pos = 1;
        let mut forward = true;

        let mut level = 1;

        let mut right_velo: isize = 0;
        let mut up_velo: isize = 0;

        let mut victory_pos: (isize, isize) = (17, 17);

        let mut collisions = vec![vec![false; 19]; 19];

        for n in 0..18 {
            collisions[n][1] = true
        }

        while window.is_open() && !window.is_key_down(Key::Escape) {
            canvas.clear();
            player_array_pos.1 += up_velo;

            if player_array_pos.0 == 18 || player_array_pos.0  == 0 || player_array_pos.1 == 0 {
                println!("dead");
                if level == 1 {
                    player_array_pos = (10, 10);
                    right_velo = 0;
                    up_velo = 0;
                }else if level == 2 {
                    player_array_pos = (2, 5);
                    right_velo = 0;
                    up_velo = 0;
                }
                else if level == 3 {
                    player_array_pos = (17, 5);
                    right_velo = 0;
                    up_velo = 0;
                    x_pos = 1;
                    forward = true
                }
            }

            if player_array_pos.0 == 2 {
                up_velo = 0;
            }

            if mki::are_pressed(&[Keyboard::A]){
                right_velo = -1;
            }else if mki::are_pressed(&[Keyboard::D]){
                right_velo = 1;
            }else {
                right_velo = 0;
            }

            if collisions[player_array_pos.0 as usize][(player_array_pos.1 - 1) as usize] == false {
                up_velo = max(-1, up_velo - 1);
            }else {
                if level == 3 && (player_array_pos.0 == 2 || player_array_pos.0 == 3){
                    up_velo = 1;
                }else {
                    up_velo = 0;
                }
            }

            if collisions[player_array_pos.0 as usize][(player_array_pos.1) as usize] == true {
                up_velo = 0;
                player_array_pos.1 += 2;
            }

            if collisions[player_array_pos.0 as usize][(player_array_pos.1 - 1) as usize] == true && mki::are_pressed(&[Keyboard::W]) {
                up_velo = 3;
            }


            if collisions[(player_array_pos.0 - 1) as usize][player_array_pos.1 as usize] == false && right_velo < 0 {
                player_array_pos.0 -= 1;
            }

            if collisions[(player_array_pos.0 + 1) as usize][player_array_pos.1 as usize] == false && right_velo > 0 {
                player_array_pos.0 += 1;
            }

            if player_array_pos.0 == victory_pos.0 && 19 - victory_pos.1 == player_array_pos.1 {
                println!("victory");
                level += 1;
                if level == 1 {
                    player_array_pos = (10, 10);
                    right_velo = 0;
                    up_velo = 0;
                }else if level == 2 {
                    player_array_pos = (2, 5);
                    right_velo = 0;
                    up_velo = 0;
                    victory_pos = (14, 5);
                    collisions = vec![vec![false; 19]; 19];
                    collisions[2][0] = true;
                    collisions[3][0] = true;
                    collisions[2][1] = true;
                    collisions[3][1] = true;
                    for n in 0..6 {
                        collisions[6][n] = true;
                        collisions[7][n] = true;
                    }
                    for n in 0..10 {
                        collisions[10][n] = true;
                        collisions[11][n] = true;
                    }
                    for n in 0..14 {
                        collisions[14][n] = true;
                        collisions[15][n] = true;
                    }
                }else if level == 3 {
                    player_array_pos = (17, 5);
                    right_velo = 0;
                    up_velo = 0;
                    victory_pos = (15, 5);
                    collisions = vec![vec![false; 19]; 19];
                }
            }

            if level == 3 {
                collisions = vec![vec![false; 19]; 19];
                collisions[18-x_pos][1] = true;
                collisions[17-x_pos][1] = true;
                collisions[2][14 - x_pos] = true;
                collisions[3][14 - x_pos] = true;
                collisions[10][10] = true;
                collisions[11][10] = true;
            }

            let player_pos = ((player_array_pos.0 as f32 - 10.0) * 0.1, (player_array_pos.1 as f32 - 10.0) * 0.1);
            canvas.set_color(Colors::WHITE);

            if level == 1 {
                canvas.line((-1.0, -0.8), (1.0, -0.8));
            }

            if level == 2 {
                canvas.sqr((-0.8, -1.0), (-0.6, -1.0), (-0.6, -0.8), (-0.8, -0.8));
                canvas.sqr((-0.4, -1.0), (-0.2, -1.0), (-0.2, -0.4), (-0.4, -0.4));
                canvas.sqr((-0.0, -1.0), (0.2, -1.0), (0.2, -0.0), (-0.0, -0.0));
                canvas.sqr((0.4, -1.0), (0.6, -1.0), (0.6, 0.4), (0.4, 0.4));
            }

            if level == 3 {
                canvas.sqr((0.7 - (x_pos as f32 / 10.0), -0.8), (0.9 - (x_pos as f32 / 10.0), -0.8), (0.9 - (x_pos as f32 / 10.0), -0.9), (0.7 - (x_pos as f32 / 10.0), -0.9));
                canvas.sqr((-0.8, 0.4 - (x_pos as f32 / 10.0)), (-0.6, 0.4 - (x_pos as f32 / 10.0)), (-0.6, 0.3 - (x_pos as f32 / 10.0)), (-0.8, 0.3 - (x_pos as f32 / 10.0)));
                canvas.sqr((0.0, 0.0), (0.2, 0.0), (0.2, 0.1), (0.0, 0.1));
            }

            canvas.sqr((0.1 + player_pos.0, 0.1 + player_pos.1), (0.1 + player_pos.0, 0.0 + player_pos.1), (0.0 + player_pos.0, 0.0 + player_pos.1), (0.0 + player_pos.0, 0.1 + player_pos.1));

            canvas.set_color(Colors::BLUE);
            canvas.tri((0.15 + (victory_pos.0 - 10) as f32 / 10.0, 0.03 + (victory_pos.1 - 10) as f32 / -10.0), (0.1 + (victory_pos.0 - 10) as f32 / 10.0, 0.09 + (victory_pos.1 - 10) as f32  / -10.0), (0.05 + (victory_pos.0 - 10) as f32 / 10.0, 0.03 + (victory_pos.1 - 10) as f32  / -10.0));

            canvas.set_color(Colors::RED);
            canvas.line((-1.0, -1.0), (-1.0, 1.0));
            canvas.line((0.9, -1.0), (0.9, 1.0));

            if forward {
                x_pos += 1;
            } else {
                x_pos -= 1;
            }

            if x_pos == 12 {
                forward = !forward;
            }else if x_pos == 1 {
                forward = true;
            }

            if(level == 4){
                println!("ultimate mega winner");
            }

            window
            .update_with_buffer(canvas.buffer(), WIDTH, HEIGHT)
            .unwrap();
        }

    }
    
   if query == "trevor" {
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
            .build()
        )
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .add_systems(Update, player_jump)
        .add_systems(Update, player_fall)
        .add_systems(Update, ground_detection)
        .add_systems(Update, coin_pickup)
        .run();
    }
}
