use std::collections::HashMap;

// 方块文件，用于处理方块逻辑
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::basic::*;

const LOAD_MAP_SIZE: i32 = 10;  // 地图注册大小，在调试时默认为4
const SPAWN_MAP_SIZE: i32 = 8;  // 地图渲染大小，调试时默认为3
const LOAD_MAT_TIME: f32 = 0.1;  // 地图加载检测时间，默认1

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
            // .add_systems(Startup, spawn_cube_start)
            .add_systems(Update, reg_cube)
            // .add_systems(Update, (remove_cube, spawn_cube))
            .add_systems(Update, remove_cube)
            .add_systems(Update, spawn_cube.after(remove_cube))
            .add_systems(Update, player_cube)
        ;
    }
}

// // 初始方块生成
// fn spawn_cube_start(
//     mut commands:Commands,
//     assets_server: Res<AssetServer>,
//     mut player_info: ResMut<PlayerInfo>
// ) {
//     let mut y_hash_map = HashMap::new();  // map_hashmap的第二级哈希表
//     y_hash_map.insert(-2, Cube::GrassCube);
//     player_info.player_map.insert(0, y_hash_map);

//     // 生成方块
//     commands
//         .spawn(CubeBundle{
//             cube_type: Cube::GrassCube,
//             model: SpriteBundle{  // 模型
//                 texture: assets_server.load("cube/grass.png"),
//                 sprite: Sprite {
//                     custom_size: Some(Vec2::new(100., 100.)),
//                     ..Default::default()
//                 },
//                 transform: Transform {
//                     translation: Vec3::new(0., 0., 0.),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             }
//         })  // 创建方块
//         // 物理引擎
//         .insert(PhysicsBundle{
//             body: RigidBody::KinematicPositionBased,
//             velocity: Velocity {
//                 linvel: Vec2::new(0., 0.),
//                 angvel: 0.2,
//             },
//             gravity_scale: GravityScale(GRAVITY),
//             sleeping: Sleeping::disabled(),
//             ccd: Ccd::enabled(),
//             mass: AdditionalMassProperties::Mass(1.0),
//             locked_axes: LockedAxes::ROTATION_LOCKED,
//             collider: Collider::cuboid(50., 50.),
//         })
//     ;
// }

