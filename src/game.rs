use asr::{
    watcher::{Pair, Watcher},
    Address, Process,
};

use std::collections::{HashMap, HashSet};

use crate::settings;
use crate::{COLLECTIBLES, FLAGS, FREAKS};

pub struct GameProcess {
    pub process: Process,
    pub state: State,
    pub splits: HashSet<String>,
    pub settings: HashMap<&'static str, bool>,
}

impl GameProcess {
    pub fn connect(process_name: &str) -> Option<Self> {
        let process = Process::attach(process_name)?;
        let base_address = process.get_module_address("GTA5.exe").unwrap();
        asr::set_tick_rate(60.0);

        Some(Self {
            process,
            state: State::setup(base_address),
            splits: HashSet::new(),
            settings: settings::get_settings(),
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
    int_alwaysupdate: HashMap<&'static str, Variable<i32>>,
    short_alwaysupdate: HashMap<&'static str, Variable<i8>>,
    int_variables: HashMap<&'static str, Variable<i32>>,
    short_variables: HashMap<&'static str, Variable<i8>>,
    string_variables: HashMap<&'static str, Variable<[u8; 100]>>,
    ulong_variables: HashMap<String, Variable<u64>>,
}

impl State {
    fn setup(base_address: Address) -> Self {
        let mut int_alwaysupdate: HashMap<&'static str, Variable<i32>> = HashMap::new();
        let mut int_variables: HashMap<&'static str, Variable<i32>> = HashMap::new();
        let mut string_variables: HashMap<&'static str, Variable<[u8; 100]>> = HashMap::new();
        let mut short_alwaysupdate: HashMap<&'static str, Variable<i8>> = HashMap::new();
        let mut short_variables: HashMap<&'static str, Variable<i8>> = HashMap::new();
        let mut ulong_variables: HashMap<String, Variable<u64>> = HashMap::new();

        int_variables.insert(
            "stunt_jumps",
            Variable::<i32> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xCE5C0],
            },
        );
        int_variables.insert(
            "random_events",
            Variable::<i32> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xBDA28],
            },
        );
        int_variables.insert(
            "hobbies",
            Variable::<i32> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xBDA10],
            },
        );
        string_variables.insert(
            "current_cutscene",
            Variable::<[u8; 100]> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1CB4340],
            },
        );
        string_variables.insert(
            "current_script",
            Variable::<[u8; 100]> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1CB4340],
            },
        );
        int_alwaysupdate.insert(
            "loading",
            Variable::<i32> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2AC7CF4],
            },
        );
        int_alwaysupdate.insert(
            "golf_hole",
            Variable::<i32> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1DE3970],
            },
        );
        short_alwaysupdate.insert(
            "in_cutscene",
            Variable::<i8> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1CB4472],
            },
        );
        short_alwaysupdate.insert(
            "in_mission",
            Variable::<i8> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1DD6CB9],
            },
        );
        short_alwaysupdate.insert(
            "in_mission2",
            Variable::<i8> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x22959C3],
            },
        );
        short_alwaysupdate.insert(
            "no_control",
            Variable::<i8> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x1DD034D],
            },
        );
        string_variables.insert(
            "debug_text",
            Variable::<[u8; 100]> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2295A10, 0x0],
            },
        );
        int_alwaysupdate.insert(
            "mpassed_screen",
            Variable::<i32> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07D48, 0xA60, 0x13C0],
            },
        );
        int_alwaysupdate.insert(
            "collectible_screen",
            Variable::<i32> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2AC7BA0, 0xD97A8],
            },
        );
        int_alwaysupdate.insert(
            "last_passed_mission",
            Variable::<i32> {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0x85CE8],
            },
        );

        for freak in FREAKS.values() {
            short_variables.insert(
                freak.script,
                Variable::<i8> {
                    var: Watcher::new(),
                    base_address,
                    address_path: vec![0x2A07E70, (0xDF030 + (48 * freak.id)).into()],
                },
            );
        }

        for flag in FLAGS.iter() {
            int_variables.insert(
                flag.name,
                Variable::<i32> {
                    var: Watcher::new(),
                    base_address,
                    address_path: vec![0x2A07E70, (0xCCCA0 + (flag.id * 8)).into()],
                },
            );
        }

        for collectible in COLLECTIBLES {
            ulong_variables.insert(
                format!("{}_address", collectible.name),
                Variable::<u64> {
                    var: Watcher::new(),
                    base_address,
                    address_path: vec![0x22B54E0 + 8, collectible.address * 16 + 8],
                },
            );
            ulong_variables.insert(
                format!("{}_value", collectible.name),
                Variable::<u64> {
                    var: Watcher::new(),
                    base_address,
                    address_path: vec![0x22B54E0 + 8, collectible.address * 16 + 8, 0x10],
                },
            );
        }

        Self {
            int_alwaysupdate,
            short_alwaysupdate,
            int_variables,
            string_variables,
            short_variables,
            ulong_variables,
        }
    }
}

