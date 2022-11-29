use asr::{
    watcher::{Pair, Watcher},
    Address, Process,
};
use std::collections::{HashMap, HashSet};

use crate::settings;

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
    // Franklin freaks
    pub tonya1: Variable<i8>,
    pub tonya2: Variable<i8>,
    pub tonya3: Variable<i8>,
    pub tonya4: Variable<i8>,
    pub tonya5: Variable<i8>,
    pub hao1: Variable<i8>,
    pub paparazzo1: Variable<i8>,
    pub paparazzo2: Variable<i8>,
    pub paparazzo3a: Variable<i8>,
    pub paparazzo3b: Variable<i8>,
    pub paparazzo4: Variable<i8>,
    pub omega1: Variable<i8>,
    pub omega2: Variable<i8>,
    pub barry3: Variable<i8>,
    pub barry3a: Variable<i8>,
    pub barry3c: Variable<i8>,
    pub barry4: Variable<i8>,
    pub extreme1: Variable<i8>,
    pub extreme2: Variable<i8>,
    pub extreme3: Variable<i8>,
    pub extreme4: Variable<i8>,
    pub fanatic3: Variable<i8>,
    pub dreyfuss1: Variable<i8>,
    pub thelastone: Variable<i8>,
    // Michael freaks
    pub barry1: Variable<i8>,
    pub fanatic1: Variable<i8>,
    pub abigail1: Variable<i8>,
    pub abigail2: Variable<i8>,
    pub epsilon1: Variable<i8>,
    pub epsilon2: Variable<i8>,
    pub epsilon3: Variable<i8>,
    pub epsilon4: Variable<i8>,
    pub epsilon5: Variable<i8>,
    pub epsilon6: Variable<i8>,
    pub epsilon7: Variable<i8>,
    pub epsilon8: Variable<i8>,
    // Trevor freaks
    pub barry2: Variable<i8>,
    pub fanatic2: Variable<i8>,
    pub rampage1: Variable<i8>,
    pub rampage2: Variable<i8>,
    pub rampage3: Variable<i8>,
    pub rampage4: Variable<i8>,
    pub rampage5: Variable<i8>,
    pub hunting1: Variable<i8>,
    pub hunting2: Variable<i8>,
    pub minute1: Variable<i8>,
    pub minute2: Variable<i8>,
    pub minute3: Variable<i8>,
    pub maude1: Variable<i8>,
    pub nigel1: Variable<i8>,
    pub nigel1a: Variable<i8>,
    pub nigel1b: Variable<i8>,
    pub nigel1c: Variable<i8>,
    pub nigel1d: Variable<i8>,
    pub nigel2: Variable<i8>,
    pub nigel3: Variable<i8>,
    pub josh1: Variable<i8>,
    pub josh2: Variable<i8>,
    pub josh3: Variable<i8>,
    pub josh4: Variable<i8>,
    pub mrsphilips1: Variable<i8>,
    pub mrsphilips2: Variable<i8>,
    // Epsilon flags
    pub epsilon_donated500: Variable<i32>,
    pub epsilon_donated5k: Variable<i32>,
    pub epsilon_donated10k: Variable<i32>,
    pub epsilon_robe10days: Variable<i32>,
    pub epsilon_desertdone: Variable<i32>,
    // Collectibles
    pub bridges_address: Variable<u64>,
    pub bridges_value: Variable<u64>,
    pub letters_address: Variable<u64>,
    pub letters_value: Variable<u64>,
    pub spaceships_address: Variable<u64>,
    pub spaceships_value: Variable<u64>,
    pub nuclear_address: Variable<u64>,
    pub nuclear_value: Variable<u64>,
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
            tonya1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDFB10],
            },
            tonya2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDFB40],
            },
            tonya3: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDFB70],
            },
            tonya4: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDFBA0],
            },
            tonya5: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDFBD0],
            },
            hao1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF4B0],
            },
            paparazzo1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF8D0],
            },
            paparazzo2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF900],
            },
            paparazzo3a: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF960],
            },
            paparazzo3b: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF990],
            },
            paparazzo4: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF9C0],
            },
            omega1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF870],
            },
            omega2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF8A0],
            },
            barry3: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF0F0],
            },
            barry3a: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF120],
            },
            barry3c: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF150],
            },
            barry4: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF180],
            },
            extreme1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF360],
            },
            extreme2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF390],
            },
            extreme3: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF3C0],
            },
            extreme4: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF3F0],
            },
            fanatic3: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF480],
            },
            dreyfuss1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF1B0],
            },
            thelastone: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDFAE0],
            },
            barry1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF090],
            },
            fanatic1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF420],
            },
            abigail1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF030],
            },
            abigail2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF060],
            },
            epsilon1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF1E0],
            },
            epsilon2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF210],
            },
            epsilon3: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF240],
            },
            epsilon4: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF270],
            },
            epsilon5: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF2A0],
            },
            epsilon6: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF2D0],
            },
            epsilon7: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF300],
            },
            epsilon8: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF330],
            },
            barry2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF0C0],
            },
            fanatic2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF450],
            },
            rampage1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF9F0],
            },
            rampage2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDFA20],
            },
            rampage3: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDFA50],
            },
            rampage4: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDFA80],
            },
            rampage5: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDFAB0],
            },
            hunting1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF4E0],
            },
            hunting2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF510],
            },
            minute1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF630],
            },
            minute2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF660],
            },
            minute3: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF690],
            },
            maude1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF600],
            },
            nigel1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF720],
            },
            nigel1a: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF750],
            },
            nigel1b: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF780],
            },
            nigel1c: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF7B0],
            },
            nigel1d: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF7E0],
            },
            nigel2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF810],
            },
            nigel3: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF840],
            },
            josh1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF540],
            },
            josh2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF570],
            },
            josh3: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF5A0],
            },
            josh4: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF5D0],
            },
            mrsphilips1: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF6C0],
            },
            mrsphilips2: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xDF6F0],
            },
            epsilon_donated500: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xCCF58],
            },
            epsilon_donated5k: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xCCF60],
            },
            epsilon_donated10k: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xCCF68],
            },
            epsilon_robe10days: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xCCF80],
            },
            epsilon_desertdone: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x2A07E70, 0xCCF90],
            },
            bridges_address: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x22B54E8, 0x4FD48],
            },
            bridges_value: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x22B54E8, 0x4FD48, 0x10],
            },
            letters_address: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x22B54E8, 0x2B6C8],
            },
            letters_value: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x22B54E8, 0x2B6C8, 0x10],
            },
            spaceships_address: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x22B54E8, 0x1A18],
            },
            spaceships_value: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x22B54E8, 0x1A18, 0x10],
            },
            nuclear_address: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x22B54E8, 0x4B598],
            },
            nuclear_value: Variable {
                var: Watcher::new(),
                base_address,
                address_path: vec![0x22B54E8, 0x4B598, 0x10],
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
            tonya1: self.tonya1.update(process)?,
            tonya2: self.tonya2.update(process)?,
            tonya3: self.tonya3.update(process)?,
            tonya4: self.tonya4.update(process)?,
            tonya5: self.tonya5.update(process)?,
            hao1: self.hao1.update(process)?,
            paparazzo1: self.paparazzo1.update(process)?,
            paparazzo2: self.paparazzo2.update(process)?,
            paparazzo3a: self.paparazzo3a.update(process)?,
            paparazzo3b: self.paparazzo3b.update(process)?,
            paparazzo4: self.paparazzo4.update(process)?,
            omega1: self.omega1.update(process)?,
            omega2: self.omega2.update(process)?,
            barry3: self.barry3.update(process)?,
            barry3a: self.barry3a.update(process)?,
            barry3c: self.barry3c.update(process)?,
            barry4: self.barry4.update(process)?,
            extreme1: self.extreme1.update(process)?,
            extreme2: self.extreme2.update(process)?,
            extreme3: self.extreme3.update(process)?,
            extreme4: self.extreme4.update(process)?,
            fanatic3: self.fanatic3.update(process)?,
            dreyfuss1: self.dreyfuss1.update(process)?,
            thelastone: self.thelastone.update(process)?,
            barry1: self.barry1.update(process)?,
            fanatic1: self.fanatic1.update(process)?,
            abigail1: self.abigail1.update(process)?,
            abigail2: self.abigail2.update(process)?,
            epsilon1: self.epsilon1.update(process)?,
            epsilon2: self.epsilon2.update(process)?,
            epsilon3: self.epsilon3.update(process)?,
            epsilon4: self.epsilon4.update(process)?,
            epsilon5: self.epsilon5.update(process)?,
            epsilon6: self.epsilon6.update(process)?,
            epsilon7: self.epsilon7.update(process)?,
            epsilon8: self.epsilon8.update(process)?,
            barry2: self.barry2.update(process)?,
            fanatic2: self.fanatic2.update(process)?,
            rampage1: self.rampage1.update(process)?,
            rampage2: self.rampage2.update(process)?,
            rampage3: self.rampage3.update(process)?,
            rampage4: self.rampage4.update(process)?,
            rampage5: self.rampage5.update(process)?,
            hunting1: self.hunting1.update(process)?,
            hunting2: self.hunting2.update(process)?,
            minute1: self.minute1.update(process)?,
            minute2: self.minute2.update(process)?,
            minute3: self.minute3.update(process)?,
            maude1: self.maude1.update(process)?,
            nigel1: self.nigel1.update(process)?,
            nigel1a: self.nigel1a.update(process)?,
            nigel1b: self.nigel1b.update(process)?,
            nigel1c: self.nigel1c.update(process)?,
            nigel1d: self.nigel1d.update(process)?,
            nigel2: self.nigel2.update(process)?,
            nigel3: self.nigel3.update(process)?,
            josh1: self.josh1.update(process)?,
            josh2: self.josh2.update(process)?,
            josh3: self.josh3.update(process)?,
            josh4: self.josh4.update(process)?,
            mrsphilips1: self.mrsphilips1.update(process)?,
            mrsphilips2: self.mrsphilips2.update(process)?,
            epsilon_donated500: self.epsilon_donated500.update(process)?,
            epsilon_donated5k: self.epsilon_donated5k.update(process)?,
            epsilon_donated10k: self.epsilon_donated10k.update(process)?,
            epsilon_robe10days: self.epsilon_robe10days.update(process)?,
            epsilon_desertdone: self.epsilon_desertdone.update(process)?,
            bridges_address: self.bridges_address.update(process)?,
            bridges_value: self.bridges_value.update(process)?,
            letters_address: self.letters_address.update(process)?,
            letters_value: self.letters_value.update(process)?,
            spaceships_address: self.spaceships_address.update(process)?,
            spaceships_value: self.spaceships_value.update(process)?,
            nuclear_address: self.nuclear_address.update(process)?,
            nuclear_value: self.nuclear_value.update(process)?,
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
    pub tonya1: &'a Pair<i8>,
    pub tonya2: &'a Pair<i8>,
    pub tonya3: &'a Pair<i8>,
    pub tonya4: &'a Pair<i8>,
    pub tonya5: &'a Pair<i8>,
    pub hao1: &'a Pair<i8>,
    pub paparazzo1: &'a Pair<i8>,
    pub paparazzo2: &'a Pair<i8>,
    pub paparazzo3a: &'a Pair<i8>,
    pub paparazzo3b: &'a Pair<i8>,
    pub paparazzo4: &'a Pair<i8>,
    pub omega1: &'a Pair<i8>,
    pub omega2: &'a Pair<i8>,
    pub barry3: &'a Pair<i8>,
    pub barry3a: &'a Pair<i8>,
    pub barry3c: &'a Pair<i8>,
    pub barry4: &'a Pair<i8>,
    pub extreme1: &'a Pair<i8>,
    pub extreme2: &'a Pair<i8>,
    pub extreme3: &'a Pair<i8>,
    pub extreme4: &'a Pair<i8>,
    pub fanatic3: &'a Pair<i8>,
    pub dreyfuss1: &'a Pair<i8>,
    pub thelastone: &'a Pair<i8>,
    pub barry1: &'a Pair<i8>,
    pub fanatic1: &'a Pair<i8>,
    pub abigail1: &'a Pair<i8>,
    pub abigail2: &'a Pair<i8>,
    pub epsilon1: &'a Pair<i8>,
    pub epsilon2: &'a Pair<i8>,
    pub epsilon3: &'a Pair<i8>,
    pub epsilon4: &'a Pair<i8>,
    pub epsilon5: &'a Pair<i8>,
    pub epsilon6: &'a Pair<i8>,
    pub epsilon7: &'a Pair<i8>,
    pub epsilon8: &'a Pair<i8>,
    pub barry2: &'a Pair<i8>,
    pub fanatic2: &'a Pair<i8>,
    pub rampage1: &'a Pair<i8>,
    pub rampage2: &'a Pair<i8>,
    pub rampage3: &'a Pair<i8>,
    pub rampage4: &'a Pair<i8>,
    pub rampage5: &'a Pair<i8>,
    pub hunting1: &'a Pair<i8>,
    pub hunting2: &'a Pair<i8>,
    pub minute1: &'a Pair<i8>,
    pub minute2: &'a Pair<i8>,
    pub minute3: &'a Pair<i8>,
    pub maude1: &'a Pair<i8>,
    pub nigel1: &'a Pair<i8>,
    pub nigel1a: &'a Pair<i8>,
    pub nigel1b: &'a Pair<i8>,
    pub nigel1c: &'a Pair<i8>,
    pub nigel1d: &'a Pair<i8>,
    pub nigel2: &'a Pair<i8>,
    pub nigel3: &'a Pair<i8>,
    pub josh1: &'a Pair<i8>,
    pub josh2: &'a Pair<i8>,
    pub josh3: &'a Pair<i8>,
    pub josh4: &'a Pair<i8>,
    pub mrsphilips1: &'a Pair<i8>,
    pub mrsphilips2: &'a Pair<i8>,
    pub epsilon_donated500: &'a Pair<i32>,
    pub epsilon_donated5k: &'a Pair<i32>,
    pub epsilon_donated10k: &'a Pair<i32>,
    pub epsilon_robe10days: &'a Pair<i32>,
    pub epsilon_desertdone: &'a Pair<i32>,
    pub bridges_address: &'a Pair<u64>,
    pub bridges_value: &'a Pair<u64>,
    pub letters_address: &'a Pair<u64>,
    pub letters_value: &'a Pair<u64>,
    pub spaceships_address: &'a Pair<u64>,
    pub spaceships_value: &'a Pair<u64>,
    pub nuclear_address: &'a Pair<u64>,
    pub nuclear_value: &'a Pair<u64>,
}

