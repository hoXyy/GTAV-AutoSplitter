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

	// Split on Prologue
	settings.Add("prologue", false, "Don't Split on Prologue", "misc");
	
	// split on Random Events
	settings.Add("randomevent", false, "Random Event", "collectibles");
	
	// split on Hobbies and Pasttimes
	settings.Add("hobbies", false, "Hobbies and Pasttimes", "collectibles");
	
	// classic%
	settings.Add("classic", true, "Don't Split during Blitz Play", "misc");
}

split
{
	// check if mission counter increased
	if (settings["missions"])
	{
		if (settings["classic"])
		{
			if (current.m < 27 || current.m > 33)
			{
				// prologue setting is enabled, don't skip on prologue
				if (settings["prologue"])
				{
					if (current.m != 1)
					{
						if (current.m > old.m)
						{
							return true;
						}
					}
				}
				else
				{
					if (current.m > old.m)
					{
						return true;
					}
				}
			}
		}
		else
		// prologue setting is enabled, don't skip on prologue
			if (settings["prologue"])
			{
				if (current.m != 1)
				{
					if (current.m > old.m)
					{
						return true;
					}
				}
			}
			else
			{
				if (current.m > old.m)
				{
					return true;
				}
			}
	}

	// check if strangers and freaks counter increased
	if (settings["sf"])
	{
		if (current.s > old.s)
		{
			return true;
		}
	}

	// check if stunt jumps counter increased
	if (settings["stuntjumps"])
	{
		if (current.u > old.u)
		{
			return true;
		}
	}
	
	// check if bridges counter changed
	if (settings["bridges"])
	{
		if (current.b != old.b)
		{
			return true;
		}
	}
	
	// check if random event increased
	if (settings["randomevent"])
	{
		if (current.r > old.r)
		{
			return true;
		}
	}
	
	// check if hobbies and pastimes increased
	if (settings["hobbies"])
	{
		if (current.h > old.h)
		{
			return true;
		}
	}
	
	
}

