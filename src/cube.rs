use std::collections::HashMap;
// 方块文件，用于处理方块逻辑
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


use super::basic::*;

use noise::{NoiseFn, Perlin};
use once_cell::sync::Lazy;
use std::sync::Mutex;


static PERLIN: Lazy<Mutex<Perlin>> = Lazy::new(|| {
    Mutex::new(Perlin::new(0))
});

const SPAWN_MAP_SIZE: i32 = 8;  // 地图渲染大小，调试时默认为3
pub const LOAD_MAP_SIZE: i32 = 12;
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
            .add_systems(Startup, init_perlin)
        ;
    }
}

fn init_perlin(player_info: ResMut<PlayerInfo>) {
    let seed = if player_info.player_map_seed < 0 {
        0
    } else {
        player_info.player_map_seed as u32
    };
    println!("[cube.rs/init_perlin]info: Seed: {}", seed);
    *PERLIN.lock().expect("[cube.rs/init_perlin]panic: Cannot lock perlin mutex") = Perlin::new(seed);
}


fn generate_noise(x: f64, octaves: usize, persistence: f64, lacunarity: f64) -> i32 {
    let mut total = 0.0;
    let mut max_value = 0.0; // 用于归一化
    let mut frequency = 1.0;
    let mut amplitude = 1.0;

    for _ in 0..octaves {
        let x_scaled = x * frequency;
        let perlin = PERLIN.lock().expect("[cube.rs/generate_noise]panic: Cannot lock perlin mutex");
        total += perlin.get([x_scaled, 0.0]) * amplitude;
        max_value += amplitude;
        frequency *= lacunarity;
        amplitude *= persistence;
    }

    // 归一化并缩放
    let normalized = total / max_value;
    (normalized * 30.0).round() as i32
}

// 根据玩家位置在玩家地图中注册方块, 与地形生成算法（value noise）
fn reg_cube(
    time: Res<Time>,
    mut timer: ResMut<RegCubeCheck>,
    // mut commands:Commands,
    mut player_query: Query<&Transform, With<Player>>,
    mut player_info: ResMut<PlayerInfo>
    // assets_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if player_info.player_map_seed < 0 {  // 超平坦地形生成
            for player in player_query.iter_mut(){
                for i in -LOAD_MAP_SIZE..LOAD_MAP_SIZE + 1 {
                    if ! player_info.player_map.contains_key(&(((player.translation.x/100.0).round()) as i32 + i)) {
                        // 设置玩家地图
                        // player_map的第二级哈希表
                        let mut y_hash_map: HashMap<i32, Cube> = HashMap::new(); 
                        for j in -63..-3{
                            y_hash_map.insert(j, Cube::StoneCube);
                        }
                        for j in -3..0 {

                        y_hash_map.insert(j, Cube::SoilCube);
                        }
                        y_hash_map.insert(0, Cube::GrassCube);
                        player_info.player_map.insert(((player.translation.x/100.0).round()) as i32 + i, y_hash_map);
                    }
                }
            }
        } else {
            for player in player_query.iter_mut(){
                for i in -LOAD_MAP_SIZE..LOAD_MAP_SIZE + 1 {
                    let x = ((player.translation.x / 100.0).round() as i32) + i;
                    if ! player_info.player_map.contains_key(&x) {
                        let y = generate_noise((x as f64)/100., 4, 0.5, 2.0);
                        // 设置玩家地图
                        // player_map的第二级哈希表
                        let mut y_hash_map: HashMap<i32, Cube> = HashMap::new(); 
                        for j in -63..y-3{
                            y_hash_map.insert(j, Cube::StoneCube);
                        }
                        for j in y-3..y {
                            y_hash_map.insert(j, Cube::SoilCube);
                        }
                        y_hash_map.insert(y, Cube::GrassCube);
                        player_info.player_map.insert(((player.translation.x/100.0).round()) as i32 + i, y_hash_map);
                    }
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
                                error!("[cube.rs/spawn_cube]error: Unexpected error");
                                false
                            }
                        } {
                            // 获取cube的值
                            let cube = match player_info.player_map.get(&x).cloned() {
                                Some(ys) => {
                                    match ys.get(&y).cloned() {
                                        Some(cu) => cu,
                                        None => {
                                            error!("[cube.rs/spawn_cube]error: Unexpected error");
                                            Cube::GrassCube
                                        }
                                    }
                                }
                                None => {
                                    error!("[cube.rs/spawn_cube]error: Unexpected error");
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
                                .insert(Ccd::disabled())
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
            let cursor_pos: (f32, f32) = ((cursor_sprite_query.get_single().expect("[cube.rs/player_cube]panic: Unexpected error! Unable to obtain mouse position!").translation.x/100.).round(),
                (cursor_sprite_query.get_single().expect("[cube.rs/player_cube]panic: Unexpected error! Unable to obtain mouse position! ").translation.y/100.).round());

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
                                    player_info.player_map.get_mut(&(cursor_pos.0 as i32)).expect("[cube.rs/player_cube]panic: Unexpected error! Unable to delete the specified cube!").remove(&(cursor_pos.1 as i32));
                                }
                            }
                        }
                    }
                    None => {
                        error!("[cube.rs/player_cube]error: Unexpected error")
                    }
                }
            }
        } else if buttons.pressed(MouseButton::Right) {  // 放置方块
            let cursor_pos: (f32, f32) = ((cursor_sprite_query.get_single().expect("[cube.rs/player_cube]panic: Unexpected error! Unable to obtain mouse position! ").translation.x/100.).round(),
                (cursor_sprite_query.get_single().expect("[cube.rs/player_cube]panic: Unexpected error! Unable to obtain mouse position! ").translation.y/100.).round());

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
            let cube_type = match player_info.player_bar[player_info.player_bar_select_index].0.clone() {
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

            // 修改玩家放置后方块的数量
                let i = player_info.player_bar_select_index;
            if player_info.player_bar[player_info.player_bar_select_index.clone()].1 != -1 {
                player_info.player_bar[i].1 -= 1;
            }
            if player_info.player_bar[player_info.player_bar_select_index.clone()].1 == 0 {
                player_info.player_bar[i].0 = None;
            }
        }
    }
}
