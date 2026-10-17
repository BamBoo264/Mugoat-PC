// Windows launcher for the experimental Mugoat port. GPL-3.0-or-later.
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Text;
using System.Threading.Tasks;
using System.Web.Script.Serialization;
using System.Windows.Forms;

class Launcher : Form {
    static readonly string Root = AppDomain.CurrentDomain.BaseDirectory;
    static readonly string Home = Path.Combine(Root, "user-data");
    static readonly string ConfigPath = Path.Combine(Home, "config.json");
    static readonly JavaScriptSerializer Json = new JavaScriptSerializer();
    Dictionary<string,object> config = new Dictionary<string,object>();
    TextBox folder = new TextBox(), server = new TextBox(), room = new TextBox(), pass = new TextBox(), name = new TextBox();
    ComboBox gpu = new ComboBox(), resolution = new ComboBox();
    NumericUpDown fps = new NumericUpDown(), scale = new NumericUpDown();
    CheckBox online = new CheckBox(), pvp = new CheckBox(), fullscreen = new CheckBox();
    TextBox log = new TextBox();
    FlowLayoutPanel actions = new FlowLayoutPanel();
    Process running;
    StreamWriter logFile;
    readonly object logLock = new object();

    [STAThread] static int Main(string[] args) {
        if (args.Length == 1 && args[0] == "--self-test") {
            var sample = new Dictionary<string,object> { {"server","127.0.0.1:47520"}, {"room","test"}, {"room_pass","abc"}, {"player_name","PC"}, {"multiplayer",true}, {"game_folder",@"D:\Game data\GTA"} };
            var parsed = Json.Deserialize<Dictionary<string,object>>(Json.Serialize(sample));
            return (string)parsed["game_folder"] == (string)sample["game_folder"] && (bool)parsed["multiplayer"] ? 0 : 1;
        }
        Application.EnableVisualStyles(); Application.SetCompatibleTextRenderingDefault(false);
        if(args.Length==2 && args[0]=="--render-preview") {
            using(var form=new Launcher()) {form.ShowInTaskbar=false;form.StartPosition=FormStartPosition.Manual;form.Location=new Point(-32000,-32000);form.Show();Application.DoEvents();form.PerformLayout();using(var bitmap=new Bitmap(form.Width,form.Height)){form.DrawToBitmap(bitmap,new Rectangle(0,0,bitmap.Width,bitmap.Height));bitmap.Save(args[1]);}form.Hide();}
            return 0;
        }
        Application.Run(new Launcher()); return 0;
    }
    Launcher() {
        Text = "Mugoat PC — Experimental"; Font = new Font("Segoe UI", 10); ClientSize = new Size(820,710); MinimumSize = new Size(740,730);
        var layout = new TableLayoutPanel { Dock = DockStyle.Fill, Padding = new Padding(18), ColumnCount = 3, RowCount = 13 };
        layout.ColumnStyles.Add(new ColumnStyle(SizeType.Absolute,140)); layout.ColumnStyles.Add(new ColumnStyle(SizeType.Percent,100)); layout.ColumnStyles.Add(new ColumnStyle(SizeType.Absolute,105));
        var info = new Label { Text = "Bản Windows thử nghiệm • Chưa xác nhận vào game cùng iOS\nCần bộ game có game.wasm + shaders + data/manifest.json. Thư mục mod riêng chưa đủ.", AutoSize = true, Dock = DockStyle.Fill };
        layout.Controls.Add(info,0,0);layout.SetColumnSpan(info,3);
        AddRow(layout,1,"Thư mục game",folder); var browse = new Button {Text="Chọn…",Dock=DockStyle.Fill};
        browse.Click += delegate { using (var d = new FolderBrowserDialog {Description="Chọn thư mục gốc chứa data và b (hoặc shaders/game.wasm)"}) {if(d.ShowDialog()==DialogResult.OK) folder.Text=d.SelectedPath;} }; layout.Controls.Add(browse,2,1);
        AddRow(layout,2,"Server (IP:port)",server); AddRow(layout,3,"Tên phòng",room); AddRow(layout,4,"Mật khẩu phòng",pass);pass.UseSystemPasswordChar=true; AddRow(layout,5,"Tên người chơi",name);
        gpu.DropDownStyle=ComboBoxStyle.DropDownList;gpu.Items.AddRange(new object[]{"auto","vulkan","dx12"});AddRow(layout,6,"GPU backend",gpu);
        resolution.DropDownStyle=ComboBoxStyle.DropDownList;resolution.Items.AddRange(new object[]{"1280x720","1600x900","1920x1080","2560x1440"});AddRow(layout,7,"Độ phân giải",resolution);
        fps.Minimum=30;fps.Maximum=240;fps.Increment=30;AddRow(layout,8,"Giới hạn FPS",fps);
        scale.Minimum=0.25M;scale.Maximum=1;scale.DecimalPlaces=2;scale.Increment=0.05M;AddRow(layout,9,"Tỷ lệ render",scale);
        var flags=new FlowLayoutPanel {Dock=DockStyle.Fill,AutoSize=true};online.Text="Multiplayer";pvp.Text="PvP";fullscreen.Text="Toàn màn hình";
        foreach(var c in new[]{online,pvp,fullscreen}) {c.AutoSize=true;flags.Controls.Add(c);}layout.Controls.Add(flags,1,10);layout.SetColumnSpan(flags,2);
        actions.Dock=DockStyle.Fill;actions.AutoSize=true;layout.Controls.Add(actions,0,11);layout.SetColumnSpan(actions,3);
        AddAction("Lưu",delegate{Save();Append("Đã lưu config.json.");});
        AddAction("Kiểm tra GPU",async delegate{await RunEngine("--gpu-check");});
        AddAction("Kiểm tra data",async delegate{await RunEngine("--check-game");});
        AddAction("Chơi",async delegate{if(online.Checked && String.IsNullOrWhiteSpace(server.Text)){MessageBox.Show("Nhập cùng server IP:port mà bản iOS đang dùng.");return;}await RunEngine("");});
        AddAction("Mở log",delegate{Directory.CreateDirectory(Home);Process.Start(new ProcessStartInfo(Home){UseShellExecute=true});});
        log.Multiline=true;log.ReadOnly=true;log.ScrollBars=ScrollBars.Both;log.WordWrap=false;log.Dock=DockStyle.Fill;log.Font=new Font("Consolas",9);
        layout.Controls.Add(log,0,12);layout.SetColumnSpan(log,3);
        for(int i=0;i<12;i++)layout.RowStyles.Add(new RowStyle(SizeType.AutoSize));layout.RowStyles.Add(new RowStyle(SizeType.Percent,100));
        Controls.Add(layout);LoadSettings();Append("Chưa có full data vẫn có thể kiểm tra GPU. Log lưu tại user-data/latest.log.");
        FormClosing += delegate(object sender,FormClosingEventArgs e){if(running!=null){e.Cancel=true;MessageBox.Show("Đóng cửa sổ game trước khi thoát launcher.");}};
    }
    void AddRow(TableLayoutPanel t,int row,string caption,Control control){t.Controls.Add(new Label{Text=caption,AutoSize=true,Anchor=AnchorStyles.Left},0,row);control.Dock=DockStyle.Fill;t.Controls.Add(control,1,row);}
    void AddAction(string text,Action action){var b=new Button {Text=text,AutoSize=true};b.Click+=delegate{try{action();}catch(Exception e){MessageBox.Show(e.Message,"Mugoat PC");}};actions.Controls.Add(b);}
    string Str(string k,string fallback){object v;return config.TryGetValue(k,out v)&&v!=null ? Convert.ToString(v):fallback;}
    bool Flag(string k){object v;return config.TryGetValue(k,out v)&&v is bool&&(bool)v;}
    decimal Num(string k,decimal fallback){decimal n;return Decimal.TryParse(Str(k,""),out n)?n:fallback;}
    void LoadSettings(){
        try{if(File.Exists(ConfigPath))config=Json.Deserialize<Dictionary<string,object>>(File.ReadAllText(ConfigPath));}
        catch(Exception e){MessageBox.Show("Không đọc được config: "+e.Message);}
        folder.Text=Str("game_folder",Path.Combine(Home,"game"));server.Text=Str("server","");room.Text=Str("room","default");pass.Text=Str("room_pass","");name.Text=Str("player_name","PC Player");
        gpu.SelectedItem=Str("gpu_backend","auto");if(gpu.SelectedIndex<0)gpu.SelectedIndex=0;
        resolution.SelectedItem=Str("resolution","1280x720");if(resolution.SelectedIndex<0)resolution.SelectedIndex=0;
        fps.Value=Math.Min(fps.Maximum,Math.Max(fps.Minimum,Num("fps_limit",60)));scale.Value=Math.Min(scale.Maximum,Math.Max(scale.Minimum,Num("scale",0.5M)));
        online.Checked=Flag("multiplayer");pvp.Checked=Flag("pvp");fullscreen.Checked=Flag("fullscreen");
    }
    void Save(){
        config["game_folder"]=folder.Text.Trim();config["server"]=server.Text.Trim();config["room"]=room.Text.Trim();config["room_pass"]=pass.Text.Trim();config["player_name"]=name.Text.Trim();
        config["gpu_backend"]=gpu.Text;config["resolution"]=resolution.Text;config["fps_limit"]=(int)fps.Value;config["scale"]=scale.Value;
        config["multiplayer"]=online.Checked;config["host"]=false;config["pvp"]=pvp.Checked;config["fullscreen"]=fullscreen.Checked;
        foreach(string k in new[]{"texture_quality","shadow_quality","shadow_distance","reflection_quality","grass_quality","water_quality","particle_quality","postfx","city_density","ped_density","distance_scaling"})if(!config.ContainsKey(k))config[k]=0;
        if(!config.ContainsKey("start_mode"))config["start_mode"]=2;if(!config.ContainsKey("fxaa"))config["fxaa"]=false;
        Directory.CreateDirectory(Home);var tmp=ConfigPath+".new";File.WriteAllText(tmp,Json.Serialize(config),new UTF8Encoding(false));
        if(File.Exists(ConfigPath))File.Replace(tmp,ConfigPath,ConfigPath+".bak");else File.Move(tmp,ConfigPath);
    }
    void Append(string text){
        if(text==null)return;
        lock(logLock){if(logFile!=null){logFile.WriteLine(text);logFile.Flush();}}
        if(IsDisposed)return;if(InvokeRequired){BeginInvoke(new Action<string>(AppendUi),text);return;}AppendUi(text);
    }
    void AppendUi(string text){if(log.TextLength>200000)log.Clear();log.AppendText(text+Environment.NewLine);}
    async Task RunEngine(string args){
        if(running!=null)return;
        try {
            Save();var exe=Path.Combine(Root,"Mugoat-PC.exe");if(!File.Exists(exe))throw new FileNotFoundException("Thiếu Mugoat-PC.exe cạnh launcher.");
            log.Clear();lock(logLock){logFile=new StreamWriter(Path.Combine(Home,"latest.log"),false,new UTF8Encoding(false));}
            var start=new ProcessStartInfo(exe,args){WorkingDirectory=Root,UseShellExecute=false,CreateNoWindow=true,RedirectStandardOutput=true,RedirectStandardError=true,StandardOutputEncoding=Encoding.UTF8,StandardErrorEncoding=Encoding.UTF8};
            start.EnvironmentVariables["MUGOAT_HOME"]=Home;start.EnvironmentVariables["MUGOAT_GPU"]=gpu.Text;
            if(!String.IsNullOrWhiteSpace(folder.Text))start.EnvironmentVariables["MUGOAT_GAME_DIR"]=folder.Text.Trim();
            running=new Process{StartInfo=start};running.OutputDataReceived+=delegate(object s,DataReceivedEventArgs e){Append(e.Data);};running.ErrorDataReceived+=delegate(object s,DataReceivedEventArgs e){Append(e.Data);};
            actions.Enabled=false;running.Start();running.BeginOutputReadLine();running.BeginErrorReadLine();
            await Task.Run(()=>running.WaitForExit());Append("Exit code: "+running.ExitCode);
        }catch(Exception e){Append("Lỗi: "+e.Message);}
        finally{if(running!=null){running.Dispose();running=null;}actions.Enabled=true;lock(logLock){if(logFile!=null){logFile.Dispose();logFile=null;}}}
    }
}
