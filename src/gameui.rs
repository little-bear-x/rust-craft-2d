use bevy::{input::mouse::{MouseWheel, MouseScrollUnit}, prelude::*, window::PrimaryWindow, text::Text2dBounds};
use super::basic::*;

pub struct GameUiPlugin;
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_bar);
        app.add_systems(Update, change_select);
        app.add_systems(Update, update_background);
    }
}

// 初始化页面
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_info: ResMut<PlayerInfo>,
) {
    let primary_window = window_query.get_single().unwrap();
    let window_width = primary_window.width();
    let window_height = primary_window.height();
    
    // 初始化背景
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform::from_xyz(0., 0., -1.),
        sprite: Sprite {
            custom_size: Some(Vec2::new(window_width, window_height)),
            ..Default::default()
        },
        ..default()
    }).insert(Background);

    // 初始化物品栏
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
        .insert(BarCom);
    }
    player_info.player_bar = [
        (Some(GameObjType::Cube(Cube::Plank)), 32),
        (Some(GameObjType::Cube(Cube::GrassCube)), -1),
        (Some(GameObjType::Cube(Cube::SoilCube)), -1),
        (Some(GameObjType::Cube(Cube::StoneCube)), -1),
        (Some(GameObjType::Cube(Cube::StoneBrick)), 32)
    ];
    // 初始化物品栏选中
    commands.spawn(SpriteBundle {
        texture: asset_server.load("other/item_choose.png"),
        transform: Transform::from_xyz(-100., sprite_position_y, 9.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            ..Default::default()
        },
        ..default()
    }).insert(BarSelectorCom);
    player_info.player_bar_select_index = 0;

    // 渲染物品栏上的图标与数量
    for (i, (bar_icon, num)) in player_info.player_bar.iter().enumerate() {
        match bar_icon {
            Some(game_obj_type) => {
                match game_obj_type.clone() {
                    GameObjType::Cube(cube) => {
                        println!("{}", num.to_string());
                        commands.spawn(SpriteBundle {
                            texture: asset_server.load(get_cube_model(&cube)),
                            transform: Transform::from_xyz((50*((i as isize)-2)) as f32, sprite_position_y, 9.0),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(30., 30.)),
                                ..Default::default()
                            },
                            ..default()
                        }).insert(BarIconCom{ bar_index: i })
                        // 渲染物品栏上物品的数量
                        .with_children(|builder| {
                            builder.spawn(Text2dBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        num.to_string(),
                                        TextStyle {
                                            font: asset_server.load("fonts/white-border.ttf"),
                                            font_size: 20.0,
                                            ..default()
                                        }
                                    )],
                                    justify: JustifyText::Left,
                                    ..Default::default()
                                },
                                text_2d_bounds: Text2dBounds {
                                    // Wrap text in the rectangle
                                    size: Vec2::new(30., 30.),
                                },
                                // ensure the text is drawn on top of the box
                                transform: Transform::from_xyz(0., 0., 0.),
                                ..default()
                            }).insert(BarTextCom { bar_index: i });
                        });
                    }
                    _ => {}
                }
                
            }
            _ => {}
        }
    }
}

// 更新bar
fn update_bar(
    mut commands: Commands,
    mut bar_query: Query<&mut Transform, (Without<BarIconCom>, With<BarCom>, Without<CameraCom>, Without<BarSelectorCom>)>,
    mut bar_selector_query: Query<&mut Transform, (Without<BarIconCom>, With<BarSelectorCom>, Without<CameraCom>, Without<BarCom>)>,
    mut bar_icon_query: Query<(&mut Transform, &BarIconCom, Entity), (With<BarIconCom>, Without<CameraCom>, Without<BarSelectorCom>, Without<BarCom>)>,
    mut bar_text_query: Query<(&mut Text, &BarTextCom, Entity), With<BarTextCom>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Transform, (Without<BarIconCom>, With<CameraCom>, Without<BarCom>, Without<BarSelectorCom>)>,
    player_info: Res<PlayerInfo>
) {
    let camera = camera_query.get_single().unwrap().translation;

    // 更新物品栏位置
    let primary_window = window_query.get_single().unwrap();
    let window_height = primary_window.height();
    let sprite_position_y = -window_height / 2.0+50.;
    for (i, mut bar_transform) in bar_query.iter_mut().enumerate() {
        bar_transform.translation.x = camera.x + (50*((i as isize)-2)) as f32;
        bar_transform.translation.y = camera.y + sprite_position_y;
    }

    // 更新物品栏选中器位置
    let mut bar_selector_translation = bar_selector_query.get_single_mut().unwrap();
    bar_selector_translation.translation.x = camera.x+(50*((player_info.player_bar_select_index as isize)-2)) as f32;
    bar_selector_translation.translation.y = camera.y + sprite_position_y;

    // 更新物品栏图标位置
    for (mut bar_transform, i, entity) in bar_icon_query.iter_mut() {
        // 如果方块数量为0，则删除该图标
        if player_info.player_bar[i.bar_index].1 == 0 {
            commands.entity(entity).despawn();
        }
        bar_transform.translation.x = camera.x + (50*((i.bar_index as isize)-2)) as f32;
        bar_transform.translation.y = camera.y + sprite_position_y;
    }

    // 更新物品栏文字
    for (mut text, i, entity) in bar_text_query.iter_mut() {
        if player_info.player_bar[i.bar_index].1 == 0 {
            commands.entity(entity).despawn();
        }
        text.sections[0].value = player_info.player_bar[i.bar_index].1.to_string();
    }
}

// 修改背景位置
fn update_background(
    mut background_query: Query<(&mut Transform, &mut Sprite), (With<Background>, Without<CameraCom>)>,
    camera_query: Query<&Transform, (Without<Background>, With<CameraCom>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = window_query.get_single().unwrap();
    let window_width = primary_window.width();
    let window_height = primary_window.height();
    let camera = camera_query.get_single().unwrap().translation;
    let (mut background_transform, mut background_sprite) = background_query.get_single_mut().unwrap();
    background_transform.translation.x = camera.x;
    background_transform.translation.y = camera.y;

    background_sprite.custom_size = Some(Vec2::new(window_width+100., window_height+100.));
    
}

// 更改选中物品
fn change_select(
    mut mouse_scroll: EventReader<MouseWheel>,
    mut player_info: ResMut<PlayerInfo>,
) {
    for ev in mouse_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if player_info.player_bar_select_index == 4 && ev.y < 0.0 {
                    player_info.player_bar_select_index = 0;
                } else if player_info.player_bar_select_index == 0 && ev.y > 0.0 {
                    player_info.player_bar_select_index = 4;
                } else if ev.y < 0.0 {
                    player_info.player_bar_select_index += 1;
                } else if ev.y > 0.0 {
                    player_info.player_bar_select_index -= 1;
                }
            }
            MouseScrollUnit::Pixel => {}
        }
    }
}
