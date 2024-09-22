use std::collections::HashMap;

// 方块文件，用于处理方块逻辑
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::basic::*;
use super::game_map::*;

const LOAD_MAP_SIZE: i32 = 7;  // 地图注册大小，在调试时默认为4
const SPAWN_MAP_SIZE: i32 = 6;  // 地图渲染大小，调试时默认为3
const LOAD_MAT_TIME: f32 = 1.0;  // 地图加载检测时间，默认1

#[derive(Resource)]
struct RegCubeCheck(Timer);  // 进行注册方块检测间隔计时器

#[derive(Resource)]
struct SpawnCubeCheck(Timer);  // 进行方块绘制间隔计时器

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
            .add_systems(Startup, spawn_cube_start)
            .add_systems(Update, reg_cube)
            .add_systems(Update, spawn_cube)
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

                    // // 生成方块
                    // commands
                    //     .spawn(CubeBundle{
                    //         cube_type: Cube::GrassCube,
                    //         model: SpriteBundle{  // 模型
                    //             texture: assets_server.load(get_cube_model(&Cube::GrassCube)),
                    //             sprite: Sprite {
                    //                 custom_size: Some(Vec2::new(100., 100.)),
                    //                 ..Default::default()
                    //             },
                    //             transform: Transform {
                    //                 translation: Vec3::new(((((player.translation.x/100.0).round()) as i32 + i) * 100) as f32, -200., 0.),
                    //                 ..Default::default()
                    //             },
                    //             ..Default::default()
                    //         }
                    //     })
                    //     // 物理引擎
                    //     .insert(RigidBody::KinematicPositionBased)
                    //     .insert(Velocity {
                    //         linvel: Vec2::new(0., 0.),
                    //         angvel: 0.2,
                    //     })
                    //     .insert(GravityScale(GRAVITY))
                    //     .insert(Sleeping::disabled())
                    //     .insert(Ccd::enabled())
                    //     .insert(AdditionalMassProperties::Mass(1.0))
                    //     .insert(Collider::cuboid(50., 50.))
                    // ;
                }
            }
        }
    }
}

// 通过表生成/移除方块
fn spawn_cube(
    time: Res<Time>,
    mut timer: ResMut<SpawnCubeCheck>,
    mut commands:Commands,
    player_query: Query<(&Transform, &PlayerMapInfo), With<Player>>,
    assets_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (player, player_map_info) in player_query.iter() {
            for x in (((player.translation.x/100.0).round()) as i32) - SPAWN_MAP_SIZE
                ..(((player.translation.x/100.0).round()) as i32) + SPAWN_MAP_SIZE+1 {
                for y in (((player.translation.y/100.0).round()) as i32) - SPAWN_MAP_SIZE
                    ..(((player.translation.y/100.0).round()) as i32) + SPAWN_MAP_SIZE+1 {
                        if player_map_info.map_hashmap.contains_key(&x) {
                            // info!("1");
                            if match player_map_info.map_hashmap.get(&x).cloned() {
                                Some(s) => {
                                    info!("keyin: {}, y: {}", s.contains_key(&y), y);
                                    s.contains_key(&y)
                                }
                                None => {
                                    error!("出现错误\ncube.rs引起, 位于fn spawn_cube中\n一个完全意外的错误!!!");
                                    false
                                }
                            } {
                                info!("2");
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
            // for (xk, xv) in &player_map_info.map_hashmap {
            //     if ((((player.translation.x/100.0).round()) as i32) - xk).abs() <= LOAD_MAP_SIZE {
            //         info!("1");
            //         for (yk, yv) in xv {
            //             if ((((player.translation.y/100.0).round()) as i32) - yk).abs() <= 2 {
            //                 info!("2");
            //                 // 生成方块
            //                 commands
            //                     .spawn(CubeBundle{
            //                         cube_type: yv.clone(),
            //                         model: SpriteBundle{  // 模型
            //                             texture: assets_server.load(get_cube_model(yv)),
            //                             sprite: Sprite {
            //                                 custom_size: Some(Vec2::new(100., 100.)),
            //                                 ..Default::default()
            //                             },
            //                             transform: Transform {
            //                                 translation: Vec3::new((xk * 100) as f32, (yk * 100) as f32, 0.),
            //                                 ..Default::default()
            //                             },
            //                             ..Default::default()
            //                         }
            //                     })
            //                     // 物理引擎
            //                     .insert(RigidBody::KinematicPositionBased)
            //                     .insert(Velocity {
            //                         linvel: Vec2::new(0., 0.),
            //                         angvel: 0.2,
            //                     })
            //                     .insert(GravityScale(GRAVITY))
            //                     .insert(Sleeping::disabled())
            //                     .insert(Ccd::enabled())
            //                     .insert(AdditionalMassProperties::Mass(1.0))
            //                     .insert(Collider::cuboid(50., 50.))
            //                 ;
            //             }
            //         }
            //     }
            // }
        }
    }
}
