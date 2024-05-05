use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

const SKY_COLOR: Color = Color::rgb(135.0 / 255.0, 206.0 / 255.0, 250.0 / 255.0);
const MOVE_SPEED: f32 = 100.0;
const FALL_SPEED: f32 = 98.0;

#[derive(Debug, Component, Clone, Copy)]
struct HitBox(Vec2);

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Resource)]
pub struct Coins(pub f32);

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
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::splat(1.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::splat(32.0)),
                ..default()
            },
            ..default()
        },
        Player { speed: 100.0 },
        Grounded(true),
        HitBox(Vec2::new(32.0, 32.0)),
    ));

    let block_texture = asset_server.load("brick.png");
    let block_positions = vec![
        Vec3::new(-64.0, -32.0, 0.0), 
        Vec3::new(64.0, -32.0, 0.0),  
        Vec3::new(0.0, -32.0, 0.0),   
        Vec3::new(192.0, 16.0, 0.0),  
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
    let Ok((player, mut transform, mut jump)) = player.get_single_mut() else { return; };
    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.0);
    jump.0 -= jump_power;
    transform.translation.y += jump_power;
    if jump.0 == 0. {
        commands.entity(player).remove::<Jump>();
    }
}

fn player_fall(
    mut player_query: Query<(&mut Transform, &Player, &HitBox), Without<Jump>>,
    blocks_query: Query<(&HitBox, &Transform), Without<Player>>,
    time: Res<Time>,
) {
    for (mut transform, _, player_hitbox) in player_query.iter_mut() {
        if transform.translation.y > 0.0 {
            let fall_amount = time.delta_seconds() * FALL_SPEED;
            let potential_new_position = Vec3::new(transform.translation.x, transform.translation.y - fall_amount, transform.translation.z);

            if let Some(collision_y) = check_landing_collision(potential_new_position, player_hitbox, &blocks_query) {
                transform.translation.y = collision_y;  
            } else {
                transform.translation.y -= fall_amount;
                if transform.translation.y < 0.0 {
                    transform.translation.y = 0.0;
                }
            }
        }
    }
}

fn check_landing_collision(
    new_position: Vec3, 
    player_hitbox: &HitBox, 
    blocks: &Query<(&HitBox, &Transform), Without<Player>>
) -> Option<f32> {

    let player_bottom = new_position.y - player_hitbox.0.y / 2.0;

    for (block_hitbox, block_transform) in blocks.iter() {
        let block_top = block_transform.translation.y + block_hitbox.0.y / 2.0;
        let block_bottom = block_transform.translation.y - block_hitbox.0.y / 2.0;

        if player_bottom <= block_top && player_bottom >= block_bottom {
            if check_horizontal_overlap(new_position, player_hitbox, block_transform, block_hitbox) {
                return Some(block_top + player_hitbox.0.y / 2.0);
            }
        }
    }

    None
}

fn check_horizontal_overlap(
    player_position: Vec3, 
    player_hitbox: &HitBox, 
    block_transform: &Transform, 
    block_hitbox: &HitBox
) -> bool {
    let player_left = player_position.x - player_hitbox.0.x / 2.0;
    let player_right = player_position.x + player_hitbox.0.x / 2.0;
    let block_left = block_transform.translation.x - block_hitbox.0.x / 2.0;
    let block_right = block_transform.translation.x + block_hitbox.0.x / 2.0;

    player_right > block_left && player_left < block_right
}



fn character_movement(
    mut commands: Commands,
    mut player: Query<(&mut Transform, &Player, Option<&Jump>, Entity, &HitBox)>,
    blocks: Query<(&HitBox, &Transform), Without<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player, jump, entity, player_hitbox) in player.iter_mut() {
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

        if input.just_pressed(KeyCode::W) && jump.is_none() {
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
        if check_hit(*player_hitbox, *new_position, *block_hitbox, block_transform.translation) {
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

    offset.x + w_size > other_offset.x - ow_size &&
        offset.x - w_size < other_offset.x + ow_size &&
        offset.y + h_size > other_offset.y - oh_size &&
        offset.y - h_size < other_offset.y + oh_size
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
        .add_systems(Update, player_jump)
        .add_systems(Update, player_fall)
        .add_systems(Update, ground_detection)
        .run();
}
