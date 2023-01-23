using System.Collections.Generic;

namespace LiveSplit.GTAV
{
    class GTAVSplits
    {
        public static Dictionary<string, Dictionary<int, string>> Missions
        {
            get
            {
                return new Dictionary<string, Dictionary<int, string>>
                {
                    {"Trevor%", new Dictionary<int, string> {
                        {53, "Prologue"},
                        {0, "Franklin & Lamar"},
                        {1, "Repossession"},
                        {40, "Chop"},
                        {2, "Complications"},
                        {17, "Father/Son"},
                        {19, "Marriage Counseling"},
                        {44, "Friend Request"},
                        {18, "Daddy's Little Girl"},
                        {86, "Casing the Jewel Store"},
                        {89, "Carbine Rifles"},
                        {90, "The Jewel Store Job"}
                    }},
                    {"Countryside", new Dictionary<int, string> {
                        {43, "The Long Stretch"},
                        {62, "Mr. Philips"},
                        {12, "Trevor Philips Industries"},
                        {63, "Nervous Ron"},
                        {13, "Crystal Maze"},
                        {64, "Friends Reunited"}
                    }},
                    {"Blitz Play", new Dictionary<int, string> {
                        {20, "Fame or Shame"},
                        {29, "Dead Man Walking"},
                        {30, "Three's Company"},
                        {41, "Hood Safari"},
                        {71, "Scouting the Port"},
                        {21, "Did Somebody Say Yoga?"},
                        {31, "By The Book"},
                        {72, "Minisub"},
                        {32, "Blitz Play Intro"},
                        {33, "Garbage Truck"},
                        {34, "Tow Truck"},
                        {36, "Masks"},
                        {37, "Boiler Suits"},
                        {73, "Cargobob (Merryweather Heist)"},
                        {74, "Merryweather Heist (Freighter)"},
                        {75, "Merryweather Heist (Offshore)"},
                        {38, "Blitz Play Finale"}
                    }},
                    {"Deep Inside", new Dictionary<int, string> {
                        {8, "I Fought The Law"},
                        {9, "Eye in the Sky"},
                        {59, "Mr. Richards"},
                        {45, "Caida Libre"},
                        {10, "Deep Inside"}
                    }},
                    {"Paleto Score", new Dictionary<int, string> {
                        {14, "Minor Turbulence"},
                        {91, "Paleto Score Setup"},
                        {15, "Predator"},
                        {92, "Military Hardware"},
                        {93, "Paleto Score"}
                    }},
                    {"Fresh Meat", new Dictionary<int, string> {
                        {16, "Derailed"},
                        {39, "Monkey Business"},
                        {65, "Hang Ten"},
                        {76, "Surveying the Score"},
                        {46, "Bury the Hatchet"},
                        {11, "Pack Man"},
                        {47, "Fresh Meat"}
                    }},
                    {"Bureau Raid", new Dictionary<int, string> {
                        {66, "Cleaning out the Bureau"},
                        {22, "Reuniting the Family"},
                        {67, "Architect's Plans"},
                        {68, "Fire Truck (Bureau Raid)"},
                        {69, "Bureau Raid (Covert)"},
                        {70, "Bureau Raid (Roof)"}
                    }},
                    {"Third Way", new Dictionary<int, string> {
                        {60, "Ballad of Rocco"},
                        {61, "Legal Trouble"},
                        {48, "The Wrap Up"},
                        {42, "Lamar Down"},
                        {49, "Meltdown"},
                        {77, "Big Score Intro"},
                        {80, "Gauntlet A"},
                        {81, "Gauntlet B"},
                        {82, "Gauntlet C"},
                        {78, "Stingers"},
                        {79, "Driller"},
                        {83, "Sidetracked"},
                        {84, "Big Score (Subtle)"},
                        {85, "Big Score (Obvious)"}
                    }},
                    {"Lester's Assassinations", new Dictionary<int, string> {
                        {3, "The Hotel Assassination"},
                        {4, "The Multi-Target Assassination"},
                        {5, "The Vice Assassination"},
                        {6, "The Bus Assassination"},
                        {7, "The Construction Assassination"}
                    }},
                };
            }
        }

