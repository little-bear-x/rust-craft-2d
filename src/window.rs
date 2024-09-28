use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use super::basic::*;

const CUSTOM_CURSOR_SIZE: f32 = 30.0;
const CURSOR_SENSITIVITY: f32 = 0.75;  // 鼠标灵敏度
static mut SHOWING_MOUSE: bool = false;  // 鼠标是否呼出

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
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.title = "rust mine 2d".to_string();
    // 创建鼠标初始位置
    commands.spawn(SpriteBundle {
        texture: assets_server.load("cursor.png"),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        sprite: Sprite {
            custom_size: Some(Vec2::new(CUSTOM_CURSOR_SIZE, CUSTOM_CURSOR_SIZE)),
            ..Default::default()
        },
        ..default()
    }).insert(CursorCom);
}

// 更新自定义鼠标样式位置
fn update_mouse(
    mut cursor_sprite_query: Query<&mut Transform, With<CursorCom>>,
    mut events: EventReader<MouseMotion>,
) {
    // tip: 跟随玩家移动的在玩家移动事件当中。
    // 位于player.rs/move_player
    unsafe {
        if SHOWING_MOUSE {
            return;
        }
    }
    for event in events.read() {
        if let Ok(mut transform) = cursor_sprite_query.get_single_mut() {
            // 更新Sprite的位置，使其与鼠标位置一致
            // println!("{}", event.delta.x);
            // println!("{}", event.delta.y);
            transform.translation.x = transform.translation.x + (event.delta.x) * CURSOR_SENSITIVITY;
            transform.translation.y = transform.translation.y - (event.delta.y) * CURSOR_SENSITIVITY;
            transform.translation.z = 1.0; // 确保Sprite在最顶层
        }
    }
}

fn show_mouse(
    mut window: Query<&mut Window>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let mut window = window.single_mut();
        unsafe{
            if SHOWING_MOUSE {
                window.cursor.visible = false;
                window.cursor.grab_mode = CursorGrabMode::Locked;
            } else {
                window.cursor.visible = true;
                window.cursor.grab_mode = CursorGrabMode::None;
            }
            SHOWING_MOUSE = !SHOWING_MOUSE;
        }
    }
}