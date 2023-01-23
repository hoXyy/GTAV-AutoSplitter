using LiveSplit.GTAV;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Reflection;
using System.Text.RegularExpressions;
using System.Windows.Forms;
using System.Xml;
using static System.Windows.Forms.VisualStyles.VisualStyleElement.TextBox;
using StringList = System.Collections.Generic.List<string>;

namespace LiveSplit.UI.Components
{
    public partial class GTAVSettings : UserControl
    {
        private TabControl settingsTabsControl;
        private TabPage generalTab;
        private TabPage tabPage2;
        private CheckBox autoSplitSettingCheckbox;
        private CheckBox autoStartSettingCheckbox;
        private Label gameConnectedLabel;
        private CheckBox autoResetSettingCheckbox;
        private TabPage autoSplitSettingsPage;
        private TabControl tabControl1;
        private TabPage missionSplitSettingsPage;
        private TabPage tabPage3;
        private TabPage segmentSplitSettingPage;
        private GroupBox segmentStartGroupBox;
        private CheckBox blitzStartSplitCheckbox;
        private CheckBox countrysideStartSplitCheckbox;
        private CheckBox freshStartSplitCheckbox;
        private CheckBox paletoStartSplitCheckbox;
        private CheckBox deepStartSplitCheckbox;
        private CheckBox bureauStartSplitCheckbox;
        private CheckBox thirdWayStartSplitCheckbox;
        private GroupBox segmentEndGroupBox;
        private CheckBox thirdWayEndSplitCheckbox;
        private CheckBox bureauEndSplitCheckbox;
        private CheckBox freshEndSplitCheckbox;
        private CheckBox paletoEndSplitCheckbox;
        private CheckBox deepEndSplitCheckbox;
        private CheckBox blitzEndSplitCheckbox;
        private CheckBox countrysideEndSplitCheckbox;
        private CheckBox trevorEndSplitCheckbox;
        private Label label1;
        private Label missionSplitSettingLabel;
        private TableLayoutPanel tableLayoutPanel1;
        private CheckedListBox freaksSplitCheckedListBox;
        private ListView missionSplitListView;
        public bool hasChanged;

        public bool AutoStart { get; set; }
        public bool AutoSplit { get; set; }
        public bool AutoReset { get; set; }
        public bool CountrysideStartSplit { get; set; }
        public bool BlitzStartSplit { get; set; }
        public bool DeepStartSplit { get; set; }
        public bool PaletoStartSplit { get; set; }
        public bool FreshStartSplit { get; set; }
        public bool BureauStartSplit { get; set; }
        public bool ThirdWayStartSplit { get; set; }
        public bool TrevorEndSplit { get; set; }
        public bool CountrysideEndSplit { get; set; }
        public bool BlitzEndSplit { get; set; }
        public bool DeepEndSplit { get; set; }
        public bool PaletoEndSplit { get; set; }
        public bool FreshEndSplit { get; set; }
        public bool BureauEndSplit { get; set; }
        public bool ThirdWayEndSplit { get; set; }


