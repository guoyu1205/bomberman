use bevy::prelude::*;
use crate::{
    components::*,
    constants::*,
    map::grid_to_world,
    resource::{GameState, GameTextures},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_player.in_schedule(OnEnter(GameState::InGame)))
            .add_system(player_movement.in_set(OnUpdate(GameState::InGame)));
    }
}

/// 生成玩家（使用TextureAtlasSprite）
fn spawn_player(mut commands: Commands, game_textures: Res<GameTextures>) {
    let start_pos = GridPosition::new(1, 1);
    let mut world_pos = grid_to_world(start_pos.x, start_pos.y);
    world_pos.z = 10.0; // 玩家在最上层

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: game_textures.player.clone(),
            sprite: TextureAtlasSprite::new(43), // 使用第43帧（第4行第2列，玩家4动画）
            transform: Transform::from_translation(world_pos).with_scale(Vec3::splat(3.5)),
            ..default()
        },
        Player,
        start_pos,
        Speed(PLAYER_SPEED),
    ));
}

/// 玩家移动系统
fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut GridPosition, &Speed), (With<Player>, Without<Stop>)>,
    wall_query: Query<&GridPosition, (Or<(With<Wall>, With<BreakableWall>)>, Without<Player>)>,
) {
    if let Ok((mut transform, mut grid_pos, speed)) = player_query.get_single_mut() {
        let mut direction = Vec2::ZERO;

        // 检测按键（支持长按）
        if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
            direction.y = 1.0;
        } else if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
            direction.y = -1.0;
        }
        
        if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
            direction.x = -1.0;
        } else if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
            direction.x = 1.0;
        }

        if direction != Vec2::ZERO {
            // 计算移动
            //move_delta = 方向 × 速度 × 帧间隔 × 格子大小    //乘以格子大小转换为像素
            let move_delta = direction * speed.0 * time.delta_seconds() * CELL_SIZE;
            let new_pos = transform.translation + Vec3::new(move_delta.x, move_delta.y, 0.0);
            
            // 计算新的网格位置
            let new_grid = world_to_grid(new_pos);
            
            // 检查新位置是否可通过（只检查墙体，不检查炸弹）
            if !is_blocked(&new_grid, &wall_query) {
                // 更新网格位置和实际位置
                *grid_pos = new_grid;
                transform.translation = new_pos;
            }
        }
    }
}

/// 检查位置是否被阻挡（只检查墙体）
fn is_blocked(
    pos: &GridPosition,
    wall_query: &Query<&GridPosition, (Or<(With<Wall>, With<BreakableWall>)>, Without<Player>)>,
) -> bool {
    // 只检查墙体，炸弹不阻挡玩家
    for wall_pos in wall_query.iter() {
        if wall_pos == pos {
            return true;
        }
    }
    
    false
}

/// 世界坐标转网格坐标
fn world_to_grid(pos: Vec3) -> GridPosition {
    let offset = -(GRID_SIZE as f32 * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0;
    GridPosition::new(
        ((pos.x - offset) / CELL_SIZE).round() as i32,
        ((-pos.y - offset) / CELL_SIZE).round() as i32,
    )
}
