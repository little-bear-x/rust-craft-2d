use bevy::{input::mouse::MouseMotion, prelude::*, window::{CursorGrabMode, WindowMode, PresentMode}};
use super::basic::*;

const CUSTOM_CURSOR_SIZE: f32 = 30.0;
const CURSOR_SENSITIVITY: f32 = 0.75;  // 鼠标灵敏度
static mut CURSOR_TO_PLAYER: (f32, f32) = (0.0, 0.0);  // 鼠标与玩家距离

pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_window);
        app.add_systems(Update, update_mouse);
        app.add_systems(Update, show_mouse);
    }
}

fn setup_window(
    mut window: Query<&mut Window>, 
    mut commands: Commands,
    assets_server: Res<AssetServer>,
) {
    let mut window = window.single_mut();
    window.mode = WindowMode::Fullscreen;
    window.cursor.visible = false;
    // window.resizable = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
    // window.mode = WindowMode::Fullscreen;
    window.title = "rust craft 2d".to_string();
    window.present_mode = PresentMode::AutoVsync;  // 开启垂直同步
    // 创建鼠标初始位置
    commands.spawn(SpriteBundle {
        texture: assets_server.load("cursor.png"),
        transform: Transform::from_translation(Vec3::new(0., 0., 99.0)),
        sprite: Sprite {
            custom_size: Some(Vec2::new(CUSTOM_CURSOR_SIZE, CUSTOM_CURSOR_SIZE)),
            ..Default::default()
        },
        ..default()
    }).insert(CursorCom);
}

// 更新自定义鼠标样式位置
fn update_mouse(
    mut cursor_sprite_query: Query<&mut Transform, (With<CursorCom>, Without<CameraCom>)>,
    mut events: EventReader<MouseMotion>,
    camera_query: Query<&Transform, (With<CameraCom>, Without<CursorCom>)>,
    player_info: Res<PlayerInfo>,
) {
    if !player_info.is_controlling {
        return;
    }

    let camera = camera_query.single();
    for event in events.read() {
        // 更新Sprite的位置，使其与鼠标位置一致
        // println!("{}", event.delta.x);
        // println!("{}", event.delta.y);
        unsafe {
            CURSOR_TO_PLAYER = (CURSOR_TO_PLAYER.0 + event.delta.x * CURSOR_SENSITIVITY, 
                CURSOR_TO_PLAYER.1 + event.delta.y * CURSOR_SENSITIVITY);
        }
    }
    if let Ok(mut transform) = cursor_sprite_query.get_single_mut() {
        unsafe {
            transform.translation.x = camera.translation.x + CURSOR_TO_PLAYER.0;
            transform.translation.y = camera.translation.y - CURSOR_TO_PLAYER.1;
            transform.translation.z = 99.0; // 确保Sprite在最顶层
        }
    }
}

fn show_mouse(
    mut window: Query<&mut Window>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_info: ResMut<PlayerInfo>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let mut window = window.single_mut();

        if !player_info.is_controlling {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        } else {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        }
        player_info.is_controlling = !player_info.is_controlling;
    }
}