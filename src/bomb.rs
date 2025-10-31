use bevy::prelude::*;
use crate::{
    components::*,
    constants::*,
    map::grid_to_world,
    resource::{GameState, GameTextures, GameAudio, GameOverDelay},
};

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(place_bomb.in_set(OnUpdate(GameState::InGame)))
            .add_system(bomb_timer.in_set(OnUpdate(GameState::InGame)))
            .add_system(explosion_timer.in_set(OnUpdate(GameState::InGame)))
            .add_system(check_game_over.in_set(OnUpdate(GameState::InGame)))
            .add_system(game_over_delay_timer.in_set(OnUpdate(GameState::InGame)));
    }
}

/// 放置炸弹（使用TextureAtlasSprite）
fn place_bomb(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    game_audio: Res<GameAudio>,
    audio: Res<Audio>,
    player_query: Query<&GridPosition, (With<Player>, Without<Stop>)>,
    bomb_query: Query<&GridPosition, With<Bomb>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok(player_pos) = player_query.get_single() {
            // 检查当前位置是否已有炸弹
            let has_bomb = bomb_query.iter().any(|bomb_pos| bomb_pos == player_pos);
            
            if !has_bomb {
                let mut world_pos = grid_to_world(player_pos.x, player_pos.y);
                world_pos.z = 5.0; // 设置Z轴，确保炸弹显示在前面
                
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: game_textures.bomb.clone(),
                        sprite: TextureAtlasSprite::new(0), 
                        transform: Transform::from_translation(world_pos).with_scale(Vec3::splat(3.5)),
                        ..default()
                    },
                    Bomb {
                        timer: Timer::from_seconds(BOMB_TIMER, TimerMode::Once),
                        range: EXPLOSION_RANGE,
                    },
                    *player_pos,
                ));
                
                // 播放放置炸弹音效（音量60%）
                audio.play_with_settings(
                    game_audio.bomb_place.clone(),
                    PlaybackSettings {
                        repeat: false,
                        volume: 0.2,
                        speed: 1.0,
                    },
                );
            }
        }
    }
}

/// 炸弹计时器
fn bomb_timer(
    mut commands: Commands,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    game_audio: Res<GameAudio>,
    audio: Res<Audio>,
    mut bomb_query: Query<(Entity, &mut Bomb, &GridPosition)>,
    wall_query: Query<(Entity, &GridPosition), (With<BreakableWall>, Without<Bomb>)>,
    solid_wall_query: Query<&GridPosition, (With<Wall>, Without<Bomb>)>,
    enemy_query: Query<(Entity, &GridPosition), (With<Enemy>, Without<Bomb>)>,
    player_query: Query<(Entity, &GridPosition), (With<Player>, Without<Bomb>)>,
) {
    for (bomb_entity, mut bomb, bomb_pos) in bomb_query.iter_mut() {
        bomb.timer.tick(time.delta());
        
        if bomb.timer.finished() {
            // 炸弹爆炸
            commands.entity(bomb_entity).despawn();
            
            // 播放爆炸音效
            audio.play_with_settings(
                game_audio.bomb_explosion.clone(),
                PlaybackSettings {
                    repeat: false,
                    volume: 0.2,
                    speed: 1.0,
                },
            );
            
            // 生成爆炸效果
            create_explosion(&mut commands, &game_textures, &game_audio, &audio, bomb_pos, bomb.range, &wall_query, &solid_wall_query, &enemy_query, &player_query);
        }
    }
}

