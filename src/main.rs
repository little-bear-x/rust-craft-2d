use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod player;
pub mod camera;
pub mod cube;
pub mod debug;
pub mod window;

// 通用组件
pub mod basic;
pub mod gameui;

use player::PlayerPlugin;
use camera::CameraPlugin;
use cube::CubePlugin;
// use debug::DebugPlugin;
use window::WindowPlugin;
use gameui::GameUiPlugin;
use basic::*;

use std::{collections::HashMap, io::Read};
use core::array::from_fn;
use std::env;

fn main(){
    // 获取命令行参数
    let mut is_creative_mode = false;  // 游戏模式
    let mut player_bar: [(Option<GameObjType>, isize); 5] = from_fn(|_| (Option::None, 0));  // 玩家物品栏
    let mut player_bar_select_index: usize = 0;  // 玩家当前手持物品在物品栏的索引
    let mut game_save: String = "default".to_string();  // 游戏存档
    let mut player_map: HashMap<i32, HashMap<i32, Cube>> = HashMap::new();  // 游戏地图
    let mut player_init_pos: (f32, f32) = (0., 1.);  // 玩家位置

    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        println!("[main.rs/main]info: Starting...received: {}", args[i]);
        // 是否开启创造模式
        if args[i] == "--gametype" {
            if args[i+1] == "sandbox" { 
                is_creative_mode = true;
            }
            else if args[i+1] == "survival" { 
                is_creative_mode = false;
            }
            else {
                println!("[main.rs/main]error: {} is an invalid game type!", args[i+1]);
                return;
            }
            player_bar = [
                (Some(GameObjType::Cube(Cube::Plank)), if is_creative_mode { -1 } else { 64 }),
                (Some(GameObjType::Cube(Cube::GrassCube)), if is_creative_mode { -1 } else { 64 }),
                (Some(GameObjType::Cube(Cube::SoilCube)), if is_creative_mode { -1 } else { 64 }),
                (Some(GameObjType::Cube(Cube::StoneCube)), if is_creative_mode { -1 } else { 64 }),
                (Some(GameObjType::Cube(Cube::StoneBrick)), if is_creative_mode { -1 } else { 64 })
            ];
        }

        // 创建新的文件
        if args[i] == "--new" {
            let file_name = args[i+1].clone();
            game_save = file_name.clone();
            println!("[main.rs/main]info: Creating archive file: {}", file_name);
            // 创建文件
            std::fs::File::create("./SavedGames/".to_string() + &file_name + ".game").expect("[main.rs/main]panic: Failed to create archive file!");
        }

        // 读取文件
        else if args[i] == "--open" {
            let file_name = args[i+1].clone();
            game_save = file_name.clone();
            println!("[main.rs/main]info: Reading archive file: {}", file_name);
            // 读取文件
            let mut file = std::fs::File::open("./SavedGames/".to_string() + &file_name + ".game").expect("[main.rs/main]panic: Failed to open archive file!");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("[main.rs/main]panic: Failed to read archive file!");
        
            let saved_game_data: SavedGameData = serde_json::from_str(&contents)
                .expect("[main.rs/main]panic: Unable to parse archive to JSON, please check if the archive is damaged or if the game version matches the archive!");
            // println!("{:#?}", saved_game_data);
            // 玩家物品栏
            for i in 0..5 {
                player_bar[i] = (new_game_obj(&saved_game_data.player_bar.player_bar_items[i]), saved_game_data.player_bar.player_bar_items_count[i] as isize);
            }
            // 玩家物品栏索引
            player_bar_select_index = saved_game_data.player_bar.player_bar_select_index;
            // 玩家地图
            for i in 0..saved_game_data.player_map.cube_pos.len() {
                if player_map.contains_key(&(saved_game_data.player_map.cube_pos[i][0] as i32)) {
                    // 存在x表
                    player_map.insert(saved_game_data.player_map.cube_pos[i][0], 
                        match player_map.get(&saved_game_data.player_map.cube_pos[i][0]).cloned() {
                            Some(mut cube_map) => {
                                cube_map.insert(saved_game_data.player_map.cube_pos[i][1], 
                                    match new_game_obj(saved_game_data.player_map.cube_type[i].as_str()) { 
                                        Some(obj) => {
                                            match obj {
                                                GameObjType::Cube(cube) => cube,
                                                _ => panic!("[main.rs/main]panic: Unable to parse player map, save may be damaged!")
                                            }
                                        },
                                        _ => panic!("[main.rs/main]panic: Unable to parse player map, save may be damaged!") 
                                    }
                                );
                                cube_map
                            },
                            _ => panic!("[main.rs/main]panic: Unable to parse player map, save may be damaged!")
                        }
                    );
                } else {
                    // 不存在x表
                    // 玩家地图二级hashmap
                    let mut player_map_y: HashMap<i32, Cube> = HashMap::new();
                    player_map_y.insert(saved_game_data.player_map.cube_pos[i][1], 
                        match new_game_obj(saved_game_data.player_map.cube_type[i].as_str()) { 
                            Some(obj) => {
                                match obj {
                                    GameObjType::Cube(cube) => cube,
                                    _ => panic!("[main.rs/main]panic: Unable to parse player map, save may be damaged!")
                                }
                            },
                            _ => panic!("[main.rs/main]panic: Unable to parse player map, save may be damaged!") 
                        }
                    );
                    player_map.insert(saved_game_data.player_map.cube_pos[i][0], player_map_y);
                }
            }
            // 玩家位置
            player_init_pos = (saved_game_data.player_info.player_pos[0], saved_game_data.player_info.player_pos[1]);
        }
    }
    
    // 构建app
    let mut app = App::new();
    app.insert_resource(PlayerInfo{
        player_map,
        is_controlling: true,
        is_paused: false,
        is_creative_mode,
        player_bar,
        player_bar_select_index,
        game_save,
        player_init_pos,
    });

    app.add_plugins(DefaultPlugins);
    // 物理引擎
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    // app.add_plugins(RapierDebugRenderPlugin::default()); // 物理引擎调试
    // 其他
    app.add_plugins(CubePlugin);
    app.add_plugins(PlayerPlugin);
    app.add_plugins(CameraPlugin);
    // 调试
    // .add_plugins(DebugPlugin)
    // 窗口
    app.add_plugins(WindowPlugin);
    app.add_plugins(GameUiPlugin);
    app.add_systems(PostUpdate, exit_on_all_closed);
    app.run();
}

