use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

pub struct Editor {
    content: String,
    md_cache: CommonMarkCache,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            content: String::new(),
            md_cache: CommonMarkCache::default(),
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui|{
            let available = ui.available_size();
            let half_width = available.x / 2.0;

            ui.add_sized(
                [half_width, available.y],
                 egui::TextEdit::multiline(&mut self.content)
                );

            ui.allocate_ui_with_layout(
                egui::Vec2 { x: half_width, y: available.y },
                egui::Layout::top_down(egui::Align::LEFT), 
                |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        CommonMarkViewer::new().show(ui, &mut self.md_cache, &self.content)
                    })
                })
        });   
    }
}