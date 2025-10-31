use bevy::prelude::*;
use crate::{
    components::*,
    constants::*,
    resource::{GameState, GameTextures},
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_map.in_schedule(OnEnter(GameState::InGame)));
    }
}

/// 设置地图
fn setup_map(mut commands: Commands, game_textures: Res<GameTextures>) {
    // 地图布局：0=空地, 1=不可破坏墙, 2=可破坏墙（零散分布）
    let map_layout = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 2, 0, 2, 0, 0, 2, 0, 0, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 2, 1, 0, 1, 0, 1],
        [1, 0, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 1, 0, 1],
        [1, 2, 0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 2, 1, 0, 1, 2, 1],
        [1, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 1],
        [1, 2, 1, 2, 1, 0, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 2, 0, 0, 0, 2, 2, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 1, 0, 1],
        [1, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    for (y, row) in map_layout.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let world_pos = grid_to_world(x as i32, y as i32);
            
            match cell {
                1 => {
                    // 不可破坏墙 - 使用索引5
                    spawn_wall(&mut commands, &game_textures, x as i32, y as i32, world_pos, false, 5);
                }
                2 => {
                    // 可破坏墙 - 使用索引3
                    spawn_wall(&mut commands, &game_textures, x as i32, y as i32, world_pos, true, 3);
                }
                _ => {} // 空地
            }
        }
    }
}

/// 生成墙体（使用TextureAtlasSprite）
fn spawn_wall(
    commands: &mut Commands,
    game_textures: &GameTextures,
    x: i32,
    y: i32,
    world_pos: Vec3,
    breakable: bool,
    sprite_index: usize,
) {
    let mut entity = commands.spawn(SpriteSheetBundle {
        texture_atlas: game_textures.wall.clone(),
        sprite: TextureAtlasSprite::new(sprite_index),
        transform: Transform {
            translation: world_pos,
            scale: Vec3::splat(3.5), // 放大精灵以适应60x60格子 (16*3.75=60) 
            ..default()
        },
        ..default()
    });

    entity.insert(GridPosition::new(x, y));

    if breakable {
        entity.insert(BreakableWall);
    } else {
        entity.insert(Wall);
    }
}

/// 网格坐标转世界坐标
pub fn grid_to_world(x: i32, y: i32) -> Vec3 {
    let offset = -(GRID_SIZE as f32 * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0;//cell_size是因为格子中心到格子边框
    Vec3::new(
        x as f32 * CELL_SIZE + offset,
        -(y as f32 * CELL_SIZE + offset), // Y轴反转
        0.0,
    )
}

/// 世界坐标转网格坐标
pub fn world_to_grid(pos: Vec3) -> GridPosition {
    let offset = -(GRID_SIZE as f32 * CELL_SIZE) / 2.0 + CELL_SIZE / 2.0;
    GridPosition::new(
        ((pos.x - offset) / CELL_SIZE).round() as i32,
        ((-pos.y - offset) / CELL_SIZE).round() as i32,
    )
}
