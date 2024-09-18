use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod player;
pub mod camera;
pub mod cube;

// 通用组件
pub mod basic;
pub mod game_map;

use player::PlayerPlugin;
use camera::CameraPlugin;
use cube::CubePlugin;


fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        // 物理引擎
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // 其他
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CubePlugin)
        .run();
}
