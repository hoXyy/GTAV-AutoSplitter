// GTA V Autosplitter for version 1.27
// Contributors: hoxi, TheStonedTurtle, Parik, Crab1k, Gogsi123, FriendlyBaron, burhac, Rake Jyals and other community members
// For any questions, ask in the GTA V Speedrunning Discord: https://discord.gg/3qjGGBM

// Social Club
state("GTA5")
{
	// usj counter
	int u: 0x2A07E70, 0xCE5C0;

	// random event counter
	int r: 0x2A07E70, 0xBDA28;

	// hobbies and pasttimes
	int h: 0x2A07E70, 0xBDA10;

	// current loaded cutscene
	string255 c: 0x01CB44A0, 0xB70;

	// current script
	string255 sc: 0x1CB4340;
	
	// loading check
	int loading: 0x2AC7CF4;

	// percentage counter
	float percent: 0x0218FAD8, 0x18068;

	// current golf hole
	int gh: 0x1DE3970;

	// in cutscene
	byte in_c: 0x1CB4472;

	// in mission
	byte in_m: 0x1DD6CB9;
	byte in_m_2: 0x22959C3;

	// no player control? - used in ending A splitting and countryside
	byte noControl: 0x1DD034D;

	// some kind of debug text - used for prologue autostart
	string255 debug_string: 0x2295A10, 0x0;

	// mission passed screen
	int mpassed: 0x2A07D48, 0xA60, 0x13C0;

	// collectible screen
	int collectible: 0x2AC7BA0, 0xD97A8;
}

