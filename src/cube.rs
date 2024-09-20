use std::collections::HashMap;

// 方块文件，用于处理方块逻辑
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::basic::*;
use super::game_map::*;

const LOAD_MAP_SIZE: i32 = 4;  // 地图加载大小，在调试时默认为4
const LOAD_MAT_TIME: f32 = 1.0;  // 地图加载检测时间，默认1

#[derive(Resource)]
struct SpawnCubeCheck(Timer);  // 一秒一次进行生成方块检测

pub struct CubePlugin;
impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpawnCubeCheck(Timer::from_seconds(LOAD_MAT_TIME, TimerMode::Repeating)))
            .add_systems(Startup, spawn_cube_start)
            .add_systems(Update, spawn_cube)
        ;
    }
}
fn spawn_cube_start(
    mut commands:Commands,
    assets_server: Res<AssetServer>,
    mut player_query: Query<&mut PlayerMapInfo, With<Player>>,
) {
    // 设置玩家地图
    for mut player_map_info in player_query.iter_mut(){   
        player_map_info.map_x_vector.push(0);
        let mut y_hash_map = HashMap::new();  // map_hashmap的第二级哈希表
        y_hash_map.insert(-2, Cube::GrassCube);
        player_map_info.map_hashmap.insert(0, y_hash_map);
    }

    // 生成方块
    commands
        .spawn(CubeBundle{
            cube_type: Cube::GrassCube
        })  // 创建方块
        .insert(SpriteBundle{  // 模型
            texture: assets_server.load("cube/grass.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., -200., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        // 物理引擎
        .insert(RigidBody::KinematicPositionBased)
        .insert(Velocity {
            linvel: Vec2::new(0., 0.),
            angvel: 0.2,
        })
        .insert(GravityScale(GRAVITY))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(AdditionalMassProperties::Mass(1.0))
        .insert(Collider::cuboid(50., 50.))
    ;
}

// 根据玩家位置创建方块
fn spawn_cube(
    time: Res<Time>,
    mut timer: ResMut<SpawnCubeCheck>,
    mut commands:Commands,
    mut player_query: Query<(&Transform, &mut PlayerMapInfo), With<Player>>,
    assets_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (player, mut player_map_info) in player_query.iter_mut(){
            for i in -LOAD_MAP_SIZE..LOAD_MAP_SIZE + 1 {
                if ! player_map_info.map_x_vector.contains(&(((player.translation.x/100.0).round()) as i32 + i)) {
                    // 设置玩家地图
                    player_map_info.map_x_vector.push(((player.translation.x/100.0).round()) as i32 + i);
                    let mut y_hash_map = HashMap::new();  // map_hashmap的第二级哈希表
                    y_hash_map.insert(-2, Cube::GrassCube);
                    player_map_info.map_hashmap.insert(((player.translation.x/100.0).round()) as i32 + i, y_hash_map);

                    // 生成方块
                    commands
                        .spawn(CubeBundle{
                            cube_type: Cube::GrassCube
                        })  // 创建方块
                        .insert(SpriteBundle{  // 模型
                            texture: assets_server.load("cube/grass.png"),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(100., 100.)),
                                ..Default::default()
                            },
                            transform: Transform {
                                translation: Vec3::new(((((player.translation.x/100.0).round()) as i32 + i) * 100) as f32, -200., 0.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        // 物理引擎
                        .insert(RigidBody::KinematicPositionBased)
                        .insert(Velocity {
                            linvel: Vec2::new(0., 0.),
                            angvel: 0.2,
                        })
                        .insert(GravityScale(GRAVITY))
                        .insert(Sleeping::disabled())
                        .insert(Ccd::enabled())
                        .insert(AdditionalMassProperties::Mass(1.0))
                        .insert(Collider::cuboid(50., 50.))
                    ;
                }
            }
        }
    }
}
