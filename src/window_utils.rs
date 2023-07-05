use eframe::egui;
use crate::path_utils;



// #[derive(Default)]
pub struct MyApp {
    picked_path: Option<String>,
    name_folder: Option<String>,
    process_folder: bool,
    pdf_only: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            picked_path: None,
            name_folder: None,
            process_folder: false,
            pdf_only: true,
        }
    }
}

impl eframe::App for MyApp {
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Select the folder \"files\" to order the bibliography");

            if ui.button("Open folder…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let path = path.display().to_string();
                    self.picked_path = Some(path.clone());
                    
                    let last_folder = path.split("\\").collect::<Vec<&str>>().last().unwrap().to_string();
                    self.name_folder = Some(last_folder);
                }
            }

            ui.checkbox(&mut self.pdf_only, "PDF only")
                .on_hover_text("Uncheck to move all files. Check to move only PDFs");

            if let Some(picked_path) = &self.picked_path {
                ui.label("\nSelected folder:");
                ui.monospace(picked_path);
                
                ui.vertical_centered(|ui| {
                    if let Some(name_folder) = &self.name_folder {
                        let name = "Process the folder: ".to_owned() + &(name_folder);
                        if ui.button(name).clicked() {
                            println!("Process files");
                            self.process_folder = true;

                            path_utils::move_and_delete_folder(picked_path.to_owned(), self.pdf_only);
                        }
                    }
                });
            }
        
            if self.process_folder {
                ui.label("\nPDFs files moved !");
                
                ui.horizontal(|ui| {
                    if ui.button("Open in file explorer").clicked() {
                        println!("Open folder");
                        let _ = open::that(self.picked_path.clone().unwrap());
                    }

                    if ui.button("Exit").clicked() {
                        frame.close();
                    }
                });
            }
            
        });
    }
}

