use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod player;
pub mod camera;
pub mod cube;
pub mod debug;
pub mod window;

// 通用组件
pub mod basic;
pub mod gameui;

use player::PlayerPlugin;
use camera::CameraPlugin;
use cube::CubePlugin;
// use debug::DebugPlugin;
use window::WindowPlugin;
use gameui::GameUiPlugin;
use basic::*;

use std::collections::HashMap;
use core::array::from_fn;


fn main(){
    App::new()
        .insert_resource(PlayerInfo{
            player_map: HashMap::new(),
            is_controlling: true,
            is_paused: false,
            player_bar: from_fn(|_| (Option::None, 0)),
            player_bar_select_index: 0
        })

        .add_plugins(DefaultPlugins)
        // 物理引擎
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())  // 物理引擎调试
        // 其他
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CubePlugin)
        // 调试
        // .add_plugins(DebugPlugin)
        // 窗口
        .add_plugins(WindowPlugin)
        .add_plugins(GameUiPlugin)
        .run();
}
