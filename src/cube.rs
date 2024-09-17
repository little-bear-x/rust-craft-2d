// 方块文件，用于处理方块逻辑
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::basic::*;

const CUBE_POS: Vec2 = Vec2::new(0., -200.);

#[derive(Bundle)]
struct CubeBundle {
    pub model: SpriteBundle,
}

pub struct CubePlugin;
impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cube);
    }
}

// 创建玩家
fn spawn_cube(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn(Cube)  // 床架方块
        .insert(SpriteBundle{  // 模型
            texture: assets_server.load("cube/grass.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(CUBE_POS.x, CUBE_POS.y, 0.),
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
