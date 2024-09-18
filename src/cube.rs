// 方块文件，用于处理方块逻辑
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::basic::*;
use super::game_map::*;

const LOAD_MAP_SIZE: i32 = 4;  // 地图加载大小，在调试时默认为4
const LOAD_MAT_TIME: f32 = 1.0;  // 地图加载检测时间，默认1
const CUBE_POS: Vec2 = Vec2::new(0., -200.);

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
) {
    for i in 0..1 {
        // info!("player: {}", (((player.translation.x/100.0).round()) as i32 + i));
        // info!("cube: {}", (((player.translation.x/100.0).round()) as i32 + i));
        commands
            .spawn(Cube)  // 创建方块
            .insert(SpriteBundle{  // 模型
                texture: assets_server.load("cube/grass.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100., 100.)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new((i * 100) as f32, -200., 0.),
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

// 根据玩家位置创建方块
fn spawn_cube(
    time: Res<Time>,
    mut timer: ResMut<SpawnCubeCheck>,
    mut commands:Commands,
    mut player_query: Query<(&Transform, &mut PlayerMapInfo), With<Player>>,
    assets_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        info!("check spawn cube");
        for (player, mut player_map_info) in player_query.iter_mut(){
            for i in -LOAD_MAP_SIZE..LOAD_MAP_SIZE + 1 {
                if ! player_map_info.map_x_vector.contains(&(((player.translation.x/100.0).round()) as i32 + i)) {
                    // info!("player: {}", (((player.translation.x/100.0).round()) as i32 + i));
                    // info!("cube: {}", (((player.translation.x/100.0).round()) as i32 + i));
                    player_map_info.map_x_vector.push((player.translation.x.round()) as i32 + i);
                    commands
                        .spawn(Cube)  // 创建方块
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
