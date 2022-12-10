use asr::Setting;
use std::collections::HashMap;

use crate::{COLLECTIBLES, FLAGS, FREAKS, MISSIONS};
use itertools::Itertools;

pub fn get_settings() -> HashMap<&'static str, bool> {
    let mut settings: HashMap<&'static str, bool> = HashMap::from([
        (
            "auto_split",
            Setting::register("auto_split", "Split automatically", true),
        ),
        (
            "auto_start",
            Setting::register("auto_start", "Start automatically", true),
        ),
    ]);

    for mission in MISSIONS
        .entries()
        .sorted_by(|a, b| Ord::cmp(&a.1.order, &b.1.order))
    {
        settings.insert(
            mission.1.script,
            Setting::register(mission.1.script, mission.1.name, true),
        );
    }

    for freak in FREAKS
        .entries()
        .sorted_by(|a, b| Ord::cmp(&a.1.order, &b.1.order))
    {
        settings.insert(
            freak.1.script,
            Setting::register(freak.1.script, freak.1.name, true),
        );
    }

    for flag in FLAGS.iter() {
        settings.insert(
            &flag.name[..],
            Setting::register(&flag.name[..], &flag.full_name[..], false),
        );
    }

    settings.insert(
        "stunt_jumps",
        Setting::register("stunt_jumps", "Stunt Jumps", false),
    );
    settings.insert(
        "random_events",
        Setting::register("random_events", "Random Events", false),
    );

    settings.insert(
        "hobbies",
        Setting::register("hobbies", "Hobbies & Pasttimes", false),
    );

    for collectible in COLLECTIBLES {
        settings.insert(
            collectible.name,
            Setting::register(collectible.name, collectible.full_name, false),
        );
    }

    settings.insert(
        "ending_a",
        Setting::register("ending_a", "Ending A Split", false),
    );

    settings.insert(
        "golf_split",
        Setting::register("golf_split", "Split on golf hole change", false),
    );

    settings
}
