use bevy::prelude::*;

/// 玩家组件
#[derive(Component)]
pub struct Player;

/// 敌人组件
#[derive(Component)]
pub struct Enemy;

/// 敌人移动方向
#[derive(Component, Clone, Copy)]
pub struct EnemyDirection {
    pub x: i32,
    pub y: i32,
}

impl EnemyDirection {
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // 上下右左
        let (x, y) = directions[rng.gen_range(0..4)];
        Self { x, y }
    }
}

/// 敌人移动计时器
#[derive(Component)]
pub struct EnemyMoveTimer {
    pub timer: Timer,
}

/// 网格位置组件
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// 炸弹组件
#[derive(Component)]
pub struct Bomb {
    pub timer: Timer,
    pub range: i32,
}

/// 爆炸效果组件
#[derive(Component)]
pub struct Explosion {
    pub timer: Timer,
}

/// 墙体组件（不可破坏）
#[derive(Component)]
pub struct Wall;

/// 可破坏墙体组件
#[derive(Component)]
pub struct BreakableWall;

/// 移动速度组件
#[derive(Component)]
pub struct Speed(pub f32);

/// 欢迎界面UI标记
#[derive(Component)]
pub struct WelcomeUI;

/// 暂停界面UI标记
#[derive(Component)]
pub struct PausedUI;

/// 游戏结束界面UI标记
#[derive(Component)]
pub struct GameOverUI;

/// 胜利界面UI标记
#[derive(Component)]
pub struct VictoryUI;

/// 背景暗化遮罩
#[derive(Component)]
pub struct DimOverlay;

/// 暂停标记组件（用于暂停实体的行为）
#[derive(Component)]
pub struct Stop;
