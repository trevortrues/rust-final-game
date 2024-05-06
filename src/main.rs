use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
#[allow(unused)]
#[allow(dead_code)]

// const SKY_COLOR: Color = Color::from_u8_rgb(135.0 / 255.0 as u8, 206.0 / 255.0 as u8, 250.0 / 255.0 as u8);

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

use std::cmp::max;
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
