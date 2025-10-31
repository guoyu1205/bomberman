use bevy::prelude::*;
use bevy::sprite::TextureAtlas;

/// 游戏纹理资源（使用TextureAtlas精灵图集）
#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<TextureAtlas>,
    pub enemy: Handle<TextureAtlas>,
    pub wall: Handle<TextureAtlas>,
    pub bomb: Handle<TextureAtlas>,
    pub fire: Handle<TextureAtlas>,
    pub font: Handle<Font>,
}

/// 游戏音频资源
#[derive(Resource)]
pub struct GameAudio {
    pub game_over: Handle<AudioSource>,
    pub enemy_explosion: Handle<AudioSource>,
    pub player_explosion: Handle<AudioSource>,
    pub bomb_place: Handle<AudioSource>,
    pub victory: Handle<AudioSource>,
    pub bomb_explosion: Handle<AudioSource>,
}

/// 游戏状态
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Welcome,
    InGame,
    Paused,
    Victory,
    GameOver,
}

/// 游戏结束延迟计时器
#[derive(Resource)]
pub struct GameOverDelay {
    pub timer: Timer,
    pub next_state: GameState,
}

/// 游戏暂停标记，不采用状态
#[derive(Resource, Default)]
pub struct GamePaused(pub bool);