pub fn exit_on_all_closed(windows: Query<&Window>, player_info: ResMut<PlayerInfo>, player: Query<&Transform, With<Player>>) {
    if windows.is_empty() {
        println!("[main.rs/exit_on_all_closed]info: Saving game...");
        // 保存游戏
        // 保存游戏地图
        let mut cube_pos: Vec<[i32; 2]> = Vec::new();
        let mut cube_type: Vec<String> = Vec::new();
        for (x, x_map) in player_info.player_map.iter() {
            for (y, cube) in x_map.iter() {
                cube_pos.push([*x, *y]);
                cube_type.push(get_game_id(GameObjType::Cube(cube.clone())));
            }
        }
        let saved_game_map = SavedGameMap {
            cube_pos,
            cube_type,
        };
        // 保存玩家物品栏
        let mut player_bar_items: [String; 5] = from_fn(|_| (String::from("None")));
        let mut player_bar_items_count: [i32; 5] = [0; 5];
        for i in 0..5 {
            match player_info.player_bar[i].0.clone() {
                Some(obj) => {
                    player_bar_items[i] = get_game_id(obj);
                },
                None => {}
            };
            player_bar_items_count[i] = player_info.player_bar[i].1 as i32;
        }
        let player_bar_select_index = player_info.player_bar_select_index;
        let saved_game_player_bar = SavedGamePlayerBar {
            player_bar_items,
            player_bar_items_count,
            player_bar_select_index,
        };
        // 玩家位置
        let player_pos = [player.single().translation.x, player.single().translation.y];
        let saved_game_player_info = SavedGamePlayerInfo {
            player_pos,
        };
        // 整合
        let saved_game_data = SavedGameData {
            player_bar: saved_game_player_bar,
            player_map: saved_game_map,
            player_info: saved_game_player_info,
        };
        let json = serde_json::to_string(&saved_game_data).expect("[main.rs/exit_on_all_closed]panic: error occurred while saving the game. Game save failed!");
        // println!("{:#?}", json);
        std::fs::write("./SavedGames/".to_string() + &(player_info.game_save) + ".game", json).expect("[main.rs/exit_on_all_closed]panic: Unable to write archive file, game save failed!");
    }
}
