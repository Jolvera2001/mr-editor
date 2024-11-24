use eframe::egui;

mod ui;

use ui::page::MrEditor;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
            ..Default::default()
    };

    eframe::run_native(
        "My Todo List",
        options, 
        Box::new(|cc| {
            Ok(Box::new(MrEditor::default()))
        }),
    )
}