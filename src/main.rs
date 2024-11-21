use std::path::PathBuf;

use eframe::egui;
use egui_file_dialog::FileDialog;

use otp_exchange::otp::OneTimePad;

struct MyApp {
    file_dialog: FileDialog,
    selected_file: Option<PathBuf>,
    pad: Option<OneTimePad>,
    pad_loc: String,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            // Create a new file dialog object
            file_dialog: FileDialog::new(),
            selected_file: None,
            pad: None,
            pad_loc: "./bin/pad_temp".to_string()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Select Pad Zip File").clicked() {
                // Open the file dialog to select a file.
                self.file_dialog.select_file();
            }

            ui.label(format!("Pad Zip: {:?}", self.selected_file));

            // Update the dialog
            self.file_dialog.update(ctx);

            // Check if the user selected a file.
            if let Some(path) = self.file_dialog.take_selected() {
                self.selected_file = Some(path.to_path_buf());
            }

            if self.selected_file.clone().is_some() {
                self.pad = Some(OneTimePad::load_zip(&self.selected_file.clone().unwrap().display().to_string(), &self.pad_loc));
                ui.label(format!("{:?} Bytes of Pad Remain", self.pad.as_ref().unwrap().remaining()));
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "OTP EXCHANGE",
        eframe::NativeOptions::default(),
        Box::new(|ctx| Ok(Box::new(MyApp::new(ctx)))),
    )
}