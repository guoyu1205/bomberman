use bevy::prelude::*;
use crate::{
    components::*,
    constants::*,
    map::grid_to_world,
    resource::{GameState, GameTextures, GameAudio},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_enemies.in_schedule(OnEnter(GameState::InGame)))
            .add_system(enemy_movement.in_set(OnUpdate(GameState::InGame)))
            .add_system(check_player_enemy_collision.in_set(OnUpdate(GameState::InGame)));
    }
}

/// 生成敌人（使用TextureAtlasSprite）
fn spawn_enemies(mut commands: Commands, game_textures: Res<GameTextures>) {
    // 在地图的几个角落放置敌人
    let enemy_positions = [
        (11, 1),
        (11, 11),
        (1, 11),
    ];
    
    for (x, y) in enemy_positions.iter() {
        let grid_pos = GridPosition::new(*x, *y);
        let mut world_pos = grid_to_world(grid_pos.x, grid_pos.y);
        world_pos.z = 10.0; // 敌人在最上层
        
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: game_textures.enemy.clone(),
                sprite: TextureAtlasSprite::new(0), // 使用第0帧
                transform: Transform::from_translation(world_pos).with_scale(Vec3::splat(3.5)),
                ..default()
            },
            Enemy,
            grid_pos,
            Speed(ENEMY_SPEED),
            EnemyDirection::random(), // 随机初始方向
            EnemyMoveTimer {
                timer: Timer::from_seconds(0.5, TimerMode::Repeating), // 每0.5秒尝试移动一次
            },
        ));
    }
}

/// 敌人移动系统（离散移动，每0.5秒跳一格）
fn enemy_movement(
    time: Res<Time>,
    mut enemy_query: Query<
        (&mut Transform, &mut GridPosition, &mut EnemyDirection, &mut EnemyMoveTimer),
        (With<Enemy>, Without<Stop>)
    >,
    wall_query: Query<&GridPosition, (Or<(With<Wall>, With<BreakableWall>)>, Without<Enemy>)>,
    bomb_query: Query<&GridPosition, (With<Bomb>, Without<Enemy>)>,
) {
    for (mut transform, mut grid_pos, mut direction, mut move_timer) in enemy_query.iter_mut() {
        // 更新移动计时器
        move_timer.timer.tick(time.delta());
        
        if move_timer.timer.just_finished() {
            // 计算新位置
            let new_x = grid_pos.x + direction.x;
            let new_y = grid_pos.y + direction.y;
            let new_grid = GridPosition::new(new_x, new_y);
            
            // 检查新位置是否被阻挡
            let blocked = wall_query.iter().any(|pos| *pos == new_grid)
                || bomb_query.iter().any(|pos| *pos == new_grid);
            
            if !blocked {
                // 可以移动
                *grid_pos = new_grid;
                let mut new_world_pos = grid_to_world(new_grid.x, new_grid.y);
                new_world_pos.z = 10.0;
                transform.translation = new_world_pos;
            } else {
                // 被阻挡，换个随机方向
                *direction = EnemyDirection::random();
            }
        }
    }
}

/// 检查玩家与敌人的碰撞
fn check_player_enemy_collision(
    mut commands: Commands,
    game_audio: Res<GameAudio>,
    audio: Res<Audio>,
    player_query: Query<(Entity, &GridPosition), With<Player>>,
    enemy_query: Query<&GridPosition, With<Enemy>>,
) {
    if let Ok((player_entity, player_pos)) = player_query.get_single() {
        // 检查玩家是否与任何敌人在同一格
        for enemy_pos in enemy_query.iter() {
            if player_pos == enemy_pos {
                // 玩家被敌人杀死
                commands.entity(player_entity).despawn();
                // 播放玩家死亡音效
                audio.play_with_settings(
                    game_audio.player_explosion.clone(),
                    PlaybackSettings {
                        repeat: false,
                        volume: 0.2,
                        speed: 1.0,
                    },
                );
                return;
            }
        }
    }
}
