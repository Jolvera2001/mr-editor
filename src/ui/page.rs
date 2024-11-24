use eframe::egui;

use super::editor::Editor;

pub struct MrEditor {
    editor: Editor
}

impl Default for MrEditor {
    fn default() -> Self {
        Self {
           editor: Editor::default(),
        }
    }
}

impl eframe::App for MrEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.editor.show(ui);   
        });
    }
}