// 根据玩家位置在玩家地图中注册方块
fn reg_cube(
    time: Res<Time>,
    mut timer: ResMut<RegCubeCheck>,
    // mut commands:Commands,
    mut player_query: Query<&Transform, With<Player>>,
    mut player_info: ResMut<PlayerInfo>
    // assets_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for player in player_query.iter_mut(){
            for i in -LOAD_MAP_SIZE..LOAD_MAP_SIZE + 1 {
                if ! player_info.player_map.contains_key(&(((player.translation.x/100.0).round()) as i32 + i)) {
                    // 设置玩家地图
                    // player_map的第二级哈希表
                    let mut y_hash_map: HashMap<i32, Cube> = HashMap::new(); 
                    for j in -63..-1{
                        y_hash_map.insert(j, Cube::StoneCube);
                    }
                    y_hash_map.insert(-1, Cube::SoilCube);
                    y_hash_map.insert(0, Cube::GrassCube);
                    player_info.player_map.insert(((player.translation.x/100.0).round()) as i32 + i, y_hash_map);
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
    player_query: Query<&Transform, With<Player>>,
    assets_server: Res<AssetServer>,
    player_info: ResMut<PlayerInfo>
) {

    if timer.0.tick(time.delta()).just_finished() {
        for player in player_query.iter() {
            for x in (((player.translation.x/100.0).round()) as i32) - SPAWN_MAP_SIZE
                ..(((player.translation.x/100.0).round()) as i32) + SPAWN_MAP_SIZE+1 {
                for y in (((player.translation.y/100.0).round()) as i32) - SPAWN_MAP_SIZE - 1
                    ..(((player.translation.y/100.0).round()) as i32) + SPAWN_MAP_SIZE {
                    // 判断是否存在x
                    if player_info.player_map.contains_key(&x) {
                        // 判断是否存在y
                        if match player_info.player_map.get(&x).cloned() {
                            Some(s) => {
                                s.contains_key(&y)
                            }
                            None => {
                                error!("出现错误\ncube.rs引起, 位于fn spawn_cube中\n一个完全意外的错误!!!");
                                false
                            }
                        } {
                            // 获取cube的值
                            let cube = match player_info.player_map.get(&x).cloned() {
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

// 删除除了玩家脚下的方块
fn remove_cube(
    time: Res<Time>,
    mut timer: ResMut<RemoveCubeCheck>,
    mut commands: Commands,
    cube_query: Query<Entity, With<Cube>>,
){
    if timer.0.tick(time.delta()).just_finished() {
        // info!("a");
        for entity in cube_query.iter() {
                commands.entity(entity).despawn();
        }
    }
}

// 玩家操作方块
fn player_cube(
    mut commands: Commands,
    cube_query: Query<(Entity, &Transform), With<Cube>>,
    cursor_sprite_query: Query<&Transform, (With<CursorCom>, Without<Cube>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    assets_server: Res<AssetServer>,
    mut player_info: ResMut<PlayerInfo>
) {
    // 确保仅在控制时候操作
    if player_info.is_controlling{
        // 删除方块
        if buttons.pressed(MouseButton::Left) {
            let cursor_pos: (f32, f32) = ((cursor_sprite_query.get_single().unwrap().translation.x/100.).round(),
                (cursor_sprite_query.get_single().unwrap().translation.y/100.).round());

            // 检查是否存在x坐标
            if player_info.player_map.contains_key(&(cursor_pos.0 as i32)) {
                match player_info.player_map.get(&(cursor_pos.0 as i32)) {
                    Some(s) => {
                        // 检查是否存在y坐标
                        if s.contains_key(&(cursor_pos.1 as i32)) {
                            // 存在则删除
                            for (entity, cube_transform) in cube_query.iter() {
                                if cube_transform.translation.x == cursor_pos.0 * 100. && cube_transform.translation.y == cursor_pos.1 * 100. {
                                    commands.entity(entity).despawn();
                                    player_info.player_map.get_mut(&(cursor_pos.0 as i32)).unwrap().remove(&(cursor_pos.1 as i32));
                                }
                            }
                        }
                    }
                    None => {
                        error!("出现错误\n由player.rs引起, 位于fn player_cube\n一个完全意外的错误!!!")
                    }
                }
            }
        } else if buttons.pressed(MouseButton::Right) {  // 放置方块
            let cursor_pos: (f32, f32) = ((cursor_sprite_query.get_single().unwrap().translation.x/100.).round(),
                (cursor_sprite_query.get_single().unwrap().translation.y/100.).round());

            // 获取y字典
            let mut map_y = match player_info.player_map.get_mut(&(cursor_pos.0 as i32)) {
                Some(s) => {
                    if s.contains_key(&(cursor_pos.1 as i32)) {
                        return;
                    }
                    s.clone()
                },
                None => {
                    let y_hash_map: HashMap<i32, Cube> = HashMap::new();
                    y_hash_map.clone()
                },
            };
            // 放置方块的类型
            let cube_type = match player_info.player_bar[player_info.player_bar_select_index].clone() {
                Some(s) => {
                    match s {
                        GameObjType::Cube(cube) => {
                            cube
                        }
                        _ => { return; }
                    }
                },
                None => {
                    return;
                }
            };

            map_y.insert(cursor_pos.1 as i32, cube_type.clone());
            player_info.player_map.insert(cursor_pos.0 as i32, map_y);

            // 在此处渲染方块
            commands
                .spawn(CubeBundle{
                    cube_type: cube_type.clone(),
                    model: SpriteBundle{  // 模型
                        texture: assets_server.load(get_cube_model(&cube_type)),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(100., 100.)),
                            ..Default::default()
                        },
                        transform: Transform {
                            translation: Vec3::new(cursor_pos.0 * 100., cursor_pos.1 * 100., 0.),
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
    }
}
