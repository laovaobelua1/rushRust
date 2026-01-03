use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;


use crate::components::*;
use crate::events::*;
use crate::resources::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const STAR_SIZE: f32 = 30.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const NUMBER_OF_STARS: usize = 10;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(
                    window.width() / 1.5,
                    window.height() / 1.5,
                    0.0,
                ),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player {},
        )
    );
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(
                        random_x,
                        random_y,
                        0.0,
                    ),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
                },
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },            
            )
            
        );
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(
                        random_x,
                        random_y,
                        0.0,
                    ),
                    texture: asset_server.load("sprites/star.png"),
                ..default()
                },
                Star {},            
            )
            
        );
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
        ),
        ..default()
        });

    
}



pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size; 

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;

    }
}

pub fn enemy_movement( 
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
   for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
   } 
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>, 
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let half_enemy_size = ENEMY_SIZE / 2.0;

        let x_min = half_enemy_size;
        let x_max = window.width() - half_enemy_size;
        let y_min = half_enemy_size;
        let y_max = window.height() - half_enemy_size;

        let mut direction_changed = false;

        if transform.translation.x <= x_min || transform.translation.x >= x_max {
            enemy.direction.x = -enemy.direction.x;
            direction_changed = true;
        }

        if transform.translation.y <= y_min || transform.translation.y >= y_max {
            enemy.direction.y = -enemy.direction.y;
            direction_changed = true;
        }

        if direction_changed {
            
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            audio.play(sound_effect);
            
            enemy.direction = enemy.direction.normalize();
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut enemy_transform in enemy_query.iter_mut() {
        
        let mut translation = enemy_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        enemy_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_query, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);

            if distance < (PLAYER_SIZE + ENEMY_SIZE) / 2.0 {
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);

                commands.entity(player_query).despawn();

                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

// pub fn enemy_hit_player(
//     mut commands: Commands,
//     player_query: Query<&Transform, With<Player>>,
//     enemy_query: Query<(Entity, &Transform), With<Enemy>>,
//     audio: Res<Audio>,
//     asset_server: Res<AssetServer>,
// ) {
//     if let Ok(player_transform) = player_query.get_single() {
//         for (enemy_entity, enemy_transform) in enemy_query.iter() {
//             let distance =
//                 player_transform.translation.distance(enemy_transform.translation);

//             if distance < (PLAYER_SIZE + ENEMY_SIZE) / 2.0 {
//                 let sound_effect =
//                     asset_server.load("audio/explosionCrunch_000.ogg");
//                 audio.play(sound_effect);

//                 // 💥 XOÁ ENEMY
//                 commands.entity(enemy_entity).despawn();
//                 break;
//             }
//         }
//     }
// }

pub fn player_collect_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>, 
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance =
                player_transform.translation.distance(star_transform.translation);

            if distance < (PLAYER_SIZE + STAR_SIZE) / 2.0 {
                score.value += 1;
                let sound_effect =
                    asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound_effect);

                commands.entity(star_entity).despawn();
                break;
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(
    time: Res<Time>,
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_star_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: ResMut<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window: &Window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(
                        random_x,
                        random_y,
                        0.0,
                    ),
                    texture: asset_server.load("sprites/star.png"),
                ..default()
                },
                Star {},            
            )
            
        );
    }
}

pub fn tick_enemy_spawn_timer(
    time: Res<Time>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemy_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window: &Window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(
                        random_x,
                        random_y,
                        0.0,
                    ),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
                },
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },            
            )
            
        );
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

pub fn handle_game_over(
    mut app_exit_events: EventWriter<AppExit>,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    for event in game_over_event_reader.iter() {
        println!("Game Over! Final Score: {}", event.score);
    }
}

pub fn update_high_score(
    mut high_score: ResMut<HighScore>,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    for event in game_over_event_reader.iter() {
        let player_name = "Player".to_string(); // In a real game, get the player's name

        high_score.score.push((player_name, event.score));
        
    }
}
