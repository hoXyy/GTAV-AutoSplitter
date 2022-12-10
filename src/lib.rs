#![allow(unused_assignments)]

use asr::timer::{self, TimerState};
use std::collections::HashMap;
use std::{collections::HashSet, sync::Mutex};

pub mod game;
use game::{GameProcess, Variables};

pub mod data;
use data::collectibles::COLLECTIBLES;
use data::flags::FLAGS;
use data::missions::{FREAKS, MISSIONS};

pub mod settings;

static GAME_PROCESS: Mutex<Option<GameProcess>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn update() {
    let mut mutex = GAME_PROCESS.lock().unwrap();

    if mutex.is_none() {
        // (Re)connect to the game
        *mutex = GameProcess::connect("GTA5");
    } else {
        let game = mutex.as_mut().unwrap();

        // Make sure we're still connected to the game
        if !game.process.is_open() {
            *mutex = None;
            return;
        }

        let splits = &mut game.splits;
        let settings = &mut game.settings;

        let vars = match game.state.update(&mut game.process, &settings) {
            Some(v) => v,
            None => {
                asr::print_message("Error updating state!");
                return;
            }
        };

        if timer::state() == TimerState::Running {
            if get_setting(&settings, "auto_split") {
                handle_split(&vars, splits, &settings);
            }
        }

        if timer::state() == TimerState::NotRunning {
            if get_setting(&settings, "auto_start") {
                handle_start(&vars);
            }
        }
    }
}

fn handle_split(
    vars: &Variables,
    splits: &mut HashSet<String>,
    settings: &HashMap<&'static str, bool>,
) {
    // Stunt Jumps split
    if get_setting(&settings, "stunt_jumps") {
        let jumps = vars.int_variables.get("stunt_jumps").unwrap();
        if jumps.current == jumps.old + 1 {
            timer::split();
            asr::print_message("[Split] Stunt Jump");
        }
    }

    // Random Event split
    if get_setting(&settings, "random_events") {
        let variable = vars.int_variables.get("random_events").unwrap();
        if variable.current == variable.old + 1 {
            timer::split();
            asr::print_message("[Split] Random Event");
        }
    }

    // Hobbies and Pastttimes split
    if get_setting(&settings, "hobbies") {
        let variable = vars.int_variables.get("hobbies").unwrap();
        if variable.current == variable.old + 1 {
            timer::split();
            asr::print_message("[Split] Hobbies and Pasttimes");
        }
    }

    // Ending A split
    if get_setting(&settings, "ending_a") {
        let cutscene = vars.string_variables.get("current_cutscene").unwrap();
        let no_control = vars.short_variables.get("no_control").unwrap();
        let in_mission = vars.short_variables.get("in_mission").unwrap();
        if !splits.contains("Ending A Split")
            && no_control.current == 1
            && no_control.current != no_control.old
            && in_mission.current == 1
            && Variables::get_as_string(&cutscene.current).unwrap() == "fin_a_ext"
        {
            timer::split();
            asr::print_message("[Split] Ending A split");
            splits.insert("Ending A Split".to_owned());
        }
    }

    // Golf split
    if get_setting(&settings, "golf_split") {
        let mut current_golf_hole = 0;
        let variable = vars.int_variables.get("golf_hole").unwrap();
        if variable.current > 1
            && variable.current != variable.old
            && variable.current == current_golf_hole + 1
        {
            timer::split();
            asr::print_message("[Split] Golf split");
        }

        if variable.current > 0 {
            current_golf_hole = variable.current;
        }
    }

    // Mission passed split
    let last_mission = vars.int_variables.get("last_passed_mission").unwrap();
    if last_mission.current != last_mission.old {
        let mission = MISSIONS.get(&last_mission.current).unwrap();
        if !splits.contains(mission.name) {
            if get_setting(&settings, mission.script) {
                timer::split();
                asr::print_message(&format!("[Split] Mission Passed: {}", mission.name));
                splits.insert(mission.name.to_owned());
            }
        }
    }

    // Freaks passed split
    for id in FREAKS.keys() {
        let freak = FREAKS.get(id).unwrap();
        let freak_variable = vars.short_variables.get(freak.script).unwrap();
        if !splits.contains(freak.name) {
            if get_setting(&settings, freak.script) {
                if get_bit_at(&freak_variable.current, 3) != get_bit_at(&freak_variable.old, 3) {
                    timer::split();
                    asr::print_message(&format!("[Split] Freaks Split: {}", &freak.name));
                    splits.insert(freak.name.to_string());
                }
            }
        }
    }

    for flag in FLAGS.iter() {
        if get_setting(settings, flag.name) {
            let flag_variable = vars.int_variables.get(flag.name).unwrap();
            if !splits.contains(flag.name) {
                if &flag_variable.current > &flag_variable.old {
                    timer::split();
                    asr::print_message(&format!("[Split] Flag Split: {}", &flag.name));
                    splits.insert(flag.name.to_string());
                }
            }
        }
    }

    for collectible in &COLLECTIBLES {
        if get_setting(settings, collectible.name) {
            let (collectible_address, collectible_value) =
                Variables::get_collectible(&vars, collectible.name);
            if get_collectible_value(collectible_address.current, collectible_value.current)
                == get_collectible_value(collectible_address.old, collectible_value.old) + 1
            {
                timer::split();
                asr::print_message(&format!(
                    "[Split] Collectible Split: {}",
                    collectible.name.to_string()
                ));
            }
        }
    }
}

fn handle_start(vars: &Variables) {
    if vars.string_variables.contains_key("debug_text") {
        let debug_text = match vars.string_variables.get("debug_text") {
            Some(v) => v,
            None => todo!(),
        };
        if Variables::get_as_string(&debug_text.current).unwrap() == "PRO_SETTING"
            && debug_text.current != debug_text.old
        {
            asr::print_message("[Start] Starting timer due to Prologue start");
            timer::start();
        }
    }
}

fn get_bit_at(input: &i8, n: u8) -> bool {
    if n < 8 {
        input & (1 << n) != 0
    } else {
        false
    }
}

fn get_collectible_value(address: u64, value: u64) -> u64 {
    address + 0x10 & 0xFFFFFFFF ^ value
}

fn get_setting(settings: &HashMap<&'static str, bool>, name: &str) -> bool {
    *settings.get(name).unwrap()
}
