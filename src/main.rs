use std::path::PathBuf;

use eframe::egui;
use egui_file_dialog::FileDialog;

use otp_exchange::otp::OneTimePad;

struct MyApp {
    pad_select_dialog: FileDialog,
    target_select_dialog: FileDialog,
    pad_zip_path: Option<PathBuf>,
    pad_data: Option<OneTimePad>,
    pad_bin: String,
    target_path: Option<PathBuf>,
    decrypt_target: String,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            // Create a new file dialog object
            pad_select_dialog: FileDialog::new(),
            target_select_dialog: FileDialog::new(),
            pad_zip_path: None,
            pad_data: None,
            pad_bin: "./bin/pad_temp".to_string(),
            target_path: None,
            decrypt_target: "".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Select Pad Zip File").clicked() {
                // Open the file dialog to select a file.
                self.pad_select_dialog.select_file();
            }

            // Update the dialog
            self.pad_select_dialog.update(ctx);

            // Check if the user selected a file.
            if let Some(path) = self.pad_select_dialog.take_selected() {
                self.pad_zip_path = Some(path.to_path_buf());
            }

            if self.pad_zip_path.clone().is_some() {

                ui.label(format!("Pad Zip: {:?}", self.pad_zip_path.clone().unwrap()));
                if !self.pad_data.is_some() {
                    self.pad_data = Some(OneTimePad::load_zip(&self.pad_zip_path.clone().unwrap().display().to_string(), &self.pad_bin));
                }
                ui.label(format!("{:?} Bytes of Pad Remain", self.pad_data.as_ref().unwrap().remaining()));
                if ui.button("Select Target File").clicked() {
                    // Open the file dialog to select a file.
                    self.target_select_dialog.select_file();
                }

                self.target_select_dialog.update(ctx);

                // Check if the user selected a file.
                if let Some(path) = self.target_select_dialog.take_selected() {
                    self.target_path = Some(path.to_path_buf());
                }

                if self.target_path.clone().is_some() {
                    ui.label(format!("target path: {:?}", self.target_path.clone().unwrap()));
                    ui.label("What Would You Like To Name The Decrypted File?");
                    let response = ui.add(egui::TextEdit::singleline(&mut self.decrypt_target));
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.pad_data.as_mut().expect("decrypting without a pad").decrypt_file(&self.target_path.clone().unwrap().display().to_string(), &self.decrypt_target);
                        self.target_path = None;
                    }
                }
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