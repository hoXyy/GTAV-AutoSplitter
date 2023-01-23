using System;
using System.Reflection;
using LiveSplit.Model;

namespace LiveSplit.UI.Components
{
    public class GTAVFactory : IComponentFactory
    {
        public string ComponentName => "Grand Theft Auto V Autosplitter";

        public string Description => "An autosplitter for Grand Theft Auto V (Version 1.27).";

        public ComponentCategory Category => ComponentCategory.Control;

        public string UpdateName => ComponentName;

        public string XMLURL => "URL_HERE";

        public string UpdateURL => "URL_HERE";

        public Version Version => Assembly.GetExecutingAssembly().GetName().Version;

        public IComponent Create(LiveSplitState state) => new GTAVAutoSplitter(state);
    }
}
