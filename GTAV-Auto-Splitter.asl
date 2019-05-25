// 1.27
state("GTA5")
{
	// mission counter
	int m: 0x2A0D4B0, 0xBDA08;

	// strangers and freaks counter
	int s: 0x2A0D4B0, 0xBDA20;

	// usj counter
	int u: 0x2193E58, 0x10378;
	
	// bridge counter
	int b: 0x2A0D4B0, 0x30318;
	
	// random event counter
	int r: 0x2A0D4B0, 0xBDA28;
	
	// hobbies and pasttimes
	int h: 0x2A0D4B0, 0xBDA10;

	// next cutscene
	string255 c: 0x01CB8530, 0xB70;

	// loading check
	int loading : 0x2157FA0;
}

startup
{
	// add settings groups
	settings.Add("main", true, "Main");
	settings.Add("collectibles", false, "Collectibles");
	settings.Add("misc", false, "Miscellaneous");

	// split on Missions
	settings.Add("missions", true, "Missions", "main");

	// split on Strangers and Freaks
	settings.Add("sf", true, "Strangers and Freaks (includes Pulling Favours)", "main");

	// split on stunt jumps
	settings.Add("stuntjumps", false, "Stunt Jumps", "collectibles");
	
	// split on under the bridge
	settings.Add("bridges", false, "Under The Bridge", "collectibles");
	
	// split on Random Events
	settings.Add("randomevent", false, "Random Event", "collectibles");
	
	// split on Hobbies and Pasttimes
	settings.Add("hobbies", false, "Hobbies and Pasttimes", "collectibles");

	// Split on Prologue
	settings.Add("prologue", false, "Don't Split on Prologue", "misc");
	
	// classic%
	settings.Add("classic", true, "Don't Split during Blitz Play", "misc");
}

init
{
	// Checks if name is enabled in settings and returns true if the diff is exactly one
	Func<string, int, bool> shouldSplit = (name, diff) => {
		// Check if anything changed and if this type of split is enabled
		if (diff == 0 || !settings[name]) {
			return false;
		}

		return diff == 1;
	};
	vars.shouldSplit = shouldSplit;
}

start
{
	return current.c != old.c && current.c == "pro_mcs_1";
}

split
{
	// check if mission counter increased
	bool missionCheck = vars.shouldSplit("missions", current.m - old.m);
	if (missionCheck) {
		// Guard clause to prevent splits on prologue
		if (settings["prologue"] && current.m == 1) {
			missionCheck = false;
		}

		// Guard clause to prevent splits during Blitz play
		if (settings["classic"] && (current.m >= 27 && current.m <= 33)) {
			missionCheck = false;
		}
	}

	// check if strangers and freaks counter increased
	bool sfCheck = vars.shouldSplit("sf", current.s - old.s);

	// check if stunt jumps counter increased
	bool stuntCheck = vars.shouldSplit("stuntjumps", current.u - old.u);

	// check if bridges counter changed
	bool bridgeCheck = vars.shouldSplit("bridges", current.b - old.b);

	// check if random event increased
	bool eventCheck = vars.shouldSplit("randomevent", current.r - old.r);

	// check if hobbies and pastimes increased
	bool hobbyCheck = vars.shouldSplit("hobbies", current.h - old.h);

	// Return true if any of the above flags are true.
	bool splitNow = missionCheck || sfCheck || stuntCheck || bridgeCheck || eventCheck || hobbyCheck;

	return splitNow;
}

isLoading
{
	return current.loading > 0;
}