        public GTAVSettings()
        {
            InitializeComponent();
            AutoStart = true;
            AutoSplit = true;
            AutoReset = true;

            CountrysideStartSplit = true;
            BlitzStartSplit = true;
            DeepStartSplit = true;
            PaletoStartSplit = true;
            FreshStartSplit = true;
            BureauStartSplit = true;
            ThirdWayStartSplit = true;

            TrevorEndSplit = true;
            CountrysideEndSplit = true;
            BlitzEndSplit = true;
            DeepEndSplit = true;
            PaletoEndSplit = true;
            FreshEndSplit = true;
            BureauEndSplit = true;
            ThirdWayEndSplit = true;

            foreach (var segment in GTAVSplits.Missions)
            {
                foreach (var mission in segment.Value)
                {
                    missionSplitListView.Items.Add(mission.Value);
                }
            }

            foreach (var header in GTAVSplits.Freaks)
            {
                foreach (var mission in header.Value)
                {
                    freaksSplitCheckedListBox.Items.Add(mission.Value);
                }
            }

            hasChanged = true;

            // General settings
            autoStartSettingCheckbox.DataBindings.Add("Checked", this, "AutoStart", false, DataSourceUpdateMode.OnPropertyChanged);
            autoSplitSettingCheckbox.DataBindings.Add("Checked", this, "AutoSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            autoResetSettingCheckbox.DataBindings.Add("Checked", this, "AutoReset", false, DataSourceUpdateMode.OnPropertyChanged);

            // Segment start split settings
            countrysideStartSplitCheckbox.DataBindings.Add("Checked", this, "CountrysideStartSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            blitzStartSplitCheckbox.DataBindings.Add("Checked", this, "BlitzStartSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            deepStartSplitCheckbox.DataBindings.Add("Checked", this, "DeepStartSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            paletoStartSplitCheckbox.DataBindings.Add("Checked", this, "PaletoStartSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            freshStartSplitCheckbox.DataBindings.Add("Checked", this, "FreshStartSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            bureauStartSplitCheckbox.DataBindings.Add("Checked", this, "BureauStartSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            thirdWayStartSplitCheckbox.DataBindings.Add("Checked", this, "ThirdWayStartSplit", false, DataSourceUpdateMode.OnPropertyChanged);

            // Segment end split settings
            trevorEndSplitCheckbox.DataBindings.Add("Checked", this, "TrevorEndSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            countrysideEndSplitCheckbox.DataBindings.Add("Checked", this, "CountrysideEndSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            blitzEndSplitCheckbox.DataBindings.Add("Checked", this, "BlitzEndSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            deepEndSplitCheckbox.DataBindings.Add("Checked", this, "DeepEndSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            paletoEndSplitCheckbox.DataBindings.Add("Checked", this, "PaletoEndSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            freshEndSplitCheckbox.DataBindings.Add("Checked", this, "FreshEndSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            bureauEndSplitCheckbox.DataBindings.Add("Checked", this, "BureauEndSplit", false, DataSourceUpdateMode.OnPropertyChanged);
            thirdWayEndSplitCheckbox.DataBindings.Add("Checked", this, "ThirdWayEndSplit", false, DataSourceUpdateMode.OnPropertyChanged);
        }

        /// <summary>
        /// Updates the list with the activated splits everytime the settings window is closed.
        /// </summary>
        private void ConfirmSplits(object sender, EventArgs e)
        {
            hasChanged = true;
        }

        public XmlNode GetSettings(XmlDocument document)
        {
            XmlElement settingsNode = document.CreateElement("Settings");
            settingsNode.AppendChild(ToElement(document, "Version", Assembly.GetExecutingAssembly().GetName().Version));

            XmlElement generalSettingsNode = document.CreateElement("General");
            settingsNode.AppendChild(generalSettingsNode);

            generalSettingsNode.AppendChild(ToElement(document, "AutoStart", AutoStart));
            generalSettingsNode.AppendChild(ToElement(document, "AutoSplit", AutoSplit));
            generalSettingsNode.AppendChild(ToElement(document, "AutoReset", AutoReset));

            XmlElement segmentStartSplitElement = document.CreateElement("SegmentStart");
            settingsNode.AppendChild(segmentStartSplitElement);
            segmentStartSplitElement.AppendChild(ToElement(document, "Countryside", CountrysideStartSplit));
            segmentStartSplitElement.AppendChild(ToElement(document, "BlitzPlay", BlitzStartSplit));
            segmentStartSplitElement.AppendChild(ToElement(document, "DeepInside", DeepStartSplit));
            segmentStartSplitElement.AppendChild(ToElement(document, "PaletoScore", PaletoStartSplit));
            segmentStartSplitElement.AppendChild(ToElement(document, "FreshMeat", FreshStartSplit));
            segmentStartSplitElement.AppendChild(ToElement(document, "BureauRaid", BureauStartSplit));
            segmentStartSplitElement.AppendChild(ToElement(document, "ThirdWay", ThirdWayStartSplit));

            XmlElement segmentEndSplitElement = document.CreateElement("SegmentEnd");
            settingsNode.AppendChild(segmentEndSplitElement);
            segmentEndSplitElement.AppendChild(ToElement(document, "Trevor", TrevorEndSplit));
            segmentEndSplitElement.AppendChild(ToElement(document, "Countryside", CountrysideEndSplit));
            segmentEndSplitElement.AppendChild(ToElement(document, "BlitzPlay", BlitzEndSplit));
            segmentEndSplitElement.AppendChild(ToElement(document, "DeepInside", DeepEndSplit));
            segmentEndSplitElement.AppendChild(ToElement(document, "PaletoScore", PaletoEndSplit));
            segmentEndSplitElement.AppendChild(ToElement(document, "FreshMeat", FreshEndSplit));
            segmentEndSplitElement.AppendChild(ToElement(document, "BureauRaid", BureauEndSplit));
            segmentEndSplitElement.AppendChild(ToElement(document, "ThirdWay", ThirdWayEndSplit));

            XmlElement missionsSplitElement = document.CreateElement("Missions");
            foreach (ListViewItem mission in missionSplitListView.Items)
            {
                missionsSplitElement.AppendChild(ToElement(document, GetXMLFriendlyString(mission.Text), mission.Checked));
            }

            settingsNode.AppendChild(missionsSplitElement);

            return settingsNode;
        }

        public void SetSettings(XmlNode settings)
        {
            XmlNode generalSettings = settings["General"];
            AutoStart = SettingsHelper.ParseBool(generalSettings["AutoStart"]);
            AutoSplit = SettingsHelper.ParseBool(generalSettings["AutoSplit"]);
            AutoReset = SettingsHelper.ParseBool(generalSettings["AutoReset"]);

            XmlNode segmentStartSettings = settings["SegmentStart"];
            CountrysideStartSplit = SettingsHelper.ParseBool(segmentStartSettings["Countryside"]);
            BlitzStartSplit = SettingsHelper.ParseBool(segmentStartSettings["BlitzPlay"]);
            DeepStartSplit = SettingsHelper.ParseBool(segmentStartSettings["DeepInside"]);
            PaletoStartSplit = SettingsHelper.ParseBool(segmentStartSettings["PaletoScore"]);
            FreshStartSplit = SettingsHelper.ParseBool(segmentStartSettings["FreshMeat"]);
            BureauStartSplit = SettingsHelper.ParseBool(segmentStartSettings["BureauRaid"]);
            ThirdWayStartSplit = SettingsHelper.ParseBool(segmentStartSettings["ThirdWay"]);

            XmlNode segmentEndSettings = settings["SegmentEnd"];
            TrevorEndSplit = SettingsHelper.ParseBool(segmentEndSettings["Trevor"]);
            CountrysideEndSplit = SettingsHelper.ParseBool(segmentEndSettings["Countryside"]);
            BlitzEndSplit = SettingsHelper.ParseBool(segmentEndSettings["BlitzPlay"]);
            DeepEndSplit = SettingsHelper.ParseBool(segmentEndSettings["DeepInside"]);
            PaletoEndSplit = SettingsHelper.ParseBool(segmentEndSettings["PaletoScore"]);
            FreshEndSplit = SettingsHelper.ParseBool(segmentEndSettings["FreshMeat"]);
            BureauEndSplit = SettingsHelper.ParseBool(segmentEndSettings["BureauRaid"]);
            ThirdWayEndSplit = SettingsHelper.ParseBool(segmentEndSettings["ThirdWay"]);
        }

        public string GetXMLFriendlyString(string input)
        {
            Regex r = new Regex(
                          "(?:[^a-zA-Z0-9 ]|(?<=['\"])s)",
                          RegexOptions.IgnoreCase | RegexOptions.CultureInvariant | RegexOptions.Compiled);
            return r.Replace(input, String.Empty).Replace(" ", String.Empty);
        }

        /// <summary>
        /// Returns a serialized version of a setting based on its identifier.
        /// </summary>
        static XmlElement ToElement<T>(XmlDocument document, string name, T value)
        {
            XmlElement str = document.CreateElement(name);
            str.InnerText = value.ToString();
            return str;
        }

        private void InitializeComponent()
        {
            this.tableLayoutPanel1 = new System.Windows.Forms.TableLayoutPanel();
            this.settingsTabsControl = new System.Windows.Forms.TabControl();
            this.generalTab = new System.Windows.Forms.TabPage();
            this.autoResetSettingCheckbox = new System.Windows.Forms.CheckBox();
            this.gameConnectedLabel = new System.Windows.Forms.Label();
            this.autoSplitSettingCheckbox = new System.Windows.Forms.CheckBox();
            this.autoStartSettingCheckbox = new System.Windows.Forms.CheckBox();
            this.tabPage2 = new System.Windows.Forms.TabPage();
            this.autoSplitSettingsPage = new System.Windows.Forms.TabPage();
            this.tabControl1 = new System.Windows.Forms.TabControl();
            this.missionSplitSettingsPage = new System.Windows.Forms.TabPage();
            this.tabPage3 = new System.Windows.Forms.TabPage();
            this.segmentSplitSettingPage = new System.Windows.Forms.TabPage();
            this.segmentEndGroupBox = new System.Windows.Forms.GroupBox();
            this.thirdWayEndSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.bureauEndSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.freshEndSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.paletoEndSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.deepEndSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.blitzEndSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.countrysideEndSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.trevorEndSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.segmentStartGroupBox = new System.Windows.Forms.GroupBox();
            this.thirdWayStartSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.bureauStartSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.freshStartSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.paletoStartSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.deepStartSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.blitzStartSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.countrysideStartSplitCheckbox = new System.Windows.Forms.CheckBox();
            this.missionSplitSettingLabel = new System.Windows.Forms.Label();
            this.label1 = new System.Windows.Forms.Label();
            this.freaksSplitCheckedListBox = new System.Windows.Forms.CheckedListBox();
            this.missionSplitListView = new System.Windows.Forms.ListView();
            this.tableLayoutPanel1.SuspendLayout();
            this.settingsTabsControl.SuspendLayout();
            this.generalTab.SuspendLayout();
            this.autoSplitSettingsPage.SuspendLayout();
            this.tabControl1.SuspendLayout();
            this.missionSplitSettingsPage.SuspendLayout();
            this.segmentSplitSettingPage.SuspendLayout();
            this.segmentEndGroupBox.SuspendLayout();
            this.segmentStartGroupBox.SuspendLayout();
            this.SuspendLayout();
            // 
            // tableLayoutPanel1
            // 
            this.tableLayoutPanel1.ColumnCount = 1;
            this.tableLayoutPanel1.ColumnStyles.Add(new System.Windows.Forms.ColumnStyle(System.Windows.Forms.SizeType.Percent, 100F));
            this.tableLayoutPanel1.Controls.Add(this.settingsTabsControl, 0, 0);
            this.tableLayoutPanel1.Dock = System.Windows.Forms.DockStyle.Fill;
            this.tableLayoutPanel1.Location = new System.Drawing.Point(7, 7);
            this.tableLayoutPanel1.Name = "tableLayoutPanel1";
            this.tableLayoutPanel1.RowCount = 1;
            this.tableLayoutPanel1.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Percent, 100F));
            this.tableLayoutPanel1.RowStyles.Add(new System.Windows.Forms.RowStyle(System.Windows.Forms.SizeType.Absolute, 20F));
            this.tableLayoutPanel1.Size = new System.Drawing.Size(458, 536);
            this.tableLayoutPanel1.TabIndex = 0;
            // 
            // settingsTabsControl
            // 
            this.settingsTabsControl.Controls.Add(this.generalTab);
            this.settingsTabsControl.Controls.Add(this.tabPage2);
            this.settingsTabsControl.Controls.Add(this.autoSplitSettingsPage);
            this.settingsTabsControl.Location = new System.Drawing.Point(3, 3);
            this.settingsTabsControl.Name = "settingsTabsControl";
            this.settingsTabsControl.SelectedIndex = 0;
            this.settingsTabsControl.Size = new System.Drawing.Size(452, 530);
            this.settingsTabsControl.TabIndex = 0;
            // 
            // generalTab
            // 
            this.generalTab.Controls.Add(this.autoResetSettingCheckbox);
            this.generalTab.Controls.Add(this.gameConnectedLabel);
            this.generalTab.Controls.Add(this.autoSplitSettingCheckbox);
            this.generalTab.Controls.Add(this.autoStartSettingCheckbox);
            this.generalTab.Location = new System.Drawing.Point(4, 22);
            this.generalTab.Name = "generalTab";
            this.generalTab.Padding = new System.Windows.Forms.Padding(3);
            this.generalTab.Size = new System.Drawing.Size(444, 504);
            this.generalTab.TabIndex = 0;
            this.generalTab.Text = "General";
            // 
            // autoResetSettingCheckbox
            // 
            this.autoResetSettingCheckbox.AutoSize = true;
            this.autoResetSettingCheckbox.Checked = true;
            this.autoResetSettingCheckbox.CheckState = System.Windows.Forms.CheckState.Checked;
            this.autoResetSettingCheckbox.Location = new System.Drawing.Point(5, 75);
            this.autoResetSettingCheckbox.Name = "autoResetSettingCheckbox";
            this.autoResetSettingCheckbox.Size = new System.Drawing.Size(115, 17);
            this.autoResetSettingCheckbox.TabIndex = 3;
            this.autoResetSettingCheckbox.Text = "Enable Auto Reset";
            this.autoResetSettingCheckbox.UseVisualStyleBackColor = true;
            // 
            // gameConnectedLabel
            // 
            this.gameConnectedLabel.Font = new System.Drawing.Font("Microsoft Sans Serif", 8.25F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.gameConnectedLabel.ForeColor = System.Drawing.Color.Red;
            this.gameConnectedLabel.Location = new System.Drawing.Point(2, 9);
            this.gameConnectedLabel.Name = "gameConnectedLabel";
            this.gameConnectedLabel.Size = new System.Drawing.Size(440, 13);
            this.gameConnectedLabel.TabIndex = 2;
            this.gameConnectedLabel.Text = "Currently NOT connected to GTA V.";
            this.gameConnectedLabel.TextAlign = System.Drawing.ContentAlignment.TopCenter;
            // 
            // autoSplitSettingCheckbox
            // 
            this.autoSplitSettingCheckbox.AutoSize = true;
            this.autoSplitSettingCheckbox.Checked = true;
            this.autoSplitSettingCheckbox.CheckState = System.Windows.Forms.CheckState.Checked;
            this.autoSplitSettingCheckbox.Location = new System.Drawing.Point(5, 52);
            this.autoSplitSettingCheckbox.Name = "autoSplitSettingCheckbox";
            this.autoSplitSettingCheckbox.Size = new System.Drawing.Size(107, 17);
            this.autoSplitSettingCheckbox.TabIndex = 1;
            this.autoSplitSettingCheckbox.Text = "Enable Auto Split";
            this.autoSplitSettingCheckbox.UseVisualStyleBackColor = true;
            // 
            // autoStartSettingCheckbox
            // 
            this.autoStartSettingCheckbox.AutoSize = true;
            this.autoStartSettingCheckbox.Checked = true;
            this.autoStartSettingCheckbox.CheckState = System.Windows.Forms.CheckState.Checked;
            this.autoStartSettingCheckbox.Location = new System.Drawing.Point(5, 29);
            this.autoStartSettingCheckbox.Name = "autoStartSettingCheckbox";
            this.autoStartSettingCheckbox.Size = new System.Drawing.Size(109, 17);
            this.autoStartSettingCheckbox.TabIndex = 0;
            this.autoStartSettingCheckbox.Text = "Enable Auto Start";
            this.autoStartSettingCheckbox.UseVisualStyleBackColor = true;
            // 
            // tabPage2
            // 
            this.tabPage2.Location = new System.Drawing.Point(4, 22);
            this.tabPage2.Name = "tabPage2";
            this.tabPage2.Padding = new System.Windows.Forms.Padding(3);
            this.tabPage2.Size = new System.Drawing.Size(444, 504);
            this.tabPage2.TabIndex = 1;
            this.tabPage2.Text = "Auto Start";
            // 
            // autoSplitSettingsPage
            // 
            this.autoSplitSettingsPage.Controls.Add(this.tabControl1);
            this.autoSplitSettingsPage.Location = new System.Drawing.Point(4, 22);
            this.autoSplitSettingsPage.Name = "autoSplitSettingsPage";
            this.autoSplitSettingsPage.Padding = new System.Windows.Forms.Padding(3);
            this.autoSplitSettingsPage.Size = new System.Drawing.Size(444, 504);
            this.autoSplitSettingsPage.TabIndex = 2;
            this.autoSplitSettingsPage.Text = "Auto Splitting";
            // 
            // tabControl1
            // 
            this.tabControl1.Controls.Add(this.missionSplitSettingsPage);
            this.tabControl1.Controls.Add(this.tabPage3);
            this.tabControl1.Controls.Add(this.segmentSplitSettingPage);
            this.tabControl1.Location = new System.Drawing.Point(7, 7);
            this.tabControl1.Name = "tabControl1";
            this.tabControl1.SelectedIndex = 0;
            this.tabControl1.Size = new System.Drawing.Size(431, 501);
            this.tabControl1.TabIndex = 0;
            // 
            // missionSplitSettingsPage
            // 
            this.missionSplitSettingsPage.Controls.Add(this.missionSplitListView);
            this.missionSplitSettingsPage.Controls.Add(this.freaksSplitCheckedListBox);
            this.missionSplitSettingsPage.Controls.Add(this.label1);
            this.missionSplitSettingsPage.Controls.Add(this.missionSplitSettingLabel);
            this.missionSplitSettingsPage.Location = new System.Drawing.Point(4, 22);
            this.missionSplitSettingsPage.Name = "missionSplitSettingsPage";
            this.missionSplitSettingsPage.Padding = new System.Windows.Forms.Padding(3);
            this.missionSplitSettingsPage.Size = new System.Drawing.Size(423, 475);
            this.missionSplitSettingsPage.TabIndex = 0;
            this.missionSplitSettingsPage.Text = "Mission Splitting";
            // 
            // tabPage3
            // 
            this.tabPage3.Location = new System.Drawing.Point(4, 22);
            this.tabPage3.Name = "tabPage3";
            this.tabPage3.Padding = new System.Windows.Forms.Padding(3);
            this.tabPage3.Size = new System.Drawing.Size(423, 475);
            this.tabPage3.TabIndex = 1;
            this.tabPage3.Text = "Collectible Splitting";
            // 
            // segmentSplitSettingPage
            // 
            this.segmentSplitSettingPage.Controls.Add(this.segmentEndGroupBox);
            this.segmentSplitSettingPage.Controls.Add(this.segmentStartGroupBox);
            this.segmentSplitSettingPage.Location = new System.Drawing.Point(4, 22);
            this.segmentSplitSettingPage.Name = "segmentSplitSettingPage";
            this.segmentSplitSettingPage.Padding = new System.Windows.Forms.Padding(3);
            this.segmentSplitSettingPage.Size = new System.Drawing.Size(423, 475);
            this.segmentSplitSettingPage.TabIndex = 2;
            this.segmentSplitSettingPage.Text = "Segment Splitting";
            // 
            // segmentEndGroupBox
            // 
            this.segmentEndGroupBox.Controls.Add(this.thirdWayEndSplitCheckbox);
            this.segmentEndGroupBox.Controls.Add(this.bureauEndSplitCheckbox);
            this.segmentEndGroupBox.Controls.Add(this.freshEndSplitCheckbox);
            this.segmentEndGroupBox.Controls.Add(this.paletoEndSplitCheckbox);
            this.segmentEndGroupBox.Controls.Add(this.deepEndSplitCheckbox);
            this.segmentEndGroupBox.Controls.Add(this.blitzEndSplitCheckbox);
            this.segmentEndGroupBox.Controls.Add(this.countrysideEndSplitCheckbox);
            this.segmentEndGroupBox.Controls.Add(this.trevorEndSplitCheckbox);
            this.segmentEndGroupBox.Location = new System.Drawing.Point(7, 78);
            this.segmentEndGroupBox.Name = "segmentEndGroupBox";
            this.segmentEndGroupBox.Size = new System.Drawing.Size(410, 68);
            this.segmentEndGroupBox.TabIndex = 1;
            this.segmentEndGroupBox.TabStop = false;
            this.segmentEndGroupBox.Text = "Select the segments you want to split at the end of:";
            // 
            // thirdWayEndSplitCheckbox
            // 
            this.thirdWayEndSplitCheckbox.AutoSize = true;
            this.thirdWayEndSplitCheckbox.Location = new System.Drawing.Point(278, 44);
            this.thirdWayEndSplitCheckbox.Name = "thirdWayEndSplitCheckbox";
            this.thirdWayEndSplitCheckbox.Size = new System.Drawing.Size(75, 17);
            this.thirdWayEndSplitCheckbox.TabIndex = 7;
            this.thirdWayEndSplitCheckbox.Text = "Third Way";
            this.thirdWayEndSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // bureauEndSplitCheckbox
            // 
            this.bureauEndSplitCheckbox.AutoSize = true;
            this.bureauEndSplitCheckbox.Location = new System.Drawing.Point(186, 43);
            this.bureauEndSplitCheckbox.Name = "bureauEndSplitCheckbox";
            this.bureauEndSplitCheckbox.Size = new System.Drawing.Size(85, 17);
            this.bureauEndSplitCheckbox.TabIndex = 6;
            this.bureauEndSplitCheckbox.Text = "Bureau Raid";
            this.bureauEndSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // freshEndSplitCheckbox
            // 
            this.freshEndSplitCheckbox.AutoSize = true;
            this.freshEndSplitCheckbox.Location = new System.Drawing.Point(101, 43);
            this.freshEndSplitCheckbox.Name = "freshEndSplitCheckbox";
            this.freshEndSplitCheckbox.Size = new System.Drawing.Size(79, 17);
            this.freshEndSplitCheckbox.TabIndex = 5;
            this.freshEndSplitCheckbox.Text = "Fresh Meat";
            this.freshEndSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // paletoEndSplitCheckbox
            // 
            this.paletoEndSplitCheckbox.AutoSize = true;
            this.paletoEndSplitCheckbox.Location = new System.Drawing.Point(7, 43);
            this.paletoEndSplitCheckbox.Name = "paletoEndSplitCheckbox";
            this.paletoEndSplitCheckbox.Size = new System.Drawing.Size(87, 17);
            this.paletoEndSplitCheckbox.TabIndex = 4;
            this.paletoEndSplitCheckbox.Text = "Paleto Score";
            this.paletoEndSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // deepEndSplitCheckbox
            // 
            this.deepEndSplitCheckbox.AutoSize = true;
            this.deepEndSplitCheckbox.Location = new System.Drawing.Point(242, 20);
            this.deepEndSplitCheckbox.Name = "deepEndSplitCheckbox";
            this.deepEndSplitCheckbox.Size = new System.Drawing.Size(83, 17);
            this.deepEndSplitCheckbox.TabIndex = 3;
            this.deepEndSplitCheckbox.Text = "Deep Inside";
            this.deepEndSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // blitzEndSplitCheckbox
            // 
            this.blitzEndSplitCheckbox.AutoSize = true;
            this.blitzEndSplitCheckbox.Location = new System.Drawing.Point(168, 20);
            this.blitzEndSplitCheckbox.Name = "blitzEndSplitCheckbox";
            this.blitzEndSplitCheckbox.Size = new System.Drawing.Size(68, 17);
            this.blitzEndSplitCheckbox.TabIndex = 2;
            this.blitzEndSplitCheckbox.Text = "Blitz Play";
            this.blitzEndSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // countrysideEndSplitCheckbox
            // 
            this.countrysideEndSplitCheckbox.AutoSize = true;
            this.countrysideEndSplitCheckbox.Location = new System.Drawing.Point(79, 20);
            this.countrysideEndSplitCheckbox.Name = "countrysideEndSplitCheckbox";
            this.countrysideEndSplitCheckbox.Size = new System.Drawing.Size(81, 17);
            this.countrysideEndSplitCheckbox.TabIndex = 1;
            this.countrysideEndSplitCheckbox.Text = "Countryside";
            this.countrysideEndSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // trevorEndSplitCheckbox
            // 
            this.trevorEndSplitCheckbox.AutoSize = true;
            this.trevorEndSplitCheckbox.Location = new System.Drawing.Point(7, 20);
            this.trevorEndSplitCheckbox.Name = "trevorEndSplitCheckbox";
            this.trevorEndSplitCheckbox.Size = new System.Drawing.Size(65, 17);
            this.trevorEndSplitCheckbox.TabIndex = 0;
            this.trevorEndSplitCheckbox.Text = "Trevor%";
            this.trevorEndSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // segmentStartGroupBox
            // 
            this.segmentStartGroupBox.Controls.Add(this.thirdWayStartSplitCheckbox);
            this.segmentStartGroupBox.Controls.Add(this.bureauStartSplitCheckbox);
            this.segmentStartGroupBox.Controls.Add(this.freshStartSplitCheckbox);
            this.segmentStartGroupBox.Controls.Add(this.paletoStartSplitCheckbox);
            this.segmentStartGroupBox.Controls.Add(this.deepStartSplitCheckbox);
            this.segmentStartGroupBox.Controls.Add(this.blitzStartSplitCheckbox);
            this.segmentStartGroupBox.Controls.Add(this.countrysideStartSplitCheckbox);
            this.segmentStartGroupBox.Location = new System.Drawing.Point(6, 6);
            this.segmentStartGroupBox.Name = "segmentStartGroupBox";
            this.segmentStartGroupBox.Size = new System.Drawing.Size(411, 65);
            this.segmentStartGroupBox.TabIndex = 0;
            this.segmentStartGroupBox.TabStop = false;
            this.segmentStartGroupBox.Text = "Select the segments you want to split at the start of:";
            // 
            // thirdWayStartSplitCheckbox
            // 
            this.thirdWayStartSplitCheckbox.AutoSize = true;
            this.thirdWayStartSplitCheckbox.Location = new System.Drawing.Point(187, 42);
            this.thirdWayStartSplitCheckbox.Name = "thirdWayStartSplitCheckbox";
            this.thirdWayStartSplitCheckbox.Size = new System.Drawing.Size(75, 17);
            this.thirdWayStartSplitCheckbox.TabIndex = 6;
            this.thirdWayStartSplitCheckbox.Text = "Third Way";
            this.thirdWayStartSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // bureauStartSplitCheckbox
            // 
            this.bureauStartSplitCheckbox.AutoSize = true;
            this.bureauStartSplitCheckbox.Location = new System.Drawing.Point(95, 42);
            this.bureauStartSplitCheckbox.Name = "bureauStartSplitCheckbox";
            this.bureauStartSplitCheckbox.Size = new System.Drawing.Size(85, 17);
            this.bureauStartSplitCheckbox.TabIndex = 5;
            this.bureauStartSplitCheckbox.Text = "Bureau Raid";
            this.bureauStartSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // freshStartSplitCheckbox
            // 
            this.freshStartSplitCheckbox.AutoSize = true;
            this.freshStartSplitCheckbox.Location = new System.Drawing.Point(8, 42);
            this.freshStartSplitCheckbox.Name = "freshStartSplitCheckbox";
            this.freshStartSplitCheckbox.Size = new System.Drawing.Size(79, 17);
            this.freshStartSplitCheckbox.TabIndex = 4;
            this.freshStartSplitCheckbox.Text = "Fresh Meat";
            this.freshStartSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // paletoStartSplitCheckbox
            // 
            this.paletoStartSplitCheckbox.AutoSize = true;
            this.paletoStartSplitCheckbox.Location = new System.Drawing.Point(258, 19);
            this.paletoStartSplitCheckbox.Name = "paletoStartSplitCheckbox";
            this.paletoStartSplitCheckbox.Size = new System.Drawing.Size(87, 17);
            this.paletoStartSplitCheckbox.TabIndex = 3;
            this.paletoStartSplitCheckbox.Text = "Paleto Score";
            this.paletoStartSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // deepStartSplitCheckbox
            // 
            this.deepStartSplitCheckbox.AutoSize = true;
            this.deepStartSplitCheckbox.Location = new System.Drawing.Point(169, 19);
            this.deepStartSplitCheckbox.Name = "deepStartSplitCheckbox";
            this.deepStartSplitCheckbox.Size = new System.Drawing.Size(83, 17);
            this.deepStartSplitCheckbox.TabIndex = 2;
            this.deepStartSplitCheckbox.Text = "Deep Inside";
            this.deepStartSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // blitzStartSplitCheckbox
            // 
            this.blitzStartSplitCheckbox.AutoSize = true;
            this.blitzStartSplitCheckbox.Location = new System.Drawing.Point(95, 19);
            this.blitzStartSplitCheckbox.Name = "blitzStartSplitCheckbox";
            this.blitzStartSplitCheckbox.Size = new System.Drawing.Size(68, 17);
            this.blitzStartSplitCheckbox.TabIndex = 1;
            this.blitzStartSplitCheckbox.Text = "Blitz Play";
            this.blitzStartSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // countrysideStartSplitCheckbox
            // 
            this.countrysideStartSplitCheckbox.AutoSize = true;
            this.countrysideStartSplitCheckbox.Location = new System.Drawing.Point(8, 19);
            this.countrysideStartSplitCheckbox.Name = "countrysideStartSplitCheckbox";
            this.countrysideStartSplitCheckbox.Size = new System.Drawing.Size(81, 17);
            this.countrysideStartSplitCheckbox.TabIndex = 0;
            this.countrysideStartSplitCheckbox.Text = "Countryside";
            this.countrysideStartSplitCheckbox.UseVisualStyleBackColor = true;
            // 
            // missionSplitSettingLabel
            // 
            this.missionSplitSettingLabel.AutoSize = true;
            this.missionSplitSettingLabel.Location = new System.Drawing.Point(6, 3);
            this.missionSplitSettingLabel.Name = "missionSplitSettingLabel";
            this.missionSplitSettingLabel.Size = new System.Drawing.Size(207, 13);
            this.missionSplitSettingLabel.TabIndex = 0;
            this.missionSplitSettingLabel.Text = "Select which missions you want to split on:";
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.Location = new System.Drawing.Point(6, 243);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(298, 13);
            this.label1.TabIndex = 2;
            this.label1.Text = "Select the Strangers and Freaks missions you want to split on:";
            // 
            // freaksSplitCheckedListBox
            // 
            this.freaksSplitCheckedListBox.FormattingEnabled = true;
            this.freaksSplitCheckedListBox.Location = new System.Drawing.Point(9, 259);
            this.freaksSplitCheckedListBox.Name = "freaksSplitCheckedListBox";
            this.freaksSplitCheckedListBox.Size = new System.Drawing.Size(408, 214);
            this.freaksSplitCheckedListBox.TabIndex = 5;
            // 
            // missionSplitListView
            // 
            this.missionSplitListView.CheckBoxes = true;
            this.missionSplitListView.HideSelection = false;
            this.missionSplitListView.Location = new System.Drawing.Point(9, 19);
            this.missionSplitListView.Name = "missionSplitListView";
            this.missionSplitListView.Size = new System.Drawing.Size(408, 221);
            this.missionSplitListView.TabIndex = 6;
            this.missionSplitListView.UseCompatibleStateImageBehavior = false;
            // 
            // GTAVSettings
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 13F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.Controls.Add(this.tableLayoutPanel1);
            this.Name = "GTAVSettings";
            this.Padding = new System.Windows.Forms.Padding(7);
            this.Size = new System.Drawing.Size(472, 550);
            this.tableLayoutPanel1.ResumeLayout(false);
            this.settingsTabsControl.ResumeLayout(false);
            this.generalTab.ResumeLayout(false);
            this.generalTab.PerformLayout();
            this.autoSplitSettingsPage.ResumeLayout(false);
            this.tabControl1.ResumeLayout(false);
            this.missionSplitSettingsPage.ResumeLayout(false);
            this.missionSplitSettingsPage.PerformLayout();
            this.segmentSplitSettingPage.ResumeLayout(false);
            this.segmentEndGroupBox.ResumeLayout(false);
            this.segmentEndGroupBox.PerformLayout();
            this.segmentStartGroupBox.ResumeLayout(false);
            this.segmentStartGroupBox.PerformLayout();
            this.ResumeLayout(false);

        }

        private void treeView1_AfterSelect(object sender, TreeViewEventArgs e)
        {

        }
    }
}
