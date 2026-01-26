use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Button, CheckButton, DropDown, Box, Orientation, Label, ComboBoxText, Stack};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Child, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct AppState {
    current_process: Option<Child>,
}

fn get_local_path() -> PathBuf {
    if let Ok(appdir) = env::var("APPDIR") {
        PathBuf::from(appdir)
    } else {
        let mut path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        if path.join("scrcpy.dir").exists() {
            path.push("scrcpy.dir");
        }
        path
    }
}

fn main() {
    env::set_var("G_LOG_LEVELS", "critical");
    env::set_var("GDK_BACKEND", "wayland,x11,*");

    let app = Application::builder()
    .application_id("com.android.webcam")
    .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
    .application(app)
    .title("Android Webcam")
    .default_width(350)
    .default_height(500)
    .build();

    let state = Arc::new(Mutex::new(AppState { current_process: None }));

    #[allow(deprecated)]
    let (tx, rx) = glib::MainContext::channel::<Option<String>>(glib::Priority::default());

    let stack = Stack::builder()
    .transition_type(gtk4::StackTransitionType::SlideLeftRight)
    .build();

    let controls_box = Box::new(Orientation::Vertical, 12);
    controls_box.set_margin_all(20);

    let device_label = Label::new(None);
    let facing_dropdown = DropDown::from_strings(&["Back Camera", "Front Camera"]);
    let res_combo = ComboBoxText::new();
    let fps_dropdown = DropDown::from_strings(&["30", "60"]);
    let mic_check = CheckButton::with_label("Block Phone Microphone");

    let button_box = Box::new(Orientation::Horizontal, 10);
    button_box.set_homogeneous(true);

    let start_btn = Button::builder()
    .label("🚀 Launch")
    .css_classes(["suggested-action"])
    .build();

    let stop_btn = Button::builder()
    .label("🛑 Stop")
    .css_classes(["destructive-action"])
    .build();

    button_box.append(&start_btn);
    button_box.append(&stop_btn);

    let status_label = Label::builder()
    .label("Ready")
    .css_classes(["caption"])
    .build();

    controls_box.append(&device_label);
    controls_box.append(&Label::new(Some("Camera Selection:")));
    controls_box.append(&facing_dropdown);
    controls_box.append(&Label::new(Some("Resolution:")));
    controls_box.append(&res_combo);
    controls_box.append(&Label::new(Some("FPS Limit:")));
    controls_box.append(&fps_dropdown);
    controls_box.append(&mic_check);
    controls_box.append(&button_box);
    controls_box.append(&status_label);

    let waiting_box = Box::new(Orientation::Vertical, 20);
    waiting_box.set_valign(gtk4::Align::Center);
    waiting_box.append(&Label::builder().label("🔌 Waiting for Android Device...").build());

    stack.add_named(&waiting_box, Some("waiting"));
    stack.add_named(&controls_box, Some("controls"));

    let apply_changes = glib::clone!(
        @weak facing_dropdown, @weak res_combo, @weak fps_dropdown, @weak mic_check, @weak status_label, @strong state => move || {
            let mut s = state.lock().unwrap();

            if let Some(mut child) = s.current_process.take() {
                let _ = child.kill();
                let _ = child.wait();
            }

            if !Path::new("/dev/video128").exists() {
                let _ = Command::new("pkexec")
                .args(["modprobe", "v4l2loopback", "video_nr=128", "card_label=Android-Webcam", "exclusive_caps=1"])
                .status();
            }

            let facing = if facing_dropdown.selected() == 1 { "front" } else { "back" };
            let fps = if fps_dropdown.selected() == 1 { "60" } else { "30" };
            let res = res_combo.active_text().unwrap_or_else(|| "1280x720".into());
            let mic_blocked = mic_check.is_active();

            if let Some(child) = run_scrcpy(fps.to_string(), facing.to_string(), mic_blocked, res.to_string()) {
                s.current_process = Some(child);
                let mic_txt = if mic_blocked { " (Mic Off)" } else { " (Mic On)" };
                status_label.set_markup(&format!("<span foreground='green'>● Stream Active{}</span>", mic_txt));
            } else {
                status_label.set_markup("<span foreground='red'>● Failed to Start</span>");
            }
        }
    );

    let apply_shared = Arc::new(apply_changes);

    let auto_apply_running = {
        let state = Arc::clone(&state);
        let apply = Arc::clone(&apply_shared);
        move || {
            let is_running = state.lock().unwrap().current_process.is_some();
            if is_running { (apply)(); }
        }
    };
    let auto_apply_shared = Arc::new(auto_apply_running);

    start_btn.connect_clicked(glib::clone!(@strong apply_shared => move |_| { (apply_shared)(); }));

    stop_btn.connect_clicked(glib::clone!(@strong state, @weak status_label => move |_| {
        if let Ok(mut s) = state.lock() {
            if let Some(mut child) = s.current_process.take() {
                let _ = child.kill();
                let _ = child.wait();
                status_label.set_text("Stopped (Ready)");
            }
        }
    }));

    let aa_face = Arc::clone(&auto_apply_shared);
    facing_dropdown.connect_selected_notify(glib::clone!(@weak res_combo => move |dd| {
        let facing = if dd.selected() == 1 { "front" } else { "back" };
        refresh_resolutions(&res_combo, facing);

        let aa = Arc::clone(&aa_face);
        glib::timeout_add_local(Duration::from_millis(150), move || {
            (aa)();
            glib::ControlFlow::Break
        });
    }));

    let aa_res = Arc::clone(&auto_apply_shared);
    res_combo.connect_changed(move |_| { (aa_res)(); });

    let aa_fps = Arc::clone(&auto_apply_shared);
    fps_dropdown.connect_selected_notify(move |_| { (aa_fps)(); });

    let aa_mic = Arc::clone(&auto_apply_shared);
    mic_check.connect_toggled(move |_| { (aa_mic)(); });

    thread::spawn(move || {
        let adb = get_local_path().join("adb");
        let mut last_status = false;
        loop {
            let output = Command::new(&adb).args(["get-state"]).output();
            let is_connected = output.is_ok() && String::from_utf8_lossy(&output.unwrap().stdout).contains("device");
            if is_connected != last_status {
                let _ = tx.send(if is_connected { get_device_name() } else { None });
                last_status = is_connected;
            }
            thread::sleep(Duration::from_millis(1500));
        }
    });

    rx.attach(None, glib::clone!(
        @weak stack, @weak device_label, @weak res_combo, @weak status_label, @strong state => @default-return glib::ControlFlow::Break, move |device_name| {
            if let Some(name) = device_name {
                device_label.set_markup(&format!("<b>Device: {}</b>", name));
                refresh_resolutions(&res_combo, "back");
                stack.set_visible_child_name("controls");
                status_label.set_text("Stopped (Ready)");
            } else {
                if let Ok(mut s) = state.lock() {
                    if let Some(mut child) = s.current_process.take() {
                        let _ = child.kill();
                        let _ = child.wait();
                    }
                }
                stack.set_visible_child_name("waiting");
            }
            glib::ControlFlow::Continue
        }));

    window.set_child(Some(&stack));
    window.show();
}