impl<'a> Variables<'a> {
    pub fn get_as_string(var: &'a [u8]) -> Option<&'a str> {
        let null_pos = var.iter().position(|&x| x == b'\0').unwrap_or(var.len());

        std::str::from_utf8(&var[0..null_pos]).ok()
    }

    pub fn get_freak(&self, freak: &str) -> &'a Pair<i8> {
        match freak {
            "tonya1" => self.tonya1,
            "tonya2" => self.tonya2,
            "tonya3" => self.tonya3,
            "tonya4" => self.tonya4,
            "tonya5" => self.tonya5,
            "hao1" => self.hao1,
            "paparazzo1" => self.paparazzo1,
            "paparazzo2" => self.paparazzo2,
            "paparazzo3a" => self.paparazzo3a,
            "paparazzo3b" => self.paparazzo3b,
            "omega1" => self.omega1,
            "omega2" => self.omega2,
            "barry3" => self.barry3,
            "barry3a" => self.barry3a,
            "barry3c" => self.barry3c,
            "barry4" => self.barry4,
            "extreme1" => self.extreme1,
            "extreme2" => self.extreme2,
            "extreme3" => self.extreme3,
            "extreme4" => self.extreme4,
            "fanatic3" => self.fanatic3,
            "dreyfuss1" => self.dreyfuss1,
            "thelastone" => self.thelastone,
            "barry1" => self.barry1,
            "fanatic1" => self.fanatic1,
            "abigail1" => self.abigail1,
            "abigail2" => self.abigail2,
            "epsilon1" => self.epsilon1,
            "epsilon2" => self.epsilon2,
            "epsilon3" => self.epsilon3,
            "epsilon4" => self.epsilon4,
            "epsilon5" => self.epsilon5,
            "epsilon6" => self.epsilon6,
            "epsilon7" => self.epsilon7,
            "epsilon8" => self.epsilon8,
            "barry2" => self.barry2,
            "fanatic2" => self.fanatic2,
            "rampage1" => self.rampage1,
            "rampage2" => self.rampage2,
            "rampage3" => self.rampage3,
            "rampage4" => self.rampage4,
            "rampage5" => self.rampage5,
            "hunting1" => self.hunting1,
            "hunting2" => self.hunting2,
            "minute1" => self.minute1,
            "minute2" => self.minute2,
            "minute3" => self.minute3,
            "maude1" => self.maude1,
            "nigel1" => self.nigel1,
            "nigel1a" => self.nigel1a,
            "nigel1b" => self.nigel1b,
            "nigel1c" => self.nigel1c,
            "nigel1d" => self.nigel1d,
            "nigel2" => self.nigel2,
            "nigel3" => self.nigel3,
            "josh1" => self.josh1,
            "josh2" => self.josh2,
            "josh3" => self.josh3,
            "josh4" => self.josh4,
            "mrsphilips1" => self.mrsphilips1,
            "mrsphilips2" => self.mrsphilips2,
            _ => self.tonya1,
        }
    }

    pub fn get_flag(&self, flag: &str) -> &'a Pair<i32> {
        match flag {
            "donated500" => self.epsilon_donated500,
            "donated5k" => self.epsilon_donated5k,
            "donated10k" => self.epsilon_donated10k,
            "robe10days" => self.epsilon_robe10days,
            "desertdone" => self.epsilon_desertdone,
            _ => self.epsilon_donated500,
        }
    }

    pub fn get_collectible(&self, collectible: &str) -> (&'a Pair<u64>, &'a Pair<u64>) {
        match collectible {
            "bridges" => (self.bridges_address, self.bridges_value),
            "letters" => (self.letters_address, self.letters_value),
            "spaceships" => (self.spaceships_address, self.spaceships_value),
            "nuclear" => (self.nuclear_address, self.nuclear_value),
            _ => (self.bridges_address, self.bridges_value),
        }
    }
}
