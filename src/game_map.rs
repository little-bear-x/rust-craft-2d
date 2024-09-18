// 游戏地图系统

use bevy::prelude::Component;

// 地图基础信息
#[derive(Component)]
pub struct PlayerMapInfo {  // 玩家地图的基础信息
    pub map_x_vector: Vec<i32>,  // 玩家一探搜的x轴表
}
