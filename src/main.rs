//! Quickly moves sub-folder files to the parent folder. 
//! It is intended to be used with bibliography PDFs created by Zotero. 
//! 
#![doc = include_str!("../README.md")]


#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
mod window_utils;
mod path_utils;

/// Main function to launch. It create the window for the GUI.
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Move Zotero biblio",
        options,
        Box::new(|_cc| Box::<window_utils::MyApp>::default()),
    )
}
