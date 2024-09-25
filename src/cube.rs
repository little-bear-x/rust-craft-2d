use std::collections::HashMap;
use std::process::Command;

// 方块文件，用于处理方块逻辑
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::basic::*;
use super::game_map::*;

const LOAD_MAP_SIZE: i32 = 7;  // 地图注册大小，在调试时默认为4
const SPAWN_MAP_SIZE: i32 = 6;  // 地图渲染大小，调试时默认为3
const LOAD_MAT_TIME: f32 = 1.0;  // 地图加载检测时间，默认1

// 定义系统标签
struct RemoveCubeLabel;

#[derive(Resource)]
struct RegCubeCheck(Timer);  // 进行注册方块检测间隔计时器

#[derive(Resource)]
struct SpawnCubeCheck(Timer);  // 进行方块绘制间隔计时器
#[derive(Resource)]
struct RemoveCubeCheck(Timer);  // 进行方块移除间隔计时器

// 方块Bundle
#[derive(Bundle)]
struct CubeBundle {
    pub cube_type: Cube,  // 方块类型
    pub model: SpriteBundle,  // 方块模型
}

pub struct CubePlugin;
impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpawnCubeCheck(Timer::from_seconds(LOAD_MAT_TIME, TimerMode::Repeating)))
            .insert_resource(RegCubeCheck(Timer::from_seconds(LOAD_MAT_TIME, TimerMode::Repeating)))
            .insert_resource(RemoveCubeCheck(Timer::from_seconds(LOAD_MAT_TIME, TimerMode::Repeating)))
            .add_systems(Startup, spawn_cube_start)
            .add_systems(Update, reg_cube)
            // .add_systems(Update, (remove_cube, spawn_cube))
            .add_systems(Update, remove_cube)
            .add_systems(Update, spawn_cube.after(remove_cube))
            
        ;
    }
}

// 初始方块生成
fn spawn_cube_start(
    mut commands:Commands,
    assets_server: Res<AssetServer>,
    mut player_query: Query<&mut PlayerMapInfo, With<Player>>,
) {
    // 设置玩家地图
    for mut player_map_info in player_query.iter_mut(){
        let mut y_hash_map = HashMap::new();  // map_hashmap的第二级哈希表
        y_hash_map.insert(-2, Cube::GrassCube);
        player_map_info.map_hashmap.insert(0, y_hash_map);
    }

    // 生成方块
    commands
        .spawn(CubeBundle{
            cube_type: Cube::GrassCube,
            model: SpriteBundle{  // 模型
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
            }
        })  // 创建方块
        // 物理引擎
        .insert(PhysicsBundle{
            body: RigidBody::KinematicPositionBased,
            velocity: Velocity {
                linvel: Vec2::new(0., 0.),
                angvel: 0.2,
            },
            gravity_scale: GravityScale(GRAVITY),
            sleeping: Sleeping::disabled(),
            ccd: Ccd::enabled(),
            mass: AdditionalMassProperties::Mass(1.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::cuboid(50., 50.),
        })
    ;
}

// 根据玩家位置在玩家地图中注册方块
fn reg_cube(
    time: Res<Time>,
    mut timer: ResMut<RegCubeCheck>,
    // mut commands:Commands,
    mut player_query: Query<(&Transform, &mut PlayerMapInfo), With<Player>>,
    // assets_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (player, mut player_map_info) in player_query.iter_mut(){
            for i in -LOAD_MAP_SIZE..LOAD_MAP_SIZE + 1 {
                if ! player_map_info.map_hashmap.contains_key(&(((player.translation.x/100.0).round()) as i32 + i)) {
                    // 设置玩家地图
                    let mut y_hash_map: HashMap<i32, Cube> = HashMap::new();  // map_hashmap的第二级哈希表
                    y_hash_map.insert(-2, Cube::GrassCube);
                    player_map_info.map_hashmap.insert(((player.translation.x/100.0).round()) as i32 + i, y_hash_map);
                }
            }
        }
    }
}

// 通过表生成方块
fn spawn_cube(
    time: Res<Time>,
    mut timer: ResMut<SpawnCubeCheck>,
    mut commands:Commands,
    player_query: Query<(&Transform, &PlayerMapInfo), With<Player>>,
    assets_server: Res<AssetServer>,
) {

    if timer.0.tick(time.delta()).just_finished() {
        info!("b");
        for (player, player_map_info) in player_query.iter() {
            for x in (((player.translation.x/100.0).round()) as i32) - SPAWN_MAP_SIZE
                ..(((player.translation.x/100.0).round()) as i32) + SPAWN_MAP_SIZE+1 {
                for y in (((player.translation.y/100.0).round()) as i32) - SPAWN_MAP_SIZE
                    ..(((player.translation.y/100.0).round()) as i32) + SPAWN_MAP_SIZE+1 {
                    // 判断是否等于玩家脚下的方块，避免重复创建
                    // 我知道这段代码写的很乱，但是目前没有想到别的解决方法了。
                    if (((player.translation.x/100.0).round()) as i32) != x && 
                        (((player.translation.y/100.0).round()) as i32) != y+1 {
                        // 判断是否存在x
                        if player_map_info.map_hashmap.contains_key(&x) {
                            // 判断是否存在y
                            if match player_map_info.map_hashmap.get(&x).cloned() {
                                Some(s) => {
                                    s.contains_key(&y)
                                }
                                None => {
                                    error!("出现错误\ncube.rs引起, 位于fn spawn_cube中\n一个完全意外的错误!!!");
                                    false
                                }
                            } {
                                // 获取cube的值
                                let cube = match player_map_info.map_hashmap.get(&x).cloned() {
                                    Some(ys) => {
                                        match ys.get(&y).cloned() {
                                            Some(cu) => cu,
                                            None => {
                                                error!("出现错误\ncube.rs引起, 位于fn spawn_cube中\n一个完全意外的错误!!!\n已默认使用草方块代替");
                                                Cube::GrassCube
                                            }
                                        }
                                    }
                                    None => {
                                        error!("出现错误\ncube.rs引起, 位于fn spawn_cube中\n一个完全意外的错误!!!\n已默认使用草方块代替");
                                        Cube::GrassCube
                                    }
                                };
                                // 绘制方块
                                commands
                                    .spawn(CubeBundle{
                                        cube_type: cube.clone(),
                                        model: SpriteBundle{  // 模型
                                            texture: assets_server.load(get_cube_model(&cube)),
                                            sprite: Sprite {
                                                custom_size: Some(Vec2::new(100., 100.)),
                                                ..Default::default()
                                            },
                                            transform: Transform {
                                                translation: Vec3::new((x * 100) as f32, (y * 100) as f32, 0.),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        }
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
        }
    }
}

// 删除除了玩家脚下的方块（方块没帧更新）
fn remove_cube(
    time: Res<Time>,
    mut timer: ResMut<RemoveCubeCheck>,
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    cube_query: Query<(Entity, &Transform), With<Cube>>,
){
    if timer.0.tick(time.delta()).just_finished() {
        info!("a");
        for player_transform in player_query.iter(){
            for (entity, cube_transform) in cube_query.iter() {
                if (player_transform.translation.x / 100.0).ceil() != (cube_transform.translation.x /100.).ceil() && 
                    (player_transform.translation.y / 100.0).ceil() - 1.0 != (cube_transform.translation.y /100.).ceil() {
                    commands.entity(entity).despawn();
                } 
            }
        }
    }
}