fn get_device_name() -> Option<String> {
    let adb = get_local_path().join("adb");
    let out = Command::new(adb).args(["shell", "getprop", "ro.product.model"]).output().ok()?;
    let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if name.is_empty() { None } else { Some(name) }
}

fn refresh_resolutions(combo: &ComboBoxText, facing: &str) {
    combo.remove_all();
    let scrcpy = get_local_path().join("scrcpy");
    let output = Command::new(scrcpy)
    .args(["--video-source=camera", &format!("--camera-facing={}", facing), "--list-camera-sizes"])
    .output();

    if let Ok(out) = output {
        let text = format!("{}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
        let mut sizes = Vec::new();
        for line in text.lines().filter(|l| l.trim().starts_with('-')) {
            for word in line.split_whitespace() {
                let clean = word.trim_matches(|c: char| !c.is_ascii_digit() && c != 'x');
                if clean.contains('x') && clean.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                    if let Ok(w) = clean.split('x').next().unwrap_or("0").parse::<u32>() {
                        if w >= 640 && w <= 2560 {
                            if !sizes.contains(&clean.to_string()) { sizes.push(clean.to_string()); }
                        }
                    }
                }
            }
        }
        sizes.sort_by_key(|s| s.split('x').next().unwrap_or("0").parse::<u32>().unwrap_or(0));
        sizes.reverse();
        for s in sizes { combo.append_text(&s); }
        combo.set_active(Some(0));
    }
}

fn run_scrcpy(fps: String, facing: String, block_mic: bool, res: String) -> Option<Child> {
    let local_path = get_local_path();
    let scrcpy = local_path.join("scrcpy");

    let mut args = vec![
        "--video-source=camera".into(),
        format!("--camera-facing={}", facing),
            format!("--camera-size={}", res),
                format!("--camera-fps={}", fps),
                    "--v4l2-sink=/dev/video128".into(),
                    "--v4l2-buffer=0".into(),
    ];

    if block_mic {
        args.push("--no-audio".into());
    } else {
        args.push("--audio-source=mic".into());
        args.push("--audio-buffer=50".into());
        args.push("--audio-output-buffer=50".into());
    }

    Command::new(scrcpy).args(&args).stdout(Stdio::null()).stderr(Stdio::null()).spawn().ok()
}

trait WidgetExtFixed { fn set_margin_all(&self, m: i32); }
impl<T: IsA<gtk4::Widget>> WidgetExtFixed for T {
    fn set_margin_all(&self, m: i32) {
        self.set_margin_start(m); self.set_margin_end(m); self.set_margin_top(m); self.set_margin_bottom(m);
    }
}
