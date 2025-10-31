/// 窗口宽度
pub const WINDOW_WIDTH: f32 = 780.0;
/// 窗口高度
pub const WINDOW_HEIGHT: f32 = 780.0;

/// 网格大小（多少行/列）
pub const GRID_SIZE: usize = 13;
/// 每个格子的像素大小
pub const CELL_SIZE: f32 = 60.0;

/// 玩家大小
pub const PLAYER_SIZE: f32 = 50.0;
/// 墙体大小
pub const WALL_SIZE: f32 = 60.0;
/// 炸弹大小
pub const BOMB_SIZE: f32 = 50.0;
/// 敌人大小
pub const ENEMY_SIZE: f32 = 50.0;
/// 火焰大小
pub const FIRE_SIZE: f32 = 55.0;

/// 炸弹爆炸时间（秒）
pub const BOMB_TIMER: f32 = 3.0;
/// 爆炸持续时间（秒）
pub const EXPLOSION_DURATION: f32 = 0.5;
/// 爆炸范围（格子数）
pub const EXPLOSION_RANGE: i32 = 2;

/// 玩家移动速度
pub const PLAYER_SPEED: f32 = 3.0;
/// 敌人移动速度
pub const ENEMY_SPEED: f32 = 100.0;

/// 背景图片
pub const BACKGROUND_SPRITE: &str = "images/background.png";

/// 资源路径
pub const PLAYER_SPRITE: &str = "images/player.png";
pub const ENEMY_SPRITE: &str = "images/creature.png";
pub const WALL_SPRITE: &str = "images/wall.png"; // 不可破坏墙
pub const BREAKABLE_WALL_SPRITE: &str = "images/door.png"; // 可破坏墙
pub const BOMB_SPRITE: &str = "images/bomb.png"; // 炸弹实体
pub const FIRE_SPRITE: &str = "images/fire.png"; // 爆炸火焰
pub const FONT_PATH: &str = "fonts/kenney_blocks.ttf";

/// 音频路径
pub const AUDIO_GAME_OVER: &str = "audios/game_over_bad_chest.wav"; // 游戏失败 (WAV格式)
pub const AUDIO_ENEMY_EXPLOSION: &str = "audios/enemy_explosion.ogg"; // 敌人死亡 (OGG格式)
pub const AUDIO_PLAYER_EXPLOSION: &str = "audios/player_explosion.ogg"; // 玩家死亡 (OGG格式)
pub const AUDIO_BOMB_PLACE: &str = "audios/bomb_place.ogg"; // 放置炸弹 (OGG格式)
pub const AUDIO_VICTORY: &str = "audios/victory.wav"; // 胜利音乐 (WAV格式，3秒)
pub const AUDIO_BOMB_EXPLOSION: &str = "audios/8bit_bomb_explosion.wav"; // 炸弹爆炸 (WAV格式)

/// 颜色定义
pub const COLOR_BACKGROUND: (f32, f32, f32) = (0.9, 0.9, 0.92); // 浅灰色偏白背景
