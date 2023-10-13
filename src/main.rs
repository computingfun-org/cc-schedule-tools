use cc_core::{schedule::Schedule, JobNumber};
use clap::Parser;
use eframe::App;
use egui::{Button, CentralPanel, Key, RichText, TextEdit};
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
///
pub struct Args {
    #[arg()]
    /// Job Number to open. If not provided, a prompt will be shown asking for Job Number.
    pub job_number: Option<JobNumber>,
}

const APP_NAME: &str = "";

const NATIVE_OPTIONS: eframe::NativeOptions = eframe::NativeOptions {
    always_on_top: false,
    maximized: false,
    decorated: true,
    fullscreen: false,
    drag_and_drop_support: true,
    icon_data: None,
    initial_window_pos: None,
    initial_window_size: None,
    min_window_size: None,
    max_window_size: None,
    resizable: true,
    transparent: false,
    mouse_passthrough: false,
    active: true,
    vsync: true,
    multisampling: 0,
    depth_buffer: 0,
    stencil_buffer: 0,
    hardware_acceleration: eframe::HardwareAcceleration::Preferred,
    renderer: eframe::Renderer::Glow,
    follow_system_theme: true,
    default_theme: eframe::Theme::Dark,
    run_and_return: true,
    event_loop_builder: None,
    window_builder: None,
    shader_version: None,
    centered: true,
    app_id: None,
    persist_window: false,
};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        APP_NAME,
        NATIVE_OPTIONS,
        Box::new(|_cc| Box::new(MainApp::from(Args::parse().job_number))),
    )
}

#[derive(Debug)]
struct MainApp {
    state: MainAppState,
}

impl From<Option<JobNumber>> for MainApp {
    fn from(value: Option<JobNumber>) -> Self {
        match value {
            Some(job_num) => Self {
                state: MainAppState::ScheduleEditor(ScheduleEditor {
                    job_num,
                    schedule: Schedule::default(),
                }),
            },
            None => Self {
                state: MainAppState::JobNumberInput(JobNumberInput::default()),
            },
        }
    }
}

#[derive(Debug)]
enum MainAppState {
    JobNumberInput(JobNumberInput),
    ScheduleEditor(ScheduleEditor),
}

impl App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let esc_pressed = ctx.input(|i| i.key_pressed(Key::Escape));
        if esc_pressed {
            match &self.state {
                MainAppState::JobNumberInput(jni) => {
                    if jni.input.is_empty() {
                        frame.close();
                    } else {
                        self.state = MainAppState::JobNumberInput(JobNumberInput::default());
                    }
                }
                MainAppState::ScheduleEditor(_) => {
                    self.state = MainAppState::JobNumberInput(JobNumberInput::default());
                }
            }
            return;
        }

        match &mut self.state {
            MainAppState::JobNumberInput(input) => match input.get() {
                Some(job_num) => {
                    self.state = MainAppState::ScheduleEditor(ScheduleEditor {
                        job_num,
                        schedule: Schedule::default(),
                    })
                }
                None => input.update(ctx, frame),
            },
            MainAppState::ScheduleEditor(editor) => {
                editor.update(ctx, frame);
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct JobNumberInput {
    input: String,
    output: Option<JobNumber>,
}

impl JobNumberInput {
    pub fn get(&self) -> Option<JobNumber> {
        self.output
    }
}

impl App for JobNumberInput {
    fn update(&mut self, ctx: &egui::Context, _sframe: &mut eframe::Frame) {
        let enter_pressed = ctx.input(|i| i.key_pressed(Key::Enter));
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Job Number");

                let job_num = TextEdit::singleline(&mut self.input).lock_focus(true);
                let job_num_response = ui.add(job_num);

                if !job_num_response.has_focus() {
                    job_num_response.request_focus();
                }

                if job_num_response.changed() {
                    self.input.retain(|c| c.is_ascii_digit());
                }

                let btn = {
                    let text = RichText::new("Enter").underline();
                    Button::new(text)
                };
                let btn_response = ui.add(btn);
                let btn_clicked = btn_response.clicked();

                if btn_clicked || enter_pressed {
                    self.output = JobNumber::from_str(&self.input).ok();
                }
            });
        });
    }
}

#[derive(Debug)]
struct ScheduleEditor {
    job_num: JobNumber,
    schedule: Schedule,
}

impl App for ScheduleEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading(format!("{:?}\n\n\n{:?}", self.job_num, self.schedule));
            });
        });
    }
}
