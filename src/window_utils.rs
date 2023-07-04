use eframe::egui;
use crate::path_utils;



#[derive(Default)]
pub struct MyApp {
    picked_path: Option<String>,
    process_folder: Option<bool>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Select the folder \"files\" to order the bibliography");

            if ui.button("Open folder…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.label("\nSelected folder:");
                ui.monospace(picked_path);
                
                ui.vertical_centered(|ui| {
                    // TODO name of the folder in the button.
                    if ui.button("Process files").clicked() {
                        println!("Process files");
                            self.process_folder = Some(true);
                            path_utils::move_and_delete_folder(picked_path.to_owned());
                        }
                    });
                }
        
            if let Some(process_folder) = self.process_folder {
                if process_folder {
                    ui.label("\nPDFs files moved !");
                    
                    if ui.button("Exit").clicked() {
                        frame.close();
                    }
                }
                }
            
        });
    }
}

