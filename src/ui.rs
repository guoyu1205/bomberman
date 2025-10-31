use bevy::prelude::*;
use crate::{
    components::*,
    constants::*,
    resource::{GameState, GameTextures, GameAudio, GamePaused},
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Welcome screen
            .add_system(cleanup_game_entities.in_schedule(OnEnter(GameState::Welcome))) // 进入主菜单时清理游戏实体
            .add_system(setup_welcome_ui.in_schedule(OnEnter(GameState::Welcome)))
            .add_system(welcome_input.in_set(OnUpdate(GameState::Welcome)))
            .add_system(cleanup_welcome_ui.in_schedule(OnExit(GameState::Welcome)))
            // Paused (使用资源标记，不切换状态)
            .add_system(handle_pause_toggle.in_set(OnUpdate(GameState::InGame)))
            .add_system(update_pause_ui.in_set(OnUpdate(GameState::InGame)).after(handle_pause_toggle))
            // Game over screen
            .add_system(cleanup_game_entities.in_schedule(OnEnter(GameState::GameOver))) // 进入游戏结束界面时清理游戏实体
            .add_system(setup_gameover_ui.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(gameover_input.in_set(OnUpdate(GameState::GameOver)))
            .add_system(cleanup_gameover_ui.in_schedule(OnExit(GameState::GameOver)))
            // Victory screen
            .add_system(cleanup_game_entities.in_schedule(OnEnter(GameState::Victory))) // 进入胜利界面时清理游戏实体
            .add_system(setup_victory_ui.in_schedule(OnEnter(GameState::Victory)))
            .add_system(victory_input.in_set(OnUpdate(GameState::Victory)))
            .add_system(cleanup_victory_ui.in_schedule(OnExit(GameState::Victory)));
    }
}

/// Welcome screen
fn setup_welcome_ui(mut commands: Commands, game_textures: Res<GameTextures>) {
    let text_style = TextStyle {
        font: game_textures.font.clone(),
        font_size: 90.0,
        color: Color::BLACK,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("BOMBERMAN\n\n", TextStyle { font_size: 90.0, color: Color::RED, ..text_style.clone() }),
                TextSection::new("CONTROLS:\n", text_style.clone()),
                TextSection::new("WASD / Arrow Keys - Move\n", TextStyle { font_size: 60.0, ..text_style.clone() }),
                TextSection::new("SPACE - Place Bomb\n", TextStyle { font_size: 60.0, ..text_style.clone() }),
                TextSection::new("P - Pause Game\n\n", TextStyle { font_size: 60.0, ..text_style.clone() }),
                TextSection::new("Press ENTER to Start", TextStyle { 
                    font_size: 60.0,
                    color: Color::rgb(1.0, 0.8, 0.0), 
                    ..text_style 
                }),
            ]).with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
        WelcomeUI,
    ));
}

fn welcome_input(
    keyboard: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Return) {
        next_state.set(GameState::InGame);
    }
}