impl State {
    pub fn update(
        &mut self,
        process: &Process,
        settings: &HashMap<&str, bool>,
    ) -> Option<Variables> {
        let mut int_variable_pairs: HashMap<&'static str, Pair<i32>> = HashMap::new();

        for variable in &mut self.int_variables {
            if settings.contains_key(variable.0) {
                if *settings.get(variable.0).unwrap() {
                    let pair = variable.1.update(process)?;
                    int_variable_pairs.insert(variable.0, *pair);
                }
            }
        }

        let mut short_variable_pairs: HashMap<&'static str, Pair<i8>> = HashMap::new();

        for variable in &mut self.short_variables {
            if settings.contains_key(variable.0) {
                if *settings.get(&variable.0[..]).unwrap() {
                    let pair = variable.1.update(process)?;
                    short_variable_pairs.insert(variable.0, *pair);
                }
            }
        }

        let mut string_variable_pairs: HashMap<&'static str, Pair<[u8; 100]>> = HashMap::new();

        for variable in &mut self.string_variables {
            match variable.1.update(process) {
                Some(v) => {
                    string_variable_pairs.insert(variable.0, *v);
                }
                None => {
                    asr::print_message("Couldn't update string variable");
                }
            }
        }

        let mut ulong_variable_pairs: HashMap<String, Pair<u64>> = HashMap::new();

        for variable in &mut self.ulong_variables {
            let pair = variable.1.update(process)?;
            ulong_variable_pairs.insert(String::from(variable.0), *pair);
        }

        for variable in &mut self.int_alwaysupdate {
            let pair = variable.1.update(process)?;
            int_variable_pairs.insert(variable.0, *pair);
        }

        for variable in &mut self.short_alwaysupdate {
            let pair = variable.1.update(process)?;
            short_variable_pairs.insert(variable.0, *pair);
        }

        Some(Variables {
            int_variables: int_variable_pairs,
            short_variables: short_variable_pairs,
            string_variables: string_variable_pairs,
            ulong_variables: ulong_variable_pairs,
        })
    }
}

pub struct Variables {
    pub int_variables: HashMap<&'static str, Pair<i32>>,
    pub short_variables: HashMap<&'static str, Pair<i8>>,
    pub string_variables: HashMap<&'static str, Pair<[u8; 100]>>,
    pub ulong_variables: HashMap<String, Pair<u64>>,
}

impl<'a> Variables {
    pub fn get_as_string(var: &'a [u8]) -> Option<&'a str> {
        let null_pos = var.iter().position(|&x| x == b'\0').unwrap_or(var.len());

        std::str::from_utf8(&var[0..null_pos]).ok()
    }

    pub fn get_collectible(&self, collectible: &str) -> (Pair<u64>, Pair<u64>) {
        let name_address = format!("{}_address", collectible);
        let name_value = format!("{}_value", collectible);
        if self.ulong_variables.contains_key(&name_address)
            && self.ulong_variables.contains_key(&name_value)
        {
            (
                *self.ulong_variables.get(&name_address).unwrap(),
                *self.ulong_variables.get(&name_value).unwrap(),
            )
        } else {
            asr::print_message("Not a valid collectible!");
            panic!()
        }
    }
}