        public static Dictionary<string, Dictionary<int, string>> Freaks
        {
            get
            {
                return new Dictionary<string, Dictionary<int, string>>
                {
                    {"Franklin", new Dictionary <int,string> {
                        {58, "Pulling Favours"},
                        {59, "Pulling Another Favour"},
                        {60, "Pulling Favours Again"},
                        {61, "Still Pulling Favours"},
                        {62, "Pulling One Last Favour"},
                        {24, "Shift Work"},
                        {46, "Paparazzo"},
                        {47, "Paparazzo - The Sex Tape"},
                        {49, "Paparazzo - The Meltdown"},
                        {50, "Paparazzo - The Highness"},
                        {51, "Paparazzo - Reality Check"},
                        {44, "Far Out"},
                        {45, "The Final Frontier"},
                        {4, "Grass Roots - Franklin"},
                        {5, "Grass Roots - The Pickup"},
                        {6, "Grass Roots - The Drag"},
                        {7, "Grass Roots - The Smoke-in"},
                        {17, "Risk Assesstment"},
                        {18, "Liqudity Risk"},
                        {19, "Targeted Risk"},
                        {20, "Uncalculated Risk"},
                        {23, "Exercising Demons - Franklin"},
                        {8, "A Starlet in Vinewood"},
                        {57, "The Last One"}
                    }},
                    {"Michael", new Dictionary<int, string> {
                        {2, "Grass Roots - Michael"},
                        {21, "Exercising Demons - Michael"},
                        {0, "Death At Sea"},
                        {1, "What Lies Beneath"},
                        {9, "Seeking the Truth"},
                        {10, "Accepting the Truth"},
                        {11, "Assuming the Truth"},
                        {12, "Chasing the Truth"},
                        {13, "Bearing the Truth"},
                        {14, "Delivering the Truth"},
                        {15, "Exercising the Truth"},
                        {16, "Unknowing the Truth"}
			            // Extra Epsilon splits added later
		            }},
                    {"Trevor", new Dictionary<int, string> {
                        {3, "Grass Roots - Trevor"},
                        {22, "Exercising Demons - Trevor"},
                        {52, "Rampage 1"},
                        {53, "Rampage 2"},
                        {54, "Rampage 3"},
                        {55, "Rampage 4"},
                        {56, "Rampage 5"},
                        {25, "Target Practice"},
                        {26, "Fair Game"},
                        {32, "The Civil Border Patrol"},
                        {33, "An American Welcome"},
                        {34, "Minute Man Blues"},
                        {31, "Special Bonds"},
                        {37, "Nigel and Mrs. Thornhill"},
                        {38, "Vinewood Souvenirs - Willie"},
                        {39, "Vinewood Souvenirs - Tyler"},
                        {40, "Vinewood Souvenirs - Kerry"},
                        {41, "Vinewood Souvenirs - Mark"},
                        {42, "Vinewood Souvenirs - Al Di Napoli"},
                        {43, "Vinewood Souvenirs - The Last Act"},
                        {27, "Extra Commission"},
                        {28, "Closing the Deal"},
                        {29, "Surreal Estate"},
                        {30, "Breach of Contract"},
                        {35, "Mrs. Philips"},
                        {36, "Damaged Goods"}
                    }}
                };
            }
        }

        public static Dictionary<int, string> Collectibles
        {
            get
            {
                return new Dictionary<int, string> {
                    {0x4FD4, "Under the Bridges"},
                    {0x2B6C, "Letter Scraps"},
                    {0x1A1, "Spaceship Parts"},
                    {0x132A, "Submarine Pieces"},
                    {0x8F8, "For Sale Signs"},
                    {0x381D, "Peyote Plants"},
                    {0x1C32, "Monkey Mosaics"},
                    {0x50D0, "Epsilon Tracts"},
                    {0x4B59, "Nuclear Waste"}
                };
            }
        }

        public static Dictionary<int, string> EpsilonFlags
        {
            get
            {
                return new Dictionary<int, string>
                {
                    {87, "donated500"},
                    {88, "donated5k"},
                    {89, "donated10k"},
                    {90, "carsdelivered"},
                    {92, "robe10days"},
                    {94, "desertdone"}
                };
            }
        }

        public static Dictionary<string, string> Endings
        {
            get
            {
                return new Dictionary<string, string>
                {
                    {"fin_a_ext", "Something Sensible (Kill Trevor)"},
                    {"fin_b_ext", "The Time's Come (Kill Michael)"},
                    {"fin_ext_p2", "The Third Way (Deathwish)"}
                };
            }
        }
    }

}
