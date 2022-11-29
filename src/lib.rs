#![allow(unused_assignments)]

use asr::timer::{self, TimerState};
use once_cell::sync::Lazy;
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
static SETTINGS: Lazy<HashMap<&str, bool>> = Lazy::new(|| settings::get_settings());

#[no_mangle]
pub extern "C" fn update() {
    let mut mutex = GAME_PROCESS.lock().unwrap();
    let settings = &SETTINGS;

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

        let vars = match game.state.update(&mut game.process) {
            Some(v) => v,
            None => {
                asr::print_message("Error updating state!");
                return;
            }
        };

        let splits = &mut game.splits;

        if timer::state() == TimerState::Running {
            if *settings.get("auto_split").unwrap() {
                handle_split(&vars, splits, settings);
            }
        }

        if timer::state() == TimerState::NotRunning {
            if *settings.get("auto_start").unwrap() {
                handle_start(&vars);
            }
        }
    }
}

fn handle_split(
    vars: &Variables,
    splits: &mut HashSet<String>,
    settings: &Lazy<HashMap<&'static str, bool>>,
) {
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
    if let Some(current_cutscene) = vars.current_cutscene {
        if !splits.contains("Ending A Split")
            && vars.no_control.current == 1
            && vars.no_control.current != vars.no_control.old
            && vars.in_mission.current == 1
            && Variables::get_as_string(&current_cutscene.current).unwrap() == "fin_a_ext"
        {
            timer::split();
            asr::print_message("[Split] Ending A split");
            splits.insert("Ending A Split".to_owned());
        }
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

    // Mission passed split
    if vars.last_passed_mission.current != vars.last_passed_mission.old {
        let mission = MISSIONS.get(&vars.last_passed_mission.current).unwrap();
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
        let freak_variable = Variables::get_freak(&vars, freak.script);
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

    for flag in FLAGS {
        let flag_variable = Variables::get_flag(&vars, flag);
        if !splits.contains(flag) {
            if &flag_variable.current > &flag_variable.old {
                timer::split();
                asr::print_message(&format!("[Split] Flag Split: {}", &flag));
                splits.insert(flag.to_string());
            }
        }
    }

    for collectible in &COLLECTIBLES {
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

fn get_setting(settings: &Lazy<HashMap<&'static str, bool>>, name: &str) -> bool {
    *settings.get(name).unwrap()
}
