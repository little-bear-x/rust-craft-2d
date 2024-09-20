// 游戏地图系统
use std::collections::HashMap;
use bevy::prelude::Component;
use super::basic::*;

// 地图基础信息
#[derive(Component)]
pub struct PlayerMapInfo {  // 玩家地图的基础信息
    pub map_x_vector: Vec<i32>,  // 玩家一探搜的x轴表
    pub map_hashmap: HashMap<i32, HashMap<i32, Cube>>  // 玩家地图哈希表
}
