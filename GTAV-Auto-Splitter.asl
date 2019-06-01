// 1.27
state("GTA5")
{
	// unsupport version
}

state("GTA5", "Steam")
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

	// percentage counter
	float percent: 0x0218FAD8, 0x18068;

	// current golf hole
	int gh: 0x1DDC004;
}

// Social Club
state("GTA5", "SocialClub")
{
	// mission counter
	int m: 0x2A07E70, 0xBDA08;

	// strangers and freaks counter
	int s: 0x2A07E70, 0xBDA20;

	// usj counter
	int u: 0x2A07E70, 0xCE5C0;

	// bridge counter
	int b: 0x2A07EC8, 0x40318;

	// random event counter
	int r: 0x2A07E70, 0xBDA28;

	// hobbies and pasttimes
	int h: 0x2A07E70, 0xBDA10;

	// current cutscene
	string255 c: 0x01CB44A0, 0xB70;
	
	// loading check
	int loading: 0x2153C30;

	// percentage counter
	float percent: 0x0218FAD8, 0x18068;

	// current golf hole
	int gh: 0x1DE3970;
}

startup
{
	// add settings groups
	settings.Add("main", true, "Main");
	settings.Add("collectibles", false, "Collectibles");
	settings.Add("misc", false, "Miscellaneous");
	settings.Add("starters", true, "Auto Starters");

	// split on Missions
	settings.Add("missions", true, "Missions", "main");

	// split on Strangers and Freaks
	settings.Add("sf", true, "Strangers and Freaks (includes Pulling Favours)", "main");

	// split on 100% completion
	settings.Add("100", false, "100% Completion", "main");
	settings.SetToolTip("100", "Split when the percentage counter reaches 100%."); 

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
	settings.Add("safari", true, "Don't Split on Hood Safari end (only works with Classic% mission order)", "misc");

	// Save Warping
	settings.Add("savewarp", true, "Don't Split when save warping (experimental)", "misc");

	// Golf autosplitter
	settings.Add("golf", false, "Split on every Golf hole", "misc");

	// Prologue timer start
	settings.Add("prologuetimer", false, "Start the timer on the Prologue start", "starters");

	// misc category auto starter
	settings.Add("misctimer", false, "Start the timer after Prologue ends", "starters");

	// Golf timer start
	settings.Add("golftimer", false, "Start the timer on the first hole in Golf", "starters");
}

init
{
	switch (modules.First().ModuleMemorySize)
	{
		case 70718464:
			version = "SocialClub";
			break;
		case 70635008:
			version = "Steam";
			break;
	}

	// Checks if name is enabled in settings and returns true if the diff is exactly one
	Func<string, int, bool> shouldSplit = (name, diff) => {
		// Check if anything changed and if this type of split is enabled
		if (diff == 0 || !settings[name]) {
			return false;
		}

		// Experimental save warping
		if (settings["savewarp"]) {
			if (diff == 1 && vars.loadHistory.Contains(name)) {
				vars.loadHistory.Remove(name);
				return false;
			}

			if (diff == -1) {
				vars.loadHistory.Add(name);
				return false;
			}
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
}

update
{
	if (version == "") {
		return false;
	}

	var oldPhase = vars.phase;
	vars.phase = timer.CurrentPhase;
	bool hasChangedPhase = oldPhase != vars.phase;

	if (vars.justStarted || vars.justSplit || hasChangedPhase) {
		vars.loadHistory.Clear();
		vars.miscFlag = false;
	}

	vars.justStarted = false;
	vars.justSplit = false;
}

start
{
	if (settings["misctimer"]) {
		if (current.c == "armenian_1_int") {
			// Keep track on vars so we only start once. Needed for loading check
			if (current.c != old.c) {
				vars.miscFlag = true;
			}

			// Finished loading for first time, start auto splitter
			if (vars.miscFlag && current.loading == 0 && current.loading != old.loading) {
				vars.miscFlag = false;
				vars.justStarted = true;
			}
		} else {
			vars.miscFlag = false;
		}
	}
	if (settings["golftimer"]) {
		vars.justStarted = current.gh == 1 && current.gh != old.gh;
	}

	if (settings["prologuetimer"]) {
		vars.justStarted = current.c != old.c && current.c == "pro_mcs_1";	
	}

	return vars.justStarted;
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

		// Guard clause to prevent split on Hood Safari end
		if (settings["safari"] && (current.m == 23)) {
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

	// check if they just reached 100% completion
	bool hundoCheck = settings["100"] && current.percent == 100 && current.percent != old.percent;

	// Golf hole split. Checks > 1 so we don't split on golf start.
	bool golfCheck = current.gh > 1 && current.gh != old.gh && vars.shouldSplit("golf", current.gh - vars.currentHole);
	// golf hole value changes to 0 inbetween holes, (walking to shot/scoreboard after hole)
	if (current.gh > 0) {
		vars.currentHole = current.gh;
	}

	// Return true if any of the above flags are true.
	vars.justSplit = missionCheck || sfCheck || stuntCheck || bridgeCheck || eventCheck || hobbyCheck || hundoCheck || golfCheck;

	return vars.justSplit;
}

isLoading
{
	return current.loading > 0;
}