/// 创建爆炸效果（修复穿透不可破坏墙的bug，使用TextureAtlasSprite）
fn create_explosion(
    commands: &mut Commands,
    game_textures: &GameTextures,
    game_audio: &GameAudio,
    audio: &Audio,
    center: &GridPosition,
    range: i32,
    wall_query: &Query<(Entity, &GridPosition), (With<BreakableWall>, Without<Bomb>)>,
    solid_wall_query: &Query<&GridPosition, (With<Wall>, Without<Bomb>)>,
    enemy_query: &Query<(Entity, &GridPosition), (With<Enemy>, Without<Bomb>)>,
    player_query: &Query<(Entity, &GridPosition), (With<Player>, Without<Bomb>)>,
) {
    let directions = [
        (0, 0),   // 中心
        (1, 0),   // 右
        (-1, 0),  // 左
        (0, 1),   // 下
        (0, -1),  // 上
    ];
    
    for (dx, dy) in directions.iter() {
        for i in 0..=range {
            let x = center.x + dx * i;
            let y = center.y + dy * i;
            let pos = GridPosition::new(x, y);
            
            // 检查是否碰到不可破坏墙（应该停止传播）
            let mut hit_solid_wall = false;
            for solid_pos in solid_wall_query.iter() {
                if solid_pos.x == x && solid_pos.y == y {
                    hit_solid_wall = true;
                    break;
                }
            }
            
            // 如果碰到不可破坏墙，停止该方向的爆炸传播
            if hit_solid_wall {
                break;
            }
            
            // 生成爆炸特效
            spawn_explosion(commands, game_textures, &pos);
            
            // 检查是否摧毁可破坏墙
            let mut wall_destroyed = false;
            for (wall_entity, wall_pos) in wall_query.iter() {
                if wall_pos.x == x && wall_pos.y == y {
                    commands.entity(wall_entity).despawn();
                    wall_destroyed = true;
                    break;
                }
            }
            
            // 检查是否击中敌人
            for (enemy_entity, enemy_pos) in enemy_query.iter() {
                if enemy_pos.x == x && enemy_pos.y == y {
                    commands.entity(enemy_entity).despawn();
                    // 播放敌人死亡音效（音量65%）
                    audio.play_with_settings(
                        game_audio.enemy_explosion.clone(),
                        PlaybackSettings {
                            repeat: false,
                            volume: 0.2,
                            speed: 1.0,
                        },
                    );
                }
            }
            
            // 检查是否击中玩家
            for (player_entity, player_pos) in player_query.iter() {
                if player_pos.x == x && player_pos.y == y {
                    commands.entity(player_entity).despawn();
                    // 播放玩家死亡音效（音量70%）
                    audio.play_with_settings(
                        game_audio.player_explosion.clone(),
                        PlaybackSettings {
                            repeat: false,
                            volume: 0.1,
                            speed: 1.0,
                        },
                    );
                }
            }
            
            // 如果遇到可破坏墙，也停止该方向的爆炸传播
            if wall_destroyed {
                break;
            }
        }
    }
}

/// 生成爆炸特效（使用火焰图片TextureAtlasSprite）
fn spawn_explosion(commands: &mut Commands, game_textures: &GameTextures, pos: &GridPosition) {
    let mut world_pos = grid_to_world(pos.x, pos.y);
    world_pos.z = 8.0; // 爆炸效果显示在最上层
    
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: game_textures.fire.clone(),
            sprite: TextureAtlasSprite::new(8), // 使用第8帧（中心火焰）
            transform: Transform::from_translation(world_pos).with_scale(Vec3::splat(3.5)),
            ..default()
        },
        Explosion {
            timer: Timer::from_seconds(EXPLOSION_DURATION, TimerMode::Once),
        },
    ));
}

/// 爆炸效果计时器
fn explosion_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut explosion_query: Query<(Entity, &mut Explosion)>,
) {
    for (entity, mut explosion) in explosion_query.iter_mut() {
        explosion.timer.tick(time.delta());
        
        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// 检查游戏结束条件（触发1秒延迟）
fn check_game_over(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    delay: Option<Res<GameOverDelay>>,
) {
    // 如果已经有延迟计时器，不重复创建
    if delay.is_some() {
        return;
    }
    
    // 玩家死亡 -> 1秒后游戏结束
    if player_query.is_empty() {
        commands.insert_resource(GameOverDelay {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            next_state: GameState::GameOver,
        });
    }
    
    // 所有敌人被消灭 -> 1秒后胜利
    if enemy_query.is_empty() {
        commands.insert_resource(GameOverDelay {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            next_state: GameState::Victory,
        });
    }
}

/// 游戏结束延迟计时器
fn game_over_delay_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut delay: Option<ResMut<GameOverDelay>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(ref mut delay) = delay {
        delay.timer.tick(time.delta());
        
        if delay.timer.finished() {
            let target_state = delay.next_state;
            commands.remove_resource::<GameOverDelay>();
            next_state.set(target_state);
        }
    }
}
