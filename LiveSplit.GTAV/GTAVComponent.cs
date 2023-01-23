using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Windows.Forms;
using System.Xml;
using LiveSplit.GTAV;
using LiveSplit.Model;

namespace LiveSplit.UI.Components
{
    class GTAVAutoSplitter : LogicComponent
    {
        public bool Initialized { get; private set; } = false;
        public GTAVSettings Settings { get; set; }

        private TimerModel _timer;
        private LiveSplitState _state;
        private GTAVMemory _gameMemory;
        private Timer _updateMainTimer;
        private Timer _updateListTimer;

        public GTAVAutoSplitter(LiveSplitState state)
        {
#if DEBUG
            Debug.Listeners.Clear();
            Debug.Listeners.Add(TimedTraceListener.Instance);
#endif
            Settings = new GTAVSettings();

            _state = state;
            _timer = new TimerModel { CurrentState = state };
            
            _updateMainTimer = new Timer() { Enabled = true, Interval = 15 };
            _updateMainTimer.Tick += updateMainTimer_Tick;
            _updateListTimer = new Timer() { Enabled = true, Interval = 45 };
            _updateListTimer.Tick += updateListTimer_Tick;

            _gameMemory = new GTAVMemory();
            _gameMemory.OnMissionPassed += gameMemory_OnMissionPassed;
            _gameMemory.OnStuntJumpCompleted += gameMemory_OnStuntJumpCompleted;

            Trace.WriteLine("Component loaded! Waiting for game process...");
        }

        private void gameMemory_OnStuntJumpCompleted(object sender, int e)
        {
            if (Settings.AutoSplit) _timer.Split();
        }

        private void gameMemory_OnMissionPassed(object sender, EventArgs e)
        {
            if (Settings.AutoSplit) _timer.Split();
        }

        private void updateMainTimer_Tick(object sender, EventArgs e)
        {
            try
            {
                _gameMemory.Update(this.Settings);
            }
            catch (Exception ex)
            {
                Trace.WriteLine(ex.ToString());
            }
        }

        private void updateListTimer_Tick(object sender, EventArgs e)
        {
            try
            {
                _gameMemory.UpdateWatcherLists();
            }
            catch (Exception ex)
            {
                Trace.WriteLine(ex.ToString());
            }
        }

        public override string ComponentName => "Grand Theft Auto V Autosplitter";

        public override void Dispose()
        {
            _updateListTimer.Dispose();
            _updateMainTimer.Dispose();
        }

        public override XmlNode GetSettings(XmlDocument document)
        {
            return Settings.GetSettings(document);
        }

        public override Control GetSettingsControl(LayoutMode mode)
        {
            return Settings;
        }

        public override void SetSettings(XmlNode settings)
        {
            Settings.SetSettings(settings);
        }

        public override void Update(IInvalidator invalidator, LiveSplitState state, float width, float height, LayoutMode mode)
        {
        }
    }

    // Debug
    public class TimedTraceListener : DefaultTraceListener
    {
        private static TimedTraceListener _instance;
        public static TimedTraceListener Instance => _instance ?? (_instance = new TimedTraceListener());

        private TimedTraceListener() { }

        public int UpdateCount
        {
            [MethodImpl(MethodImplOptions.Synchronized)]
            get;
            [MethodImpl(MethodImplOptions.Synchronized)]
            set;
        }

        public override void WriteLine(string message)
        {
            base.WriteLine($"[GTA V AutoSplitter, Update count: {this.UpdateCount}]: {message}");
        }
    }
}
