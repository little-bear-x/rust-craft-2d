use bevy::prelude::*;
use super::basic::*;
use super::game_map::PlayerMapInfo;

#[derive(Resource)]
struct DebugTick(Timer);  // debug间隔时长
const LOAD_MAT_TIME: f32 = 5.0;  // debug检测时间，默认2

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugTick(Timer::from_seconds(LOAD_MAT_TIME, TimerMode::Repeating)));
        app.add_systems(Update, debug);
    }
}

fn debug(
    player_query: Query<&PlayerMapInfo, With<Player>>,
    mut timer: ResMut<DebugTick>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // for player in player_query.iter() {

        //     println!("{:#?}", &player.map_hashmap);

        //     // for (x, yh) in &player.map_hashmap {
        //     //     print!("x: {}, y:", x);
        //     //     for (y, tp) in yh {
        //     //         print!("{}", y);
        //     //     }
        //     //     println!();
        //     // }
        // }
    }
}

