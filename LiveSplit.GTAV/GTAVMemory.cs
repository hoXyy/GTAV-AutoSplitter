using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using LiveSplit.ComponentUtil;
using LiveSplit.UI.Components;

namespace LiveSplit.GTAV
{
    class GTAVData
    {
        public MemoryWatcher<int> StuntJumps { get; }
        public MemoryWatcher<int> RandomEvents { get; }
        public MemoryWatcher<int> Hobbies { get; }
        public StringWatcher CurrentCutscene { get; }
        public StringWatcher CurrentScript { get; }
        public MemoryWatcher<int> Loading { get; }
        public MemoryWatcher<float> CompletionPercent { get; }
        public MemoryWatcher<int> GolfHole { get; }
        public MemoryWatcher<byte> InCutscene { get; }
        public MemoryWatcher<byte> InMission { get; }
        public MemoryWatcher<byte> InMission2 { get; }
        public MemoryWatcher<byte> NoPlayerControl { get; }
        public StringWatcher DebugString { get; }
        public MemoryWatcher<int> MissionPassedScreen { get; }
        public MemoryWatcher<int> CollectibleScreen { get; }
        public MemoryWatcher<int> LastMissionPassed { get; }
        public MemoryWatcher<int> LoadState { get; }
        public StringWatcher GXTLabel { get; }
        public MemoryWatcher<int> GameState { get; }
        public MemoryWatcherList FreaksWatchers { get; }
        public MemoryWatcherList CollectibleWatchers { get; }
        public MemoryWatcherList FlagWatchers { get; }

        public GTAVData(long baseOffset)
        {
            // Main variables
            StuntJumps = new MemoryWatcher<int>(new DeepPointer((IntPtr)baseOffset, 0x2A07E70, 0xCE5C0));
            RandomEvents = new MemoryWatcher<int>(new DeepPointer((IntPtr)baseOffset, 0x2A07E70, 0xBDA28));
            Hobbies = new MemoryWatcher<int>(new DeepPointer((IntPtr)baseOffset, 0x2A07E70, 0xBDA10));
            CurrentCutscene = new StringWatcher(new DeepPointer((IntPtr)baseOffset, 0x01CB44A0, 0xB70), 128);
            CurrentScript = new StringWatcher(new DeepPointer((IntPtr)baseOffset, 0x1CB4340), 128);
            Loading = new MemoryWatcher<int>(new DeepPointer((IntPtr)baseOffset, 0x2AC7CF4));
            CompletionPercent = new MemoryWatcher<float>(new DeepPointer((IntPtr)baseOffset, 0x0218FAD8, 0x18068));
            GolfHole = new MemoryWatcher<int>(new DeepPointer((IntPtr)baseOffset, 0x1DE3970));
            InCutscene = new MemoryWatcher<byte>(new DeepPointer((IntPtr)baseOffset, 0x1CB4472));
            InMission = new MemoryWatcher<byte>(new DeepPointer((IntPtr)baseOffset, 0x1DD6CB9));
            InMission2 = new MemoryWatcher<byte>(new DeepPointer((IntPtr)baseOffset, 0x22959C3));
            NoPlayerControl = new MemoryWatcher<byte>(new DeepPointer((IntPtr)baseOffset, 0x1DD034D));
            DebugString = new StringWatcher(new DeepPointer((IntPtr)baseOffset, 0x2295A10, 0x0), 128);
            MissionPassedScreen = new MemoryWatcher<int>(new DeepPointer((IntPtr)baseOffset, 0x2A07D48, 0xA60, 0x13C0));
            CollectibleScreen = new MemoryWatcher<int>(new DeepPointer((IntPtr)baseOffset, 0x2AC7BA0, 0xD97A8));
            LastMissionPassed = new MemoryWatcher<int>(new DeepPointer((IntPtr)baseOffset, 0x2A07E70, 0x85CE8));
            LoadState = new MemoryWatcher<int>(new DeepPointer((IntPtr)baseOffset, 0x1DCFDB0));
            GXTLabel = new StringWatcher(new DeepPointer((IntPtr)baseOffset, 0x2A07E70, 0xAAC68), 64);
            GameState = new MemoryWatcher<int>(new DeepPointer((IntPtr)baseOffset, 0x1CB4488));

            // Memory watcher lists
            FreaksWatchers = new MemoryWatcherList();
            CollectibleWatchers = new MemoryWatcherList();
            FlagWatchers = new MemoryWatcherList();

            // Add freaks watchers
            foreach (var header in GTAVSplits.Freaks)
            {
                foreach (var freak in header.Value)
                {
                    FreaksWatchers.Add(new MemoryWatcher<byte>(new DeepPointer((IntPtr)baseOffset, 0x2A07E70, 0xDF030 + (48 * freak.Key))) { Name = freak.Value });
                }
            }

            // Add collectible watchers
            foreach (var collectible in GTAVSplits.Collectibles)
            {
                CollectibleWatchers.Add(new MemoryWatcher<ulong>(new DeepPointer((IntPtr)baseOffset, 0x22B54E0 + 8, collectible.Key * 16 + 8)) { Name = $"{collectible.Value}_address"});
                CollectibleWatchers.Add(new MemoryWatcher<ulong>(new DeepPointer((IntPtr)baseOffset, 0x22B54E0 + 8, collectible.Key * 16 + 8, 0x10)) { Name = $"{collectible.Value}_value" });
            }

            // Add flag watchers
            foreach (var flag in GTAVSplits.EpsilonFlags)
            {
                FlagWatchers.Add(new MemoryWatcher<int>(new DeepPointer((IntPtr)baseOffset, 0x2A07E70, 0xCCCA0 + (flag.Key * 8))) { Name = flag.Value });
            }
        }
        public void UpdateMainVariables(Process game)
        {
            StuntJumps.Update(game);
            RandomEvents.Update(game);
            Hobbies.Update(game);
            CurrentCutscene.Update(game);
            CurrentScript.Update(game);
            Loading.Update(game);
            CompletionPercent.Update(game);
            GolfHole.Update(game);
            InCutscene.Update(game);
            InMission.Update(game);
            InMission2.Update(game);
            NoPlayerControl.Update(game);
            DebugString.Update(game);
            MissionPassedScreen.Update(game);
            CollectibleScreen.Update(game);
        }

