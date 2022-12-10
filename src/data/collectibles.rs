#[derive(Clone, Copy)]
pub struct Collectible {
    pub name: &'static str,
    pub full_name: &'static str,
    pub address: u64,
}

pub static COLLECTIBLES: [Collectible; 9] = [
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
    Collectible {
        name: "submarine",
        full_name: "Submarine Pieces",
        address: 0x132A,
    },
    Collectible {
        name: "for_sale_signs",
        full_name: "For Sale Signs",
        address: 0x8F8,
    },
    Collectible {
        name: "peyote",
        full_name: "Peyote Plants",
        address: 0x381D,
    },
    Collectible {
        name: "mosaics",
        full_name: "Monkey Mosaics",
        address: 0x1C32,
    },
    Collectible {
        name: "tracts",
        full_name: "Epsilon Tracts",
        address: 0x50D0,
    },
];
