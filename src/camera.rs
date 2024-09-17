use bevy::prelude::*;
use super::basic::*;



pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, update_camera_pos);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(CameraCom)
    .insert(Camera2dBundle::default())
    ;
}

fn update_camera_pos(
    mut camera_q: Query<&mut Transform, With<CameraCom>>,
    player_q: Query<(&Transform, &Movement), Without<CameraCom>>,
) {
    for (
        player_transform,
        movement,
    ) in player_q.iter() { 
        match movement.move_type {  // 判断是否是玩家
            GameObjType::Player => {
                for mut camera_transform in camera_q.iter_mut() {
                    // 更新相机的位置
                    camera_transform.translation = player_transform.translation;  // 将相机位置设置为玩家位置
                }
            }
            _ => {}
        }
    }
}