        public void UpdateWatcherLists(Process game)
        {
            FreaksWatchers.UpdateAll(game);
            CollectibleWatchers.UpdateAll(game);
            FlagWatchers.UpdateAll(game);
        }

        public void UpdateAll(Process game)
        {
            StuntJumps.Update(game);
            RandomEvents.Update(game);
            Hobbies.Update(game);
            CurrentCutscene.Update(game);
            CurrentScript.Update(game);
            Loading.Update(game);
            CompletionPercent.Update(game);
            GolfHole.Update(game);
            InCutscene.Update(game);
            InMission.Update(game);
            InMission2.Update(game);
            NoPlayerControl.Update(game);
            DebugString.Update(game);
            MissionPassedScreen.Update(game);
            CollectibleScreen.Update(game);
            FreaksWatchers.UpdateAll(game);
            CollectibleWatchers.UpdateAll(game);
        }
    }

    class GTAVMemory
    {
        // Vars
        private List<int> _ignorePIDs;      // PIDs to ignore if necessary
        private readonly Dictionary<int, string> _missionsList; // Flat structured missions list
        private List<string> _completedSplits; // List of already completed splits
        private GTAVData _data; // Game Memory Data
        private Process _game; // Game Process
        private string _lastExecutedCutscene; // Last executed cutscene

        // Event handlers
        public event EventHandler OnMissionPassed;
        public event EventHandler OnEndingTrigger;
        public event EventHandler<int> OnStuntJumpCompleted;
        public event EventHandler OnPrologueStart;
        public event EventHandler OnMiscCategoryStart;

        // Module size for finding game version
        private enum ExpectedModuleSizes
        {
            GTAVExe = 70718464
        }

        public GTAVMemory()
        {
            _ignorePIDs = new List<int>();
            _completedSplits = new List<string>();
            _missionsList = new Dictionary<int, string>();
            foreach (var header in GTAVSplits.Missions)
            {
                foreach (var mission in header.Value)
                {
                    _missionsList[mission.Key] = mission.Value;
                }
            }
        }

        // Main loop for main variables
        public void Update(GTAVSettings settings)
        {
            // Try to rehook process if lost
            if (_game == null || _game.HasExited)
            {
                if (!TryGetGameProcess())
                    return;
            }

            _data.UpdateMainVariables(_game);

            TimedTraceListener.Instance.UpdateCount++;

            if (_data.InCutscene.Changed)
            {
                if (_data.InCutscene.Current == 1)
                {
                    _lastExecutedCutscene = _data.CurrentCutscene.Current;
                }
            }

            if (settings.AutoSplit)
            {
                if (_data.StuntJumps.Changed)
                {
                    if (_data.StuntJumps.Current == _data.StuntJumps.Old + 1)
                    {
                        this.OnStuntJumpCompleted?.Invoke(this, _data.StuntJumps.Current);
                    }
                }
                foreach (var mission in _missionsList)
                {
                    if (_data.LastMissionPassed.Changed)
                    {
                        if (_data.LastMissionPassed.Current == mission.Key && !_completedSplits.Contains(mission.Value))
                        {
                            _completedSplits.Add(mission.Value);
                        }
                    }
                }


                // Handling all cutscene based splits
                if (_data.CurrentCutscene.Changed)
                {
                    // Ending A check
                    if (_data.CurrentCutscene.Current == "fin_a_ext" && _data.NoPlayerControl.Current == 1 && _data.NoPlayerControl.Changed && _data.InMission.Current == 1)
                    {
                        this.OnEndingTrigger?.Invoke(this, EventArgs.Empty);
                    }
                }
            }
 
        }

        // Main loop for watcher lists we want to update less often to save CPU
        public void UpdateWatcherLists()
        {
            // Don't do anything if game not running
            if (_game == null || _game.HasExited)
            {
                return;
            }
            _data.UpdateWatcherLists(_game);
        }

        bool TryGetGameProcess()
        {
            // Find process
            Process game = Process.GetProcesses().FirstOrDefault(p => p.ProcessName.ToLower() == "gta5" && !p.HasExited && !_ignorePIDs.Contains(p.Id));

            if (game == null) return false;


            // Verify .exe size
            if (game.MainModuleWow64Safe().ModuleMemorySize == (int)ExpectedModuleSizes.GTAVExe)
            {
                _data = new GTAVData(game.MainModule.BaseAddress.ToInt64());
                _game = game;
                Trace.WriteLine($"Connected to GTAV, PID: {game.Id}");

                return true;
            }
            else
            {
                _ignorePIDs.Add(game.Id);
                MessageBox.Show("Unexpected game version. GTA V 1.27 is required. Use the Project 1.27 utility to downgrade your game or try to restart the game.", "Grand Theft Auto V Autosplitter", MessageBoxButtons.OK, MessageBoxIcon.Error);
                return false;
            }
        }
    }
}
