pub struct Collectible {
    pub name: &'static str,
    pub address: u64,
}

pub static COLLECTIBLES: [Collectible; 4] = [
    Collectible {
        name: "bridges",
        address: 0x4FD4,
    },
    Collectible {
        name: "letters",
        address: 0x2B6C,
    },
    Collectible {
        name: "spaceships",
        address: 0x1A1,
    },
    Collectible {
        name: "nuclear",
        address: 0x4B59,
    },
];
