pub struct Flag {
    pub name: &'static str,
    pub full_name: &'static str,
    pub id: u32,
}

pub static FLAGS: [Flag; 5] = [
    Flag {
        name: "donated500",
        full_name: "Epsilon: Donated 500$",
        id: 87,
    },
    Flag {
        name: "donated5k",
        full_name: "Epsilon: Donated 5000$",
        id: 88,
    },
    Flag {
        name: "donated10k",
        full_name: "Epsilon: Donated 10000$",
        id: 89,
    },
    Flag {
        name: "robe10days",
        full_name: "Epsilon: Wore the robes for 10 days",
        id: 92,
    },
    Flag {
        name: "desertdone",
        full_name: "Epsilon: Finished the pilgrimage in the desert",
        id: 94,
    },
];
