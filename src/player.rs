// 玩家文件，用于处理玩家逻辑
use bevy::prelude::*;
use std::collections::HashMap;
use bevy_rapier2d::prelude::*;
use super::basic::*;
use super::game_map::*;

const STARTING_PLAYER_POS: Vec2 = Vec2::new(1., 400.);


#[derive(Bundle)]
struct PlayerBundle {
    pub player: Player,  // 玩家基本信息
    pub movement: Movement,  // 玩家移动组件
    pub player_map: PlayerMapInfo,  // 玩家地图
    pub model: SpriteBundle,  // 玩家模型
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, move_player);
    }
}

// 创建玩家
fn spawn_player(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        // 添加玩家
        .spawn(PlayerBundle{
            player: Player,
            movement: Movement{
                basic_speed: 5.,
                basic_jump_high: 100.,
                actual_speed: 5.,
                actual_jump_high: 100.,
                move_type: GameObjType::Player,
            },
            player_map: PlayerMapInfo{
                map_hashmap: HashMap::new()
            },
            model: SpriteBundle{
                texture: assets_server.load("player.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100., 100.)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(STARTING_PLAYER_POS.x, STARTING_PLAYER_POS.y, 0.),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        // 物理引擎
        .insert(PhysicsBundle{
            body: RigidBody::Dynamic,
            velocity: Velocity {  // 添加玩家速度
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.,
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

// 玩家移动\下蹲
fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Sprite, &mut Movement, &mut Velocity, &PlayerMapInfo), With<Player>>
) {
    for (
        mut transform, 
        mut sprite, 
        mut movement, 
        mut velocities,
        player_map
    ) in query.iter_mut() {
        // 检测移动
        if keys.pressed(KeyCode::KeyA) {  // 向右
            transform.translation.x -= movement.actual_speed;
        } else if keys.pressed(KeyCode::KeyD) {  // 向左
            transform.translation.x += movement.actual_speed;
        }
        // 检测跳跃
        if keys.just_pressed(KeyCode::Space) {  // 跳跃
            if player_map.map_hashmap.contains_key(&(((transform.translation.x/100.0).round()) as i32)){
                match player_map.map_hashmap.get(&(((transform.translation.x/100.0).round()) as i32)) {
                    Some(s) => {
                        if s.contains_key(&(((transform.translation.y/100.0).ceil()) as i32 - 1)) {
                            velocities.linvel = Vec2::new(0.0, 500.0);
                            velocities.angvel = 0.0;
                        }
                    }
                    _  => {
                        error!("出现错误\n由player.rs引起, 位于fn move_player -> jump_check中\n一个完全意外的错误!!!")
                    }
                }

            }
        } 
        // 检测下蹲
        if keys.just_pressed(KeyCode::ShiftLeft) {  // 下蹲
            sprite.custom_size = Some(Vec2::new(100., 75.));
            transform.translation.y -= 22.5;
            movement.actual_speed = movement.basic_speed * SQUATTING_SPEED_INCREASES_MULT;
        } else if keys.just_released(KeyCode::ShiftLeft) {  // 取消下蹲
            sprite.custom_size = Some(Vec2::new(100., 100.));
            transform.translation.y += 22.5;
            movement.actual_speed = movement.actual_speed / SQUATTING_SPEED_INCREASES_MULT;
        }
    }
}

