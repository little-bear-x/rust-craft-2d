use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
pub enum GameObjType {
    Player,  // 玩家
    Creature,  // 生物
    Cube,  // 方块
}

// 玩家速度
// 玩家速度常量
pub const SQUATTING_SPEED_INCREASES_MULT: f32 = 0.5;  // 蹲下速度增加倍率

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
// 标记为玩家
#[derive(Component)]
pub struct Player;
// 标记为相机
#[derive(Component)]
pub struct CameraCom;
// 标记为方块, 并指定方块类型
#[derive(Component, Debug)]
pub enum Cube {
    GrassCube,  // 草方块
    SoilCube,  // 土方块
    StoneCube,  // 石块
}
// 为cube实现clone trait
impl Clone for Cube {
    fn clone(&self) -> Self {
        match self {
            Cube::GrassCube => Cube::GrassCube,
            Cube::SoilCube => Cube::SoilCube,
            Cube::StoneCube => Cube::StoneCube,
        }
    }
}

// 通过方块类型获取方块模块位置
pub fn get_cube_model(cube_type: &Cube)  -> &'static str {
    match cube_type {
        Cube::GrassCube => {
            "cube/grass.png"
        }
        Cube::SoilCube => {
            "cube/soil.png"
        }
        Cube::StoneCube => {
            "cube/stone.png"
        }
    }
}

