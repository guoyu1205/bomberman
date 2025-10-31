use bevy::prelude::*;

mod components;
mod constants;
mod map;
mod player;
mod bomb;
mod enemy;
mod resource;
mod ui;

use constants::*;
use map::MapPlugin;
use player::PlayerPlugin;
use bomb::BombPlugin;
use enemy::EnemyPlugin;
use resource::{GameState, GameTextures, GameAudio, GamePaused};
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bomberman Game".to_owned(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .insert_resource(ClearColor(Color::rgb(
            COLOR_BACKGROUND.0,
            COLOR_BACKGROUND.1,
            COLOR_BACKGROUND.2,
        )))
        .init_resource::<GamePaused>()
        .add_startup_system(setup)
        .add_startup_system(setup_audio)
        .add_startup_system(setup_background)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BombPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(UIPlugin)
        .run();
}

/// 初始化设置 - 加载TextureAtlas精灵图集
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // 设置摄像机
    commands.spawn(Camera2dBundle::default());

    // 加载玩家精灵图集 (14列×4行 = 56帧)
    let player_texture_handle = asset_server.load(PLAYER_SPRITE);
    let player_texture_atlas =
        TextureAtlas::from_grid(player_texture_handle, Vec2::new(16.0, 16.0), 14, 4, None, None);

    // 加载炸弹精灵图集 (3列×1行 = 3帧)
    let bomb_texture_handle = asset_server.load(BOMB_SPRITE);
    let bomb_texture_atlas =
        TextureAtlas::from_grid(bomb_texture_handle, Vec2::new(16.0, 16.0), 3, 1, None, None);

    // 加载火焰精灵图集 (4列×3行 = 12帧)
    let fire_texture_handle = asset_server.load(FIRE_SPRITE);
    let fire_texture_atlas =
        TextureAtlas::from_grid(fire_texture_handle, Vec2::new(16.0, 16.0), 4, 3, None, None);

    // 加载墙体精灵图集 (6列×1行 = 6帧)
    let wall_texture_handle = asset_server.load(WALL_SPRITE);
    let wall_texture_atlas =
        TextureAtlas::from_grid(wall_texture_handle, Vec2::new(16.0, 16.0), 6, 1, None, None);

    // 加载敌人精灵图集 (14列×1行 = 14帧)
    let enemy_texture_handle = asset_server.load(ENEMY_SPRITE);
    let enemy_texture_atlas =
        TextureAtlas::from_grid(enemy_texture_handle, Vec2::new(16.0, 16.0), 14, 1, None, None);

    // 创建游戏纹理资源，将精灵图集添加到 Bevy 的资源管理器
    let game_textures = GameTextures {
        player: texture_atlases.add(player_texture_atlas),
        enemy: texture_atlases.add(enemy_texture_atlas),
        wall: texture_atlases.add(wall_texture_atlas),
        bomb: texture_atlases.add(bomb_texture_atlas),
        fire: texture_atlases.add(fire_texture_atlas),
        font: asset_server.load(FONT_PATH),
    };

    commands.insert_resource(game_textures);
}

/// 加载游戏音频资源
fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_audio = GameAudio {
        game_over: asset_server.load(AUDIO_GAME_OVER),
        enemy_explosion: asset_server.load(AUDIO_ENEMY_EXPLOSION),
        player_explosion: asset_server.load(AUDIO_PLAYER_EXPLOSION),
        bomb_place: asset_server.load(AUDIO_BOMB_PLACE),
        victory: asset_server.load(AUDIO_VICTORY),
        bomb_explosion: asset_server.load(AUDIO_BOMB_EXPLOSION),
    };
    
    commands.insert_resource(game_audio);
}

/// 设置背景精灵，填充窗口
fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load(BACKGROUND_SPRITE);
    commands.spawn(SpriteBundle {
        texture,
        sprite: Sprite {
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
        ..Default::default()
    });
}
