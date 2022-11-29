#[derive(Clone, Copy)]
pub struct Collectible {
    pub name: &'static str,
    pub full_name: &'static str,
    pub address: u64,
}

pub static COLLECTIBLES: [Collectible; 4] = [
    Collectible {
        name: "bridges",
        full_name: "Under the Bridges",
        address: 0x4FD4,
    },
    Collectible {
        name: "letters",
        full_name: "Letter Scraps",
        address: 0x2B6C,
    },
    Collectible {
        name: "spaceships",
        full_name: "Spaceship Parts",
        address: 0x1A1,
    },
    Collectible {
        name: "nuclear",
        full_name: "Nuclear Waste",
        address: 0x4B59,
    },
];
