use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Button, CheckButton, DropDown, Box, Orientation, Label, Stack, StringList};
use std::env;
use std::path::{PathBuf};
use std::process::{Command, Child, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct AppState {
    current_process: Option<Child>,
}

fn get_local_path() -> PathBuf {
    if let Ok(appdir) = env::var("APPDIR") {
        PathBuf::from(appdir).join("usr").join("bin")
    } else {
        let mut path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        if path.join("scrcpy.dir").exists() {
            path.push("scrcpy.dir/usr/bin");
        }
        path
    }
}

fn main() {
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
    .default_height(550)
    .build();

    let state = Arc::new(Mutex::new(AppState { current_process: None }));
    let (tx, rx) = async_channel::unbounded::<Option<String>>();

    let stack = Stack::builder()
    .transition_type(gtk4::StackTransitionType::SlideLeftRight)
    .build();

    let controls_box = Box::new(Orientation::Vertical, 12);
    controls_box.set_margin_all(20);

    let device_label = Label::new(None);
    let facing_dropdown = DropDown::from_strings(&["Back Camera", "Front Camera"]);

    let camera_warning = Label::builder()
    .use_markup(true)
    .halign(gtk4::Align::Center)
    .build();

    let res_list = StringList::new(&[]);
    let res_dropdown = DropDown::builder().model(&res_list).build();

    let warning_label = Label::builder()
    .use_markup(true)
    .halign(gtk4::Align::Center)
    .wrap(true)
    .build();

    let fps_dropdown = DropDown::from_strings(&["30", "60"]);
    let mic_check = CheckButton::with_label("Block Phone Microphone");

    let button_box = Box::new(Orientation::Horizontal, 10);
    button_box.set_homogeneous(true);

    let start_btn = Button::builder().label("🚀 Launch / Update").css_classes(["suggested-action"]).build();
    let stop_btn = Button::builder().label("🛑 Stop").css_classes(["destructive-action"]).build();

    button_box.append(&start_btn);
    button_box.append(&stop_btn);

    let status_label = Label::builder().label("Ready").build();

    controls_box.append(&device_label);
    controls_box.append(&Label::new(Some("Camera Selection:")));
    controls_box.append(&facing_dropdown);
    controls_box.append(&camera_warning);
    controls_box.append(&Label::new(Some("Resolution:")));
    controls_box.append(&res_dropdown);
    controls_box.append(&warning_label);
    controls_box.append(&Label::new(Some("FPS Limit:")));
    controls_box.append(&fps_dropdown);
    controls_box.append(&mic_check);
    controls_box.append(&button_box);
    controls_box.append(&status_label);

    let waiting_box = Box::new(Orientation::Vertical, 20);
    waiting_box.set_valign(gtk4::Align::Center);
    waiting_box.append(&Label::new(Some("🔌 Waiting for Android Device...")));

    stack.add_named(&waiting_box, Some("waiting"));
    stack.add_named(&controls_box, Some("controls"));

    facing_dropdown.connect_selected_notify(glib::clone!(@weak res_dropdown, @weak camera_warning => move |dd| {
        let facing = if dd.selected() == 1 {
            camera_warning.set_markup("<span foreground='#ffa500' size='small'>⚠️ Note: Back camera usually has better resolution</span>");
            "front"
        } else {
            camera_warning.set_text("");
            "back"
        };
        refresh_resolutions(&res_dropdown, facing);
    }));

    res_dropdown.connect_selected_item_notify(glib::clone!(@weak warning_label => move |dd| {
        if let Some(item) = dd.selected_item().and_then(|i| i.downcast::<gtk4::StringObject>().ok()) {
            let res_str = item.string();
            if let Some(width_str) = res_str.split('x').next() {
                if let Ok(width) = width_str.parse::<u32>() {
                    if width > 1920 {
                        warning_label.set_markup("<span foreground='#ffa500' size='small'>⚠️ High resolution/FPS may cause phone to overheat</span>");
                    } else {
                        warning_label.set_text("");
                    }
                }
            }
        }
    }));

    let apply_changes = glib::clone!(
        @weak facing_dropdown, @weak res_dropdown, @weak fps_dropdown, @weak mic_check, @weak status_label, @strong state => move || {
            let mut s = state.lock().unwrap();
            let bin = get_local_path();

            if let Some(mut child) = s.current_process.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
            let _ = Command::new("killall").arg("-9").arg("scrcpy").status();
            let _ = Command::new(bin.join("adb")).args(["shell", "am", "force-stop", "com.genymobile.scrcpy"]).status();

            thread::sleep(Duration::from_millis(1000));

            let facing = if facing_dropdown.selected() == 1 { "front" } else { "back" };
            let res = res_dropdown.selected_item()
            .and_then(|item| item.downcast::<gtk4::StringObject>().ok())
            .map(|obj| obj.string().to_string())
            .unwrap_or_else(|| "1920x1080".to_string());

            let fps = if fps_dropdown.selected() == 1 { "60" } else { "30" };
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
    start_btn.connect_clicked(glib::clone!(@strong apply_shared => move |_| (apply_shared)()));

    stop_btn.connect_clicked(glib::clone!(@strong state, @weak status_label => move |_| {
        if let Ok(mut s) = state.lock() {
            let bin = get_local_path();
            if let Some(mut child) = s.current_process.take() {
                let _ = child.kill();
                let _ = child.wait();
                let _ = Command::new(bin.join("adb")).args(["shell", "am", "force-stop", "com.genymobile.scrcpy"]).status();
                status_label.set_text("Stopped (Ready)");
            }
        }
    }));

    thread::spawn(move || {
        let adb = get_local_path().join("adb");
        let mut last = false;
        loop {
            let output = Command::new(&adb).args(["get-state"]).output();
            let connected = output.is_ok() && String::from_utf8_lossy(&output.unwrap().stdout).contains("device");
            if connected != last {
                let _ = tx.send_blocking(if connected { get_device_name() } else { None });
                last = connected;
            }
            thread::sleep(Duration::from_millis(1500));
        }
    });

    glib::spawn_future_local(glib::clone!(@weak stack, @weak device_label, @weak res_dropdown, @strong state => async move {
        while let Ok(name) = rx.recv().await {
            if let Some(n) = name {
                device_label.set_markup(&format!("<b>Device: {}</b>", n));
                refresh_resolutions(&res_dropdown, "back");
                stack.set_visible_child_name("controls");
            } else {
                stack.set_visible_child_name("waiting");
            }
        }
    }));

    window.set_child(Some(&stack));
    window.present();
}

fn get_device_name() -> Option<String> {
    let adb = get_local_path().join("adb");
    let out = Command::new(adb).args(["shell", "getprop", "ro.product.model"]).output().ok()?;
    let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if name.is_empty() { None } else { Some(name) }
}

fn refresh_resolutions(dropdown: &DropDown, facing: &str) {
    let bin = get_local_path();
    dropdown.set_model(None::<&StringList>);

    let output = Command::new(bin.join("scrcpy"))
    .env("SCRCPY_SERVER_PATH", bin.join("scrcpy-server"))
    .args(["--video-source=camera", &format!("--camera-facing={}", facing), "--list-camera-sizes"])
    .output();

    let standards = ["3840x2160", "2560x1440", "1920x1080", "1280x720", "720x480"];
    let mut found_sizes = Vec::new();

    if let Ok(out) = output {
        let text = format!("{}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
        let target_id = if facing == "back" { "--camera-id=0" } else { "--camera-id=1" };
        let mut inside_target_block = false;

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("--camera-id=") {
                inside_target_block = trimmed.contains(target_id);
                continue;
            }
            if inside_target_block && trimmed.starts_with("- ") {
                let size = trimmed.trim_start_matches("- ").trim();
                if standards.contains(&size) {
                    found_sizes.push(size.to_string());
                }
            }
        }
    }

    found_sizes.sort_by_key(|s| s.split('x').next().unwrap_or("0").parse::<u32>().unwrap_or(0));
    found_sizes.reverse();
    found_sizes.dedup();

    let string_list = StringList::new(&[]);
    for s in &found_sizes { string_list.append(s); }
    dropdown.set_model(Some(&string_list));

    let default_idx = found_sizes.iter().position(|r| r == "1920x1080").unwrap_or(0);
    if !found_sizes.is_empty() {
        dropdown.set_selected(default_idx as u32);
    }
}

fn run_scrcpy(fps: String, facing: String, block_mic: bool, res: String) -> Option<Child> {
    let bin = get_local_path();
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

    Command::new(bin.join("scrcpy"))
    .env("SCRCPY_SERVER_PATH", bin.join("scrcpy-server"))
    .args(&args)
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .ok()
}

trait WidgetExtFixed { fn set_margin_all(&self, m: i32); }
impl<T: IsA<gtk4::Widget>> WidgetExtFixed for T {
    fn set_margin_all(&self, m: i32) {
        self.set_margin_start(m); self.set_margin_end(m); self.set_margin_top(m); self.set_margin_bottom(m);
    }
}
