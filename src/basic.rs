use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

// 物理引擎
// 重力常量
pub const GRAVITY: f32 = 1.;  // 重力
// 渲染物体物理Bundle
#[derive(Bundle)]
pub struct PhysicsBundle {
    pub body: RigidBody,  // 创建刚体
    pub velocity: Velocity,  // 速度
    pub gravity_scale: GravityScale,  // 设置重力
    pub sleeping: Sleeping,  // 是否睡眠
    pub ccd: Ccd,  // 是否开启ccd
    pub mass: AdditionalMassProperties,  // 重量
    pub locked_axes: LockedAxes,  // 是否禁止旋转
    pub collider: Collider,  // 碰撞器
}


// 游戏逻辑
// 物体类型
#[derive(Component)]
pub enum GameObjType {
    Player,  // 玩家
    Creature,  // 生物
    Cube(Cube),  // 方块
}
// 为GameObjType实现clone trait
impl Clone for GameObjType {
    fn clone(&self) -> Self {
        match self {
            GameObjType::Player => GameObjType::Player,
            GameObjType::Creature => GameObjType::Creature,
            GameObjType::Cube(c) => GameObjType::Cube(c.clone()),
        }
    }
}

// 玩家速度
// 玩家速度常量
pub const SQUATTING_SPEED_INCREASES_MULT: f32 = 0.5;  // 蹲下速度增加倍率

// 玩家信息(在main.rs中初始化)
#[derive(Resource)]
pub struct PlayerInfo {
    pub player_map: HashMap<i32, HashMap<i32, Cube>>,  // 玩家地图哈希表
    pub is_controlling: bool,  // 玩家是否处于控制状态（是否没有呼出鼠标）
    pub is_paused: bool,  // 游戏是否处于暂停状态
    pub is_creative_mode: bool,  // 是否处于创造模式
    pub player_bar: [(Option<GameObjType>, isize); 5],  // 玩家物品栏  (Option<GameObjType>, 物品数量)
    pub player_bar_select_index: usize,  // 玩家当前手持物品在物品栏的索引
}

// 组件
// 移动属性
#[derive(Component)]
pub struct Movement{  // 移动组件
    pub basic_speed: f32,  // 基础速度
    pub basic_jump_high: f32,  // 基础跳跃高度
    pub actual_speed: f32,  // 实际的速度
    pub actual_jump_high: f32,  // 实际的跳跃高度
    pub move_type: GameObjType,  // 移动物体类型
}

// 游戏类型标记
// 标记为游戏背景
#[derive(Component)]
pub struct Background;
// 标记为玩家
#[derive(Component)]
pub struct Player;
// 标记为相机
#[derive(Component)]
pub struct CameraCom;
// 标记为鼠标
#[derive(Component)]
pub struct CursorCom;
// 标记为物品栏
#[derive(Component)]
pub struct BarCom;
// 标记为物品栏的选择器
#[derive(Component)]
pub struct BarSelectorCom;
// 标记为方块, 并指定方块类型
#[derive(Component, Debug)]
pub enum Cube {
    GrassCube,  // 草方块
    SoilCube,  // 土方块
    StoneCube,  // 石块
    Plank,  // 木板
    StoneBrick,  // 石砖块
}
// 为cube实现clone trait
impl Clone for Cube {
    fn clone(&self) -> Self {
        match self {
            Cube::GrassCube => Cube::GrassCube,
            Cube::SoilCube => Cube::SoilCube,
            Cube::StoneCube => Cube::StoneCube,
            Cube::Plank => Cube::Plank,
            Cube::StoneBrick => Cube::StoneBrick,
        }
    }
}
// 通过方块类型获取方块模块位置
pub fn get_cube_model(game_obj_type: &Cube)  -> &'static str {
    match game_obj_type {
        Cube::GrassCube => {
            "cube/grass.png"
        }
        Cube::SoilCube => {
            "cube/soil.png"
        }
        Cube::StoneCube => {
            "cube/stone.png"
        }
        Cube::Plank => {
            "cube/plank.png"
        }
        Cube::StoneBrick => {
            "cube/stone_brick.png"
        }
    }
}

// 显示在物品栏上的物品图标com
#[derive(Component)]
pub struct BarIconCom{
    pub bar_index: usize,  // 位于物品栏中的索引
}
// 物品栏上文字com
#[derive(Component)]
pub struct BarTextCom{
    pub bar_index: usize,  // 位于物品栏中的索引
}
