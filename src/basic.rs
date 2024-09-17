use bevy::prelude::*;

// 游戏逻辑
// 基础常量
pub const GRAVITY: f32 = 1.;  // 重力
// 组件、方法
// 物体类型
pub enum GameObjType {
    Player,  // 玩家
    Creature,  // 生物
    Cube,  // 方块
}

// 速度
// 基础常量
pub const SQUATTING_SPEED_INCREASES_MULT: f32 = 0.5;  // 蹲下速度增加倍率

// 组件
#[derive(Component)]
// 移动属性
pub struct Movement{  // 移动组件
    pub basic_speed: f32,  // 基础速度
    pub basic_jump_high: f32,  // 基础跳跃高度
    pub actual_speed: f32,  // 实际的速度
    pub actual_jump_high: f32,  // 实际的跳跃高度
    pub move_type: GameObjType,  // 移动物体类型
}


#[derive(Component)]
pub struct Player;  // 标记为玩家

#[derive(Component)]
pub struct CameraCom;  // 标记为相机

#[derive(Component)]
pub struct Cube;  // 标记为方块
