use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use eframe::egui;
use std::time::{Duration, Instant};

struct APMTracker {
    actions: Vec<Instant>,
    current_apm: f32,
    device_state: DeviceState,
    last_mouse_buttons: Vec<bool>,
    last_keys: Vec<Keycode>,
}

impl APMTracker {
    fn new() -> Self {
        Self {
            actions: Vec::new(),
            current_apm: 0.0,
            device_state: DeviceState::new(),
            last_mouse_buttons: Vec::new(),
            last_keys: Vec::new(),
        }
    }

    fn update_apm(&mut self) {
        // Clean up old actions (older than 60 seconds)
        let minute_ago = Instant::now() - Duration::from_secs(60);
        self.actions.retain(|&time| time > minute_ago);

        // Get current input states
        let mouse: MouseState = self.device_state.get_mouse();
        let keys: Vec<Keycode> = self.device_state.get_keys();

        // Check for new mouse clicks
        if mouse.button_pressed != self.last_mouse_buttons {
            self.actions.push(Instant::now());
            self.last_mouse_buttons = mouse.button_pressed;
        }

        // Check for new keystrokes
        if keys != self.last_keys {
            self.actions.push(Instant::now());
            self.last_keys = keys.clone();
        }

        // Calculate APM - just use the count of actions in the last minute
        self.current_apm = self.actions.len() as f32;
    }

    fn should_quit(&self) -> bool {
        let keys = self.device_state.get_keys();
        (keys.contains(&Keycode::LControl) && keys.contains(&Keycode::Q)) ||
        (keys.contains(&Keycode::LAlt) && keys.contains(&Keycode::F4))
    }
}

impl eframe::App for APMTracker {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_apm();

        // Check for quit shortcut
        if self.should_quit() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::Window::new("")
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
            .resizable(false)
            .frame(egui::Frame::none())
            .title_bar(false)
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(format!("APM: {:.0}", self.current_apm))
                        .size(24.0)
                );
            });

        // Request continuous updates
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([100.0, 50.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_window_level(egui::WindowLevel::AlwaysOnTop)
            .with_app_id("apm_tracker"),
        ..Default::default()
    };

    eframe::run_native(
        "APM Tracker",
        options,
        Box::new(|_cc| Box::new(APMTracker::new())),
    )
}
