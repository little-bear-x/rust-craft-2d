// 玩家文件，用于处理玩家逻辑
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::basic::*;

const STARTING_PLAYER_POS: Vec2 = Vec2::new(0., 0.);


#[derive(Bundle)]
struct PlayerBundle {
    pub vel: Movement,
    pub model: SpriteBundle,
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
        .spawn(Player)  // 创建一个玩家
        .insert(SpriteBundle{  // 添加玩家模型
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
        })
        .insert( Movement{  // 添加玩家移动组件
            basic_speed: 5.,
            basic_jump_high: 100.,
            actual_speed: 5.,
            actual_jump_high: 100.,
            move_type: GameObjType::Player,
        })
        .insert(RigidBody::Dynamic)  // 创建玩家刚体
        .insert(Velocity {  // 添加玩家速度
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.,
        })
        .insert(GravityScale(GRAVITY))  // 设置重力
        .insert(Sleeping::disabled())  // 睡眠设置
        .insert(Ccd::enabled())  // ccd设置
        .insert(AdditionalMassProperties::Mass(1.0))  // 重量设置
        .insert(LockedAxes::ROTATION_LOCKED)  // 禁止旋转
        .insert(Collider::cuboid(50., 50.))  // 碰撞器
        ;
}

// 玩家移动\下蹲
fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Sprite, &mut Movement, &mut Velocity), With<Player>>
) {
    for (
        mut transform, 
        mut sprite, 
        mut movement, 
        mut velocities
    ) in query.iter_mut() {
        // 检测移动
        if keys.pressed(KeyCode::KeyA) {  // 向右
            transform.translation.x -= movement.actual_speed;
        } else if keys.pressed(KeyCode::KeyD) {  // 向左
            transform.translation.x += movement.actual_speed;
        }
        // 检测跳跃
        if keys.just_pressed(KeyCode::Space) {  // 跳跃
            velocities.linvel = Vec2::new(0.0, 500.0);
            velocities.angvel = 0.0;
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

