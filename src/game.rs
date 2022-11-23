use asr::{
    watcher::{Pair, Watcher},
    Address, Process,
};
use std::collections::HashSet;

pub struct GameProcess {
    pub process: Process,
    pub state: State,
    pub splits: HashSet<String>,
}

impl GameProcess {
    pub fn connect(process_name: &str) -> Option<Self> {
        let process = Process::attach(process_name)?;
        let base_address = process.get_module_address("GTA5.exe").unwrap();

        Some(Self {
            process,
            state: State::setup(base_address),
            splits: HashSet::new(),
        })
    }
}

pub struct Variable<T> {
    var: Watcher<T>,
    base_address: Address,
    address_path: Vec<u64>,
}

impl<T: bytemuck::Pod + std::fmt::Debug> Variable<T> {
    pub fn update(&mut self, process: &Process) -> Option<&Pair<T>> {
        self.var.update(
            process
                .read_pointer_path64(self.base_address.0.try_into().unwrap(), &self.address_path)
                .ok(),
        )
    }
}

pub struct State {
    pub stunt_jumps: Variable<i32>,
    pub random_events: Variable<i32>,
    pub hobbies: Variable<i32>,
    pub current_cutscene: Variable<[u8; 100]>,
    pub current_script: Variable<[u8; 100]>,
    pub loading: Variable<i32>,
    pub golf_hole: Variable<i32>,
    pub in_cutscene: Variable<i8>,
    pub in_mission: Variable<i8>,
    pub in_mission2: Variable<i8>,
    pub no_control: Variable<i8>,
    pub debug_text: Variable<[u8; 100]>,
    pub mpassed_screen: Variable<i32>,
    pub collectible_screen: Variable<i32>,
    pub last_passed_mission: Variable<i32>,
}

impl State {
    fn setup(base_address: Address) -> Self {
        Self {
            stunt_jumps: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xCE5C0],
            },
            random_events: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xBDA28],
            },
            hobbies: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xBDA10],
            },
            current_cutscene: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x01CB44A0, 0xB70],
            },
            current_script: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1CB4340],
            },
            loading: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2AC7CF4],
            },
            golf_hole: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1DE3970],
            },
            in_cutscene: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1CB4472],
            },
            in_mission: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1DD6CB9],
            },
            in_mission2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x22959C3],
            },
            no_control: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1DD034D],
            },
            debug_text: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2295A10, 0x0],
            },
            mpassed_screen: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07D48, 0xA60, 0x13C0],
            },
            collectible_screen: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2AC7BA0, 0xD97A8],
            },
            last_passed_mission: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0x85CE8],
            },
        }
    }
}

impl State {
    pub fn update(&mut self, process: &Process) -> Option<Variables> {
        Some(Variables {
            stunt_jumps: self.stunt_jumps.update(process)?,
            random_events: self.random_events.update(process)?,
            hobbies: self.hobbies.update(process)?,
            current_cutscene: self.current_cutscene.update(process),
            current_script: self.current_script.update(process),
            loading: self.loading.update(process)?,
            golf_hole: self.golf_hole.update(process)?,
            in_cutscene: self.in_cutscene.update(process)?,
            in_mission: self.in_mission.update(process)?,
            in_mission2: self.in_mission2.update(process)?,
            no_control: self.no_control.update(process)?,
            debug_text: self.debug_text.update(process),
            mpassed_screen: self.mpassed_screen.update(process)?,
            collectible_screen: self.collectible_screen.update(process)?,
            last_passed_mission: self.last_passed_mission.update(process)?,
        })
    }
}

pub struct Variables<'a> {
    pub stunt_jumps: &'a Pair<i32>,
    pub random_events: &'a Pair<i32>,
    pub hobbies: &'a Pair<i32>,
    pub current_cutscene: Option<&'a Pair<[u8; 100]>>,
    pub current_script: Option<&'a Pair<[u8; 100]>>,
    pub loading: &'a Pair<i32>,
    pub golf_hole: &'a Pair<i32>,
    pub in_cutscene: &'a Pair<i8>,
    pub in_mission: &'a Pair<i8>,
    pub in_mission2: &'a Pair<i8>,
    pub no_control: &'a Pair<i8>,
    pub debug_text: Option<&'a Pair<[u8; 100]>>,
    pub mpassed_screen: &'a Pair<i32>,
    pub collectible_screen: &'a Pair<i32>,
    pub last_passed_mission: &'a Pair<i32>,
}

impl<'a> Variables<'a> {
    pub fn get_as_string(var: &'a [u8]) -> Option<&'a str> {
        let null_pos = var.iter().position(|&x| x == b'\0').unwrap_or(var.len());

        std::str::from_utf8(&var[0..null_pos]).ok()
    }
}