startup
{
	vars.missionList = new Dictionary<int, string> {};
	vars.freaksList = new List<string>();

	vars.memoryWatchers = new MemoryWatcherList();
	vars.freaksWatchers = new MemoryWatcherList(); // needed to not eat up CPU

	// mission ids taken from here https://github.com/Sainan/GTA-V-Decompiled-Scripts/blob/245d1611c36454ccce2e1c047026f817f7f29f33/decompiled_scripts/standard_global_init.c#L1653
	vars.missions = new Dictionary<string, Dictionary<int, string>> {
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
			{60, "Ballad of Rocco"},
			{66, "Cleaning out the Bureau"},
			{22, "Reuniting the Family"},
			{67, "Architect's Plans"},
			{68, "Fire Truck (Bureau Raid)"},
			{69, "Bureau Raid (Covert)"},
			{70, "Bureau Raid (Roof)"}
		}},
		{"Third Way", new Dictionary<int, string> {
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

	// Add last passed mission memory watcher
	vars.memoryWatchers.Add(new MemoryWatcher<int>(new DeepPointer("GTA5.exe", 0x2A07E70, 0x85CE8)) { Name = "lastMission" });

	// GXT label
	vars.memoryWatchers.Add(new StringWatcher(new DeepPointer("GTA5.exe", 0x2A07E70, 0xAAC68), 64) { Name = "GXTLabel"});

	// Game state
	vars.memoryWatchers.Add(new MemoryWatcher<int>(new DeepPointer("GTA5.exe", 0x14167 + 0x1ca0317 + 0xA)) { Name = "GameState"});

	// Loading state
	vars.memoryWatchers.Add(new MemoryWatcher<int>(new DeepPointer("GTA5.exe", 0x1E59F4 + 0x01bea3b0 + 0xc)) { Name = "LoadState"});

	// Inserts split into settings and adds the mission to our separate list.
	Action<string, bool> addMissionChain = (missions, defaultValue) => {
		var parent = missions;
		foreach (var address in vars.missions[missions]) {
			settings.Add(address.Value, defaultValue, address.Value, parent + " segment");
			vars.missionList[address.Key] = address.Value;
		}
	};
	
	// Inserts header (i.e. mission giver) into settings.
	Action<string, bool, string> addMissionHeader = (missions, defaultValue, header) => {
		var parent = missions;
		settings.Add(parent + " segment", defaultValue, header);
		addMissionChain(missions, defaultValue);
	};


	Action<string, bool> addFreaksChain = (missions, defaultValue) => {
		var parent = missions;
		foreach (var address in vars.freaks[missions]) {
			settings.Add(address.Value, defaultValue, address.Value, parent + " segment");
			vars.freaksList.Add(address.Value);
		}
	};
	

	Action<string, bool, string> addFreaksHeader = (missions, defaultValue, header) => {
		var parent = missions;
		settings.Add(parent + " segment", defaultValue, header);
		addFreaksChain(missions, defaultValue);
	};

	vars.freaks = new Dictionary<string, Dictionary<int,string>> {
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

	// Add mission memory watchers
	foreach (var address in vars.freaks) {
		foreach (var m in address.Value) {
			vars.freaksWatchers.Add(new MemoryWatcher<byte>(new DeepPointer("GTA5.exe", 0x2A07E70, 0xDF030 + (48 * m.Key))) { Name = m.Value });
		}
	}

	vars.michaelEpsilonMissions = new Dictionary<string,string> {
		{"donated500", "Seeking the Truth (donated 500$)"},
		{"donated5k", "Accepting the Truth (donated 5000$)"},
		{"carsdelivered", "Assuming the Truth (all cars collected)"},
		{"donated10k", "Chasing the Truth (donated 10000$)"},
		{"robe10days", "Bearing the Truth (10 days with robe passed)"},
		{"desertdone", "Exercising the Truth (after pilgrimage done)"},
	};

	vars.epsilonFlags = new Dictionary<int, string> {
		{87, "donated500"},
		{88, "donated5k"},
		{89, "donated10k"},
		{90, "carsdelivered"},
		{92, "robe10days"},
		{94, "desertdone"}
	};

	foreach(var flag in vars.epsilonFlags) {
		vars.memoryWatchers.Add(new MemoryWatcher<int>(new DeepPointer("GTA5.exe", 0x2A07E70, 0xCCCA0 + (flag.Key * 8))) { Name = flag.Value });
	}


	vars.endings = new Dictionary<string,string> {
		{"fin_a_ext", "Something Sensible (Kill Trevor)"},
		{"fin_b_ext", "The Time's Come (Kill Michael)"},
		{"fin_ext_p2", "The Third Way (Deathwish)"}
	};

	// used to add to settings, second string is the cutscene's name
	vars.cutsceneNames = new Dictionary<string,List<string>> {
		{"pro_mcs_5", new List<string> {"Prologue: Getting in getaway vehicle", "Trevor%"}},
		{"pro_mcs_6", new List<string> {"Prologue: The Train", "Trevor%"}},
		// no cutscene id for post drive in F&L, can be circumvented by checking for cutscene = armenian_1_int and counting the number of times player loses control
		{"armenian_2_int", new List<string> {"Start Reposession", "Trevor%"}},
		{"arm_2_mcs_4", new List<string> {"Reposession: Confronted by Vagos", "Trevor%"}},
		{"tonya_mcs_1", new List<string> {"Start Pulling Favors", "Trevor%"}},
		{"fra_0_int", new List<string> {"Start Chop", "Trevor%"}},
		{"fra_0_mcs_1", new List<string> {"Chop: Confront D in Alley", "Trevor%"}},
		{"fra_0_mcs_4_p2_t3", new List<string> {"Chop: Put D in Van", "Trevor%"}},
		{"armenian_3_int", new List<string> {"Start Complications", "Trevor%"}},
		{"armenian_3_mcs_8", new List<string> {"Complications: Start Simeon Fight", "Trevor%"}},
		// fatherson: getting franklin on boat sets noControl to 1
		{"fam_1_mcs_2", new List<string> {"Father/Son: Arrive at LSC", "Trevor%"}},
		{"family_3_int", new List<string> {"Start Marriage Counseling", "Trevor%"}},
		{"fam_3_mcs_1", new List<string> {"Marriage Counseling: Pull down house", "Trevor%"}},
		{"lester_1_int", new List<string> {"Start Friend Request", "Trevor%"}},
		{"les_1a_mcs_0", new List<string> {"Friend Request: Clothes Store", "Trevor%"}},
		{"les_1a_mcs_1", new List<string> {"Friend Request: Enter Lifeinvader", "Trevor%"}},
		{"les_1a_mcs_3", new List<string> {"Friend Request: Finish popups", "Trevor%"}},
		{"family_2_mcs_2", new List<string> {"DLG: Finish Bike Ride", "Trevor%"}},
		{"family_2_mcs_3", new List<string> {"DLG: Finish Swimming", "Trevor%"}},
		{"jh_1_ig_3", new List<string> {"Casing the Jewel Store: Greet Employee", "Trevor%"}},
		{"jh2_fina_mcs4_a1a2", new List<string> {"Jewel Store Job: Leave Jewel Store", "Trevor%"}},
		{"lamar_1_int", new List<string> {"Start The Long Stretch", "Misc"}},
		{"lam_1_mcs_1_concat", new List<string> {"The Long Stretch: Meet D", "Misc"}},
		{"trv_1_mcs_1_p1", new List<string> {"Mr. Phillips: Finish First Drive", "Countryside"}},
		{"trv_1_mcs_3_concat", new List<string> {"Mr. Phillips: Smash Trailer", "Countryside"}},
		{"chinese_1_int", new List<string> {"Start TPI", "Countryside"}},
		{"trevor_2_int", new List<string> {"Start Nervous Ron", "Countryside"}},
		{"trv_2_mcs_4_concat", new List<string> {"Nervous Ron: Shoot Down Chopper", "Countryside"}},
		{"trv_2_mcs_6", new List<string> {"Nervous Ron: Board Plane", "Countryside"}},
		{"chinese_2_int", new List<string> {"Start Crystal Maze", "Countryside"}},
		{"trevor_drive_int", new List<string> {"Start Friends Reunited", "Countryside"}},
		{"trv_dri_mcs_concat", new List<string> {"Friends Reunited: Look over Los Santos", "Countryside"}},
		{"family_4_mcs_2", new List<string> {"Fame or Shame: Enter Arena", "Blitz Play"}},
		{"fbi_1_int", new List<string> {"Start Dead Man Walking", "Blitz Play"}},
		{"family_5_mcs_1", new List<string> {"Yoga: Finish 1st Pose", "Blitz Play"}},
		{"family_5_mcs_2", new List<string> {"Yoga: Finish 2nd Pose", "Blitz Play"}},
		{"family_5_mcs_3", new List<string> {"Yoga: Finish 3rd Pose", "Blitz Play"}},
		{"family_5_mcs_4", new List<string> {"Yoga: Meet Jimmy", "Blitz Play"}},
		{"family_5_mcs_5", new List<string> {"Yoga: Greet Drug Dealer", "Blitz Play"}},
		{"fam_5_mcs_6", new List<string> {"Yoga: Hallucination Sequence", "Blitz Play"}},
		{"fbi_2_int", new List<string> {"Three's Company: Meet Dave", "Blitz Play"}},
		{"fbi_2_mcs_1", new List<string> {"Three's Company: Arrive at El Burro", "Blitz Play"}},
		{"fbi_2_mcs_2", new List<string> {"Three's Company: Start Rapelling", "Blitz Play"}},
		{"fbi_2_mcs_3_p1a", new List<string> {"Three's Company: Break Glass", "Blitz Play"}},
		{"fbi_2_mcs_3b", new List<string> {"Three's Company: Finish Building Shootout", "Blitz Play"}},
		{"lsdh_int", new List<string> {"Start Scouting the Port", "Blitz Play"}},
		{"lsdhs_mcs_2", new List<string> {"Scouting the Port: Arrive at Security Gate", "Blitz Play"}},
		{"franklin_1_int", new List<string> {"Start Hood Safari", "Blitz Play"}},
		{"fra_1_mcs_1", new List<string> {"Hood Safari: The Deal", "Blitz Play"}},
		{"fbi_3_int", new List<string> {"Start By the Book", "Blitz Play"}},
		{"fbi_3_mcs_1", new List<string> {"By the Book: Finish Interrogation 1", "Blitz Play"}},
		{"fbi_3_mcs_2", new List<string> {"By the Book: Arrive at house", "Blitz Play"}},
		{"fbi_3_mcs_7", new List<string> {"By the Book: Finish the Assassination", "Blitz Play"}},
		{"ass_int_2_alt1", new List<string> {"Start Hotel Assassination", "Misc"}},
		{"lsdh_2a_int", new List<string> {"Start Merryweather Heist", "Blitz Play"}},
		{"sol_1_mcs_2", new List<string> {"Mr. Richards: Confront Rocco", "Deep Inside"}},
		{"sol_1_mcs_3", new List<string> {"Mr. Richards: Finish Rocco Fight", "Deep Inside"}},
		{"martin_1_mcs_1", new List<string> {"Caida Libre: Get Files", "Deep Inside"}},
		{"exl_1_mcs_1_p3_b", new List<string> {"Minor Turbulence: Enter Cargo Jet", "Paleto Score"}},
		{"rbhs_mcs_1", new List<string> {"Paleto Score Setup: Enter Paleto Bay", "Paleto Score"}},
		{"rbh_2a_mcs_2_p7", new List<string> {"Paleto Score: Confront Cops", "Paleto Score"}},
		{"rbh_2ab_mcs_6", new List<string> {"Paleto Score: Forced into Alley", "Paleto Score"}},
		{"rbh_2a_mcs_4", new List<string> {"Paleto Score: Franklin Meets Michael", "Paleto Score"}},
		{"fbi_5_int", new List<string> {"Start Monkey Business", "Fresh Meat"}},
		{"fbi_5a_mcs_1", new List<string> {"Monkey Business: Leave Water", "Fresh Meat"}},
		{"fbi_5b_mcs_1", new List<string> {"Monkey Business: Pick up Container", "Fresh Meat"}},
		{"fbi_5a_mcs_10", new List<string> {"Monkey Business: Leave Helicopter", "Fresh Meat"}},
		{"trv_5_int", new List<string> {"Start Hang Ten", "Fresh Meat"}},
		{"bss_1_mcs_2", new List<string> {"Survey the Score: Franklin and Michael Case the Back Enterance", "Fresh Meat"}},
		{"bss_1_mcs_3", new List<string> {"Survey the Score: Security Trucks Enter Depository", "Fresh Meat"}},
		{"mic_1_int", new List<string> {"Start Bury the Hatchet", "Fresh Meat"}},
		{"mic_1_mcs_1", new List<string> {"Bury the Hatchet: Arrive at Airport", "Fresh Meat"}},
		{"mic_1_mcs_2", new List<string> {"Bury the Hatchet: Confront Trevor", "Fresh Meat"}},
		{"mic_1_mcs_3", new List<string> {"Bury the Hatchet: Attempt to Drive Away", "Fresh Meat"}},
		{"car_5_mcs_1", new List<string> {"Pack Man: Start Packer Drive", "Fresh Meat"}},
		{"fam_6_mcs_1", new List<string> {"Reuniting the Family: Confront Amanda", "Bureau Raid"}},
		{"fam_6_mcs_2_concat", new List<string> {"Reuniting the Family: Enter Tatoo Shop", "Bureau Raid"}},
		{"fam_6_mcs_3", new List<string> {"Reuniting the Family: Finish Piercings", "Bureau Raid"}},
		{"fam_6_mcs_4", new List<string> {"Reuniting the Family: Finish Tattoo", "Bureau Raid"}},
		{"fam_6_mcs_5", new List<string> {"Reuniting the Family: Cut Hair", "Bureau Raid"}},
		{"fam_6_mcs_6", new List<string> {"Reuniting the Family: Arrive at Therapy", "Bureau Raid"}},
		{"ah_3b_mcs_1", new List<string> {"Bureau Raid: Jump out of Heli", "Bureau Raid"}},
		{"ah_3b_mcs_3", new List<string> {"Bureau Raid: Start Hack", "Bureau Raid"}},
		{"ah_3b_mcs_4", new List<string> {"Bureau Raid: Finish Hack", "Bureau Raid"}},
		{"ah_3b_mcs_5", new List<string> {"Bureau Raid: Finish Shootout", "Bureau Raid"}},
		{"ah_3b_mcs_6_p1", new List<string> {"Bureau Raid: Pass Crashed Helicopter", "Bureau Raid"}},
		{"ah_3b_mcs_7", new List<string> {"Bureau Raid: Start Repelling", "Bureau Raid"}},
		{"mic_3_int", new List<string> {"The Wrap Up: Meet Dave", "Third Way"}},
		{"bs_2a_mcs_1", new List<string> {"Big Score Subtle: Hijack security trucks", "Third Way"}},
		{"bs_2a_mcs_8_p3", new List<string> {"Big Score Subtle: Finish shootout", "Third Way"}},
		{"bs_2a_mcs_11", new List<string> {"Big Score Subtle: Finish Drive Sequence", "Third Way"}},
		{"fin_c_int", new List<string> {"Start The Third Way", "Third Way"}},
		{"fin_c_mcs_1", new List<string> {"The Third Way: Start Foundry Shootout", "Third Way"}},
		{"fin_c_ext", new List<string> {"The Third Way: Finish Foundry Shootout", "Third Way"}}, // noControl indicates character switch after this?
		{"fin_c2_mcs_5", new List<string> {"The Third Way: Kidnap Devin", "Third Way"}},
		{"bar_5_rcm_p2", new List<string> {"Grass Roots: Franklin", "Misc"}},
		{"pap_2_rcm_p2", new List<string> {"Start Paparazzo: The Sex Tape", "Misc"}},
		{"pap_3_rcm", new List<string> {"Paparazzo: The Proposal", "Misc"}},
		{"es_3_rcm", new List<string> {"Start Targeted Risk", "Misc"}}
		
	};

	vars.collectibleIDs = new Dictionary<int, string> {
		{0x4FD4, "Under the Bridges"},
		{0x2B6C, "Letter Scraps"},
		{0x1A1, "Spaceship Parts"},
		{0x4B59, "Nuclear Waste"}
	};
	
	vars.noCollectibleStates = new Dictionary<string,string> {
		{"franklin1", "Hood Safari"},
		{"bss_1_mcs_2", "Surveying the Score: Finding/Tracking Security Vans"},
		{"assassin_valet", "The Hotel Assassination"},
		{"assassin_hooker", "The Vice Assassination"},
		{"pilot_school", "Flight School"},
		{"golf", "Golf"}
	};

	foreach(var collectible in vars.collectibleIDs) {
		vars.memoryWatchers.Add(new MemoryWatcher<ulong>(new DeepPointer("GTA5.exe", 0x22B54E0 + 8, collectible.Key * 16 + 8)) { Name = collectible.Value + " address" });
		vars.memoryWatchers.Add(new MemoryWatcher<ulong>(new DeepPointer("GTA5.exe", 0x22B54E0 + 8, collectible.Key * 16 + 8, 0x10)) { Name = collectible.Value + " value" });
	}



	// add settings groups
	settings.Add("main", true, "Main");
	settings.Add("collectibles", false, "Collectibles");
	settings.Add("misc", false, "Miscellaneous");
	settings.Add("starters", true, "Auto Starters");
	settings.Add("timerend", true, "Auto Finishers");
 	settings.Add("cutscene", false, "Cutscenes");


	// Add missions to setting list
	settings.Add("missions", true, "Missions", "main");
	settings.CurrentDefaultParent = "missions";
	addMissionHeader("Trevor%", true, "Trevor%");
	addMissionHeader("Countryside", true, "Countryside");
	addMissionHeader("Blitz Play", true, "Blitz Play");
	addMissionHeader("Deep Inside", true, "Deep Inside");
	addMissionHeader("Paleto Score", true, "Paleto Score");
	addMissionHeader("Fresh Meat", true, "Fresh Meat");
	addMissionHeader("Bureau Raid", true, "Bureau Raid");
	addMissionHeader("Third Way", true, "Third Way");
	addMissionHeader("Lester's Assassinations", true, "Lester's Assassinations");


	// Add strangers and freaks to setting list
	settings.Add("sf", true, "Strangers and Freaks", "main");
	settings.CurrentDefaultParent = "sf";
	addFreaksHeader("Franklin", true, "Franklin");
	addFreaksHeader("Michael", true, "Michael");
	addFreaksHeader("Trevor", true, "Trevor");

	foreach (var mission in vars.michaelEpsilonMissions) {
		settings.Add(mission.Key, true, mission.Value, "Michael segment");
	}

	// split on stunt jumps
	settings.Add("stuntjumps", false, "Stunt Jumps", "collectibles");
	// split on Random Events
	settings.Add("randomevent", false, "Random Event", "collectibles");
	// split on Hobbies and Pasttimes
	settings.Add("hobbies", false, "Hobbies and Pasttimes", "collectibles");
	// don't split on these during specific missions/parts of missions
	settings.Add("no_collect", false, "Don't split during:", "collectibles");

	foreach(var collectible in vars.collectibleIDs) {
		settings.Add(collectible.Value, false, collectible.Value, "collectibles");
	}

	foreach(var state in vars.noCollectibleStates) {
		settings.Add(state.Key + "_noc", false, state.Value, "no_collect");
	}
	
	// Golf autosplitter
	settings.Add("golf", false, "Split on every Golf hole", "misc");

	// Option to increase refresh rate
	settings.Add("refreshRate", false, "Refresh Rate Settings", "misc");
	settings.Add("highRefreshRate", false, "Increase script refresh rate (higher CPU load)", "refreshRate");
	settings.SetToolTip("highRefreshRate", "Checks to determine whether to increase splitting accuracy. Enabling this setting will use more processing power because code is running more often.");
	settings.Add("lowRefreshRate", false, "Decrease script refresh rate (lower CPU load)", "refreshRate");
	settings.SetToolTip("lowRefreshRate", "Checks to determine whether to decrease splitting accuracy. Enabling this setting will make LiveSplit use a bit less CPU.");
	settings.Add("updateFreaksWatchers", false, "Segment start double split fix (Increases CPU load)", "misc");
	settings.SetToolTip("updateFreaksWatchers", "This setting updates the S&F watchers every update cycle (default 60 times a second, dependant on the refresh rate settings). \nThis semi-heavily increases LiveSplit's CPU usage. Test if your in-game FPS is good enough if you enable this setting.");

	vars.segmentsStart = new Dictionary<string,string> {
		{"countryside", "Countryside"},
		{"fam_4_int_alt1", "Blitz Play"},
		{"car_1_int_concat", "Deep Inside"},
		{"paleto_score" , "Paleto Score"},
		{"exile_3_int", "Fresh Meat"},
		{"ah_1_int", "Bureau Raid"},
		{"sol_2_int_alt1", "Third Way"}
	};

	// Add segments to autostart
	settings.Add("segments_start", false, "Segments", "starters");
	settings.SetToolTip("segments_start", "For Trevor% segment, use the Start the timer on the Prologue start option.");
	settings.Add("segments_split", false, "Split at the beginning of segments", "cutscene");

	// Add actual segments to starter, cutscene splits
	foreach(var Segment in vars.segmentsStart) {
		settings.Add(Segment.Key + "start", true, Segment.Value, "segments_start");
		settings.Add(Segment.Key + "split", false, Segment.Value, "segments_split");
	};

	// Add cutscenes to setting list
	settings.Add("Trevor%_c", false, "Trevor%", "cutscene");
	settings.Add("Countryside_c", false, "Countryside", "cutscene");
	settings.Add("Blitz Play_c", false, "Blitz Play", "cutscene");
	settings.Add("Deep Inside_c", false, "Deep Inside", "cutscene");
	settings.Add("Paleto Score_c", false, "Paleto Score", "cutscene");
	settings.Add("Fresh Meat_c", false, "Fresh Meat", "cutscene");
	settings.Add("Bureau Raid_c", false, "Bureau Raid", "cutscene");
	settings.Add("Third Way_c", false, "Third Way", "cutscene");
	settings.Add("Misc_c", false, "Other", "cutscene");
	
	foreach (var cutscene in vars.cutsceneNames) {
		settings.Add(cutscene.Key, false, cutscene.Value[0], cutscene.Value[1] + "_c");
	};

	// Prologue timer start
	settings.Add("prologuetimer", true, "Start the timer on the Prologue start", "starters");

	// misc category auto starter
	settings.Add("misctimer", false, "Start the timer after Prologue ends", "starters");

	// Golf timer start
	settings.Add("golftimer", false, "Start the timer on the first hole in Golf", "starters");

	// Endings timer end
	settings.Add("endings", true, "Endings", "timerend");

	// Add endings to settings
	foreach(var Ending in vars.endings) {
		settings.Add(Ending.Key, true, Ending.Value, "endings");
	};

	settings.Add("segments_end", false, "Segments", "timerend");
	settings.SetToolTip("segments_end", "For Third Way segment, use the Ending C auto finisher.");

	// used only for settings, will need to check for them manually
	vars.segmentsEnd = new Dictionary<string,string> {
		{"trevis", "Trevor%"},
		{"country_end", "Countryside"},
		{"blitz_end", "Blitz Play"},
		{"deep", "Deep Inside"},
		{"paleto_end", "Paleto Score"},
		{"fresh_meat_end", "Fresh Meat"},
		{"bureau_end", "Bureau Raid"},
		{"epsilon_end", "Epsilon Program"},
		{"asf_end", "All Strangers and Freaks"}
	};

	// Add segment ends to settings list
	foreach(var Segment in vars.segmentsEnd) {
		settings.Add(Segment.Key, true, Segment.Value, "segments_end");
	};
}


init
{
	// Checks if name is enabled in settings and returns true if the diff is exactly one
	Func<string, int, bool> shouldSplit = (name, diff) => {
		// Check if anything changed and if this type of split is enabled, probably can get removed
		if (diff == 0 || !settings[name]) {
			return false;
		}

		return diff == 1;
	};
	vars.shouldSplit = shouldSplit;

	vars.miscFlag = false;
	vars.justStarted = false;
	vars.justSplit = false;
	vars.phase = timer.CurrentPhase;
	vars.loadHistory = new HashSet<string>();
	vars.currentHole = 1;

/* 	vars.freaksUpdateThread = new Thread(() => {
		while(true) {
			Thread.Sleep(2000);
			vars.freaksWatchers.UpdateAll(game);
		}
	});

	vars.freaksUpdateThread.Start(); */

	vars.splits = new List<string>();
}

update
{
	var oldPhase = vars.phase;
	vars.phase = timer.CurrentPhase;
	bool hasChangedPhase = oldPhase != vars.phase;

	vars.memoryWatchers.UpdateAll(game);
	if (settings["updateFreaksWatchers"])
	{
		vars.freaksWatchers.UpdateAll(game);
	}

	if (vars.memoryWatchers["GXTLabel"].Current != vars.memoryWatchers["GXTLabel"].Old)
	{
		vars.freaksWatchers.UpdateAll(game);
	}

	if (vars.memoryWatchers["GameState"].Current != vars.memoryWatchers["GameState"].Old)
	{
		vars.freaksWatchers.UpdateAll(game);
	}

	if (vars.justStarted || vars.justSplit || hasChangedPhase) {
		vars.loadHistory.Clear();
		vars.miscFlag = false;
		if (!vars.justSplit) {
			vars.splits.Clear();
		}
	}

	vars.justStarted = false;
	vars.justSplit = false;

	if (settings["highRefreshRate"]) {
    	refreshRate = 120;
	} 
	if (settings["lowRefreshRate"]) {
    	refreshRate = 30;
	}
	else {
    	refreshRate = 60;
	}
	
	if (current.in_c == 1) {
		vars.lastExecutedCutscene = current.c;
	};
	
	// Don't split if a load is going on
	if (vars.memoryWatchers["LoadState"].Current == 0) {
		// splitting stuff
		// check if stunt jumps counter increased
		if (vars.shouldSplit("stuntjumps", current.u - old.u)) {
			vars.justSplit = true;
		};

		// check if random event increased
		if (vars.shouldSplit("randomevent", current.r - old.r)) {
			vars.justSplit = true;
		};

		// check if hobbies and pastimes increased
		if (vars.shouldSplit("hobbies", current.h - old.h)) {
			vars.justSplit = true;
		};
		
		// check if split on this ending/cutscene
		if (settings.ContainsKey(current.c) && settings[current.c] && current.in_c == 1 && current.in_c != old.in_c && current.in_m == 1) {
			vars.justSplit = true;
		};

		// ending A check
		if (settings["fin_a_ext"] && current.c == "fin_a_ext" && current.noControl == 1 && current.noControl != old.noControl && current.in_m == 1) {
			vars.justSplit = true;
		};

		// generic segment timer split
		if (settings.ContainsKey(current.c + "split") && settings[current.c + "split"]) {
			if (current.in_c == 0 && current.in_c != old.in_c && current.in_m == 1) {
				vars.justSplit = true;
			}
		}

		// exception for countryside
		if (settings["countrysidesplit"]) {
			if (current.c == "trevor_1_int" && current.in_m == 1 && current.in_c == 0 && current.loading == 0 && current.loading != old.loading && current.noControl == 0) {
				vars.justSplit = true;
			}
		}

		// exception for paleto score
		if (settings["paleto_scoresplit"]) {
			if (current.sc == "exile1" && current.loading == 0 && current.loading != old.loading && current.in_m == 1) {
				vars.justSplit = true;
			}
		}

		// Golf hole split. Checks > 1 so we don't split on golf start.
		if (current.gh > 1 && current.gh != old.gh && vars.shouldSplit("golf", current.gh - vars.currentHole)) {
			vars.justSplit = true;
		};
		// golf hole value changes to 0 inbetween holes, (walking to shot/scoreboard after hole)
		if (current.gh > 0) {
			vars.currentHole = current.gh;
		}

		// Segment end splits
		// Trevor%
		if (settings["trevis"] && current.mpassed == 1 && current.mpassed != old.mpassed && current.sc == "jewelry_heist") {
			vars.justSplit = true;
		};

		// Countryside
		if (settings["country_end"] && current.mpassed == 1 && current.mpassed != old.mpassed && current.sc == "trevor3") {
			vars.justSplit = true;
		};

		// Blitz Play
		if (settings["blitz_end"] && current.mpassed == 1 && current.mpassed != old.mpassed && current.sc == "fbi4") {
			vars.justSplit = true;
		};
		
		// Deep Inside
		if (settings["deep"] && current.mpassed == 1 && current.mpassed != old.mpassed && current.sc == "carsteal3") {
			vars.justSplit = true;
		};

		// Paleto Score
		if (settings["paleto_end"] && current.mpassed == 1 && current.mpassed != old.mpassed && current.sc == "rural_bank_heist") {
			vars.justSplit = true;
		}; 

		// Fresh Meat
		if (settings["fresh_meat_end"] && current.mpassed == 1 && current.mpassed != old.mpassed && current.sc == "michael2") {
			vars.justSplit = true;
		};

		// Bureau Raid
		if (settings["bureau_end"] && current.mpassed == 1 && current.mpassed != old.mpassed && current.sc.StartsWith("agency_heist3")) {
			vars.justSplit = true;
		};

		// Epsilon Program
		if (settings["epsilon_end"] && current.mpassed == 1 && current.mpassed != old.mpassed && current.sc == "epsilon8") {
			vars.justSplit = true;
		};

		// All Strangers and Freaks
		if (settings["asf_end"] && current.mpassed == 1 && current.mpassed != old.mpassed && current.sc == "fanatic1") {
			vars.justSplit = true;
		};

		if (current.in_m_2 == 0 || !((settings.ContainsKey(vars.lastExecutedCutscene + "_noc") && settings[vars.lastExecutedCutscene + "_noc"]) || (settings.ContainsKey(current.sc + "_noc") && settings[current.sc + "_noc"]))) { //todo: optimize for quicker execution
			foreach (var collectible in vars.collectibleIDs) {
				vars.currentValue = (vars.memoryWatchers[collectible.Value + " address"].Current + 0x10 & 0xFFFFFFFF) ^ vars.memoryWatchers[collectible.Value + " value"].Current;
				vars.oldValue = (vars.memoryWatchers[collectible.Value + " address"].Old + 0x10 & 0xFFFFFFFF) ^ vars.memoryWatchers[collectible.Value + " value"].Old;
				if ((vars.currentValue > vars.oldValue) && settings[collectible.Value])
				{
					vars.justSplit = true;
				}
			}
		};


		foreach (var flag in vars.epsilonFlags) {
			if (vars.memoryWatchers[flag.Value].Current > vars.memoryWatchers[flag.Value].Old) {
				if (settings[flag.Value] && !vars.splits.Contains(flag.Value)) {
					vars.justSplit = true;
					vars.splits.Add(flag.Value);
				}
			}
		};

		foreach (var mission in vars.missionList) {
			if (vars.memoryWatchers["lastMission"].Current == mission.Key && vars.memoryWatchers["lastMission"].Current != vars.memoryWatchers["lastMission"].Old) {
				if (settings[mission.Value] && !vars.splits.Contains(mission.Value)) {
					vars.justSplit = true;
					vars.splits.Add(mission.Value);
				}
			}
		};

		foreach (var freaks in vars.freaksList) {
			bool curVal = Convert.ToBoolean((vars.freaksWatchers[freaks].Current >> 3) & 1);
			bool oldVal = Convert.ToBoolean((vars.freaksWatchers[freaks].Old >> 3) & 1);
			if (curVal && !oldVal) {
				if (settings.ContainsKey(freaks) && settings[freaks] && !vars.splits.Contains(freaks)) {
					vars.justSplit = true;
					vars.splits.Add(freaks);
				}
			}
		}
	}

}

start
{
	bool startFlag = false;
	if (settings["misctimer"]) {
		if (current.c == "armenian_1_int" && current.in_c == 1 && current.in_c != old.in_c) {
			startFlag = true;
		}	
	}


	// generic segment timer start
	if (settings.ContainsKey(current.c + "start") && settings[current.c + "start"]) {
		if (current.in_c == 0 && current.in_c != old.in_c && current.in_m == 1) {
			startFlag = true;
		}
	}

	// exception for countryside
	if (settings["countrysidestart"]) {
		if (current.c == "trevor_1_int" && current.in_m == 1 && current.in_c == 0 && current.loading == 0 && current.loading != old.loading && current.noControl == 0) {
			startFlag = true;
		}
	}

	// exception for paleto score
	if (settings["paleto_scorestart"]) {
		if (current.sc == "exile1" && current.loading == 0 && current.loading != old.loading && current.in_m == 1) {
			startFlag = true;
		}
	}

	bool golfFlag = settings["golftimer"] && current.gh == 1 && current.gh != old.gh;
	
	bool prologueFlag = settings["prologuetimer"] && current.debug_string == "PRO_SETTING" && current.debug_string != old.debug_string;

	vars.justStarted = startFlag || golfFlag || prologueFlag;

	return vars.justStarted;
}

split
{
	return vars.justSplit;
}
