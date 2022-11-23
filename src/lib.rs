use asr::timer;
use std::{collections::HashSet, sync::Mutex};

pub mod game;
use game::{GameProcess, Variables};

static GAME_PROCESS: Mutex<Option<GameProcess>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn update() {
    let mut mutex = GAME_PROCESS.lock().unwrap();

    if mutex.is_none() {
        // (Re)connect to the game
        *mutex = GameProcess::connect("GTA5");
    } else {
        let game = mutex.as_mut().unwrap();

        // Make sure we're still connected to the game, pause game time if not
        if !game.process.is_open() {
            *mutex = None;
            return;
        }

        let vars = match game.state.update(&mut game.process) {
            Some(v) => v,
            None => {
                asr::print_message("Error updating state!");
                return;
            }
        };

        let splits = &mut game.splits;

        handle_split(&vars, splits);
        handle_start(&vars);
    }
}

fn handle_split(vars: &Variables, splits: &mut HashSet<String>) {
    let mut current_golf_hole = 0;

    // Stunt Jumps split
    if vars.stunt_jumps.current == vars.stunt_jumps.old + 1 {
        timer::split();
        asr::print_message("[Split] Stunt Jump");
    }

    // Random Event split
    if vars.random_events.current == vars.random_events.old + 1 {
        timer::split();
        asr::print_message("[Split] Random Event");
    }

    // Hobbies and Pastttimes split
    if vars.hobbies.current == vars.hobbies.old + 1 {
        timer::split();
        asr::print_message("[Split] Hobbies and Pasttimes");
    }

    // Ending A split
    if !splits.contains("Ending A Split")
        && vars.no_control.current == 1
        && vars.no_control.current != vars.no_control.old
        && vars.in_mission.current == 1
        && Variables::get_as_string(&vars.current_cutscene.unwrap().current).unwrap() == "fin_a_ext"
    {
        timer::split();
        asr::print_message("[Split] Ending A split");
        splits.insert("Ending A Split".to_owned());
    }

    // Golf split
    if vars.golf_hole.current > 1
        && vars.golf_hole.current != vars.golf_hole.old
        && vars.golf_hole.current == current_golf_hole + 1
    {
        timer::split();
        asr::print_message("[Split] Golf split");
    }

    if vars.golf_hole.current > 0 {
        current_golf_hole = vars.golf_hole.current;
    }

    if vars.last_passed_mission.current != vars.last_passed_mission.old {
        timer::split();
        asr::print_message("[Split] Mission Passed");
    }
}

fn handle_start(vars: &Variables) {
    if let Some(debug_text) = vars.debug_text {
        if Variables::get_as_string(&debug_text.current).unwrap() == "PRO_SETTING"
            && vars.debug_text.unwrap().current != vars.debug_text.unwrap().old
        {
            asr::print_message("[Start] Starting timer due to Prologue start");
            timer::start();
        }
    }
}