fn cleanup_welcome_ui(
    mut commands: Commands,
    query: Query<Entity, With<WelcomeUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// 处理暂停切换（在 InGame 状态下按 P 或 R 键）
fn handle_pause_toggle(
    keyboard: Res<Input<KeyCode>>,
    mut game_paused: ResMut<GamePaused>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    stop_query: Query<Entity, With<Stop>>,
    mut bomb_query: Query<&mut Bomb>,
    mut explosion_query: Query<&mut Explosion>,
) {
    // 按 P 键切换暂停状态
    if keyboard.just_pressed(KeyCode::P) {
        game_paused.0 = !game_paused.0;
        
        if game_paused.0 {
            // 暂停：添加 Stop 组件，暂停计时器
            for entity in player_query.iter() {
                commands.entity(entity).insert(Stop);
            }
            for entity in enemy_query.iter() {
                commands.entity(entity).insert(Stop);
            }
            for mut bomb in bomb_query.iter_mut() {
                bomb.timer.pause();
            }
            for mut explosion in explosion_query.iter_mut() {
                explosion.timer.pause();
            }
        } else {
            // 恢复：移除 Stop 组件，恢复计时器
            for entity in stop_query.iter() {
                commands.entity(entity).remove::<Stop>();
            }
            for mut bomb in bomb_query.iter_mut() {
                bomb.timer.unpause();
            }
            for mut explosion in explosion_query.iter_mut() {
                explosion.timer.unpause();
            }
        }
    }
    
    // 暂停时按 R 恢复游戏
    if game_paused.0 && keyboard.just_pressed(KeyCode::R) {
        game_paused.0 = false;
        for entity in stop_query.iter() {
            commands.entity(entity).remove::<Stop>();
        }
        for mut bomb in bomb_query.iter_mut() {
            bomb.timer.unpause();
        }
        for mut explosion in explosion_query.iter_mut() {
            explosion.timer.unpause();
        }
    }
    
    // 暂停时按 ESC 返回主菜单
    if game_paused.0 && keyboard.just_pressed(KeyCode::Escape) {
        game_paused.0 = false;
        next_state.set(GameState::Welcome);
    }
}

/// 更新暂停 UI（根据暂停状态显示/隐藏）
fn update_pause_ui(
    mut commands: Commands,
    game_paused: Res<GamePaused>,
    game_textures: Res<GameTextures>,
    ui_query: Query<Entity, With<PausedUI>>,
) {
    let has_ui = !ui_query.is_empty();
    
    // 如果暂停但没有 UI，创建 UI
    if game_paused.0 && !has_ui {
        // 显示暂停 UI
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 0.0, 0.0, 0.7),
                    custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                ..default()
            },
            DimOverlay,
            PausedUI,
        ));
        
        let text_style = TextStyle {
            font: game_textures.font.clone(),
            font_size: 90.0,
            color: Color::WHITE,
        };

        commands.spawn((
            Text2dBundle {
                text: Text::from_sections([
                    TextSection::new("PAUSED\n\n", text_style.clone()),
                    TextSection::new("R - Resume Game\n", TextStyle { font_size: 60.0, color: Color::rgb(0.5, 1.0, 0.5), ..text_style.clone() }),
                    TextSection::new("ESC - Back to Menu", TextStyle { font_size: 60.0, color: Color::rgb(1.0, 0.5, 0.5), ..text_style }),
                ]).with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(0.0, 0.0, 101.0),
                ..default()
            },
            PausedUI,
        ));
    } 
    // 如果没暂停但有 UI，删除 UI
    else if !game_paused.0 && has_ui {
        for entity in ui_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Game over screen
fn setup_gameover_ui(mut commands: Commands, game_textures: Res<GameTextures>, game_audio: Res<GameAudio>, audio: Res<Audio>) {
    let text_style = TextStyle {
        font: game_textures.font.clone(),
        font_size: 90.0,
        color: Color::rgb(1.0, 0.3, 0.3),
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("GAME OVER!\n\n", text_style.clone()),
                TextSection::new("You were blown up\n\n", TextStyle { font_size: 60.0, color: Color::BLACK, ..text_style.clone() }),
                TextSection::new("ENTER - Try Again", TextStyle { 
                    font_size: 60.0,
                    color: Color::rgb(0.8, 0.8, 0.8), 
                    ..text_style 
                }),
            ]).with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
        GameOverUI,
    ));
    
    // 播放游戏失败音效（音量75%）
    audio.play_with_settings(
        game_audio.game_over.clone(),
        PlaybackSettings {
            repeat: false,
            volume: 0.2,
            speed: 1.0,
        },
    );
}

fn gameover_input(
    keyboard: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Return) {
        next_state.set(GameState::InGame);
    }
}

fn cleanup_gameover_ui(
    mut commands: Commands,
    query: Query<Entity, With<GameOverUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Victory screen
fn setup_victory_ui(
    mut commands: Commands, 
    game_textures: Res<GameTextures>, 
    game_audio: Res<GameAudio>, 
    audio: Res<Audio>,
) {
    let text_style = TextStyle {
        font: game_textures.font.clone(),
        font_size: 90.0,
        color: Color::rgb(1.0, 0.8, 0.0),
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("VICTORY!\n\n", text_style.clone()),
                TextSection::new("All enemies defeated\n\n", TextStyle { font_size: 60.0, color: Color::BLACK, ..text_style.clone() }),
                TextSection::new("ENTER - Play Again", TextStyle { 
                    font_size: 60.0,
                    color: Color::rgb(0.8, 0.8, 0.8), 
                    ..text_style 
                }),
            ]).with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
        VictoryUI,
    ));
    
    // 播放胜利音效（victory.wav 已剪辑为3秒，会自动停止）
    audio.play_with_settings(
        game_audio.victory.clone(),
        PlaybackSettings {
            repeat: false,
            volume: 0.2, // 音量 20%
            speed: 1.0,
        },
    );
}

fn victory_input(
    keyboard: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Return) {
        next_state.set(GameState::InGame);
    }
}

fn cleanup_victory_ui(
    mut commands: Commands,
    query: Query<Entity, With<VictoryUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Clean up game entities when exiting InGame state
fn cleanup_game_entities(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    bomb_query: Query<Entity, With<Bomb>>,
    explosion_query: Query<Entity, With<Explosion>>,
    wall_query: Query<Entity, Or<(With<Wall>, With<BreakableWall>)>>,
) {
    // Clean up all game entities
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in bomb_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in explosion_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in wall_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
