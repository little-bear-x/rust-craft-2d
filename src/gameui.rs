use bevy::{input::mouse::{MouseWheel, MouseScrollUnit}, prelude::*, window::PrimaryWindow};
use super::basic::*;

pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_bar);
        app.add_systems(Update, change_select);
    }
}

// 初始化页面
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // 初始化物品栏
    let primary_window = window_query.get_single().unwrap();
    let window_height = primary_window.height();
    let sprite_position_y = -window_height / 2.0 + 150.;
    for i in 0..5 {
        commands.spawn(SpriteBundle {
            texture: asset_server.load("other/item_bar.png"),
            transform: Transform::from_xyz((50*((i as isize)-2)) as f32, sprite_position_y, 9.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 50.)),
                ..Default::default()
            },
            ..default()
        })
        .insert(BarCom{
            bar_index: i,
            bar_item: Option::None,
        });
    }
    // 初始化物品栏选中
    commands.spawn(SpriteBundle {
        texture: asset_server.load("other/item_choose.png"),
        transform: Transform::from_xyz(-100., sprite_position_y, 9.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            ..Default::default()
        },
        ..default()
    }).insert(BarSelectorCom{
        select_index: 0,
    });
}

// 更新bar
fn update_bar(
    mut bar_query: Query<(&mut Transform, &BarCom), (With<BarCom>, Without<CameraCom>, Without<BarSelectorCom>)>,
    mut bar_selector_query: Query<(&mut Transform, &mut BarSelectorCom), (With<BarSelectorCom>, Without<CameraCom>, Without<BarCom>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Transform, (With<CameraCom>, Without<BarCom>, Without<BarSelectorCom>)>
) {
    let camera = camera_query.get_single().unwrap().translation;

    // 更新物品栏位置
    let primary_window = window_query.get_single().unwrap();
    // let window_width = primary_window.width();
    let window_height = primary_window.height();
    // 计算Sprite的位置
    let sprite_position_y = -window_height / 2.0+50.;
    for (mut bar_transform, bar_com) in bar_query.iter_mut() {
        bar_transform.translation.x = camera.x+(50*((bar_com.bar_index as isize)-2)) as f32;
        bar_transform.translation.y = camera.y + sprite_position_y;
    }

    // 更新物品栏选中器位置
    let (mut bar_selector_translation, bar_selector_com) = bar_selector_query.get_single_mut().unwrap();
    bar_selector_translation.translation.x = camera.x+(50*((bar_selector_com.select_index as isize)-2)) as f32;
    bar_selector_translation.translation.y = camera.y + sprite_position_y;
}

// 更改选中物品
fn change_select(
    mut bar_selector_query: Query<&mut BarSelectorCom>,
    mut mouse_scroll: EventReader<MouseWheel>,
) {
    let mut bar_selector_com = bar_selector_query.get_single_mut().unwrap();
    for ev in mouse_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if bar_selector_com.select_index == 4 && ev.y < 0.0 {
                    bar_selector_com.select_index = 0;
                } else if bar_selector_com.select_index == 0 && ev.y > 0.0 {
                    bar_selector_com.select_index = 4;
                } else if ev.y < 0.0 {
                    bar_selector_com.select_index += 1;
                } else if ev.y > 0.0 {
                    bar_selector_com.select_index -= 1;
                }
            }
            MouseScrollUnit::Pixel => {}
        }
    }
}
