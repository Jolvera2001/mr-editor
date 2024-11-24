use eframe::egui;

pub struct Editor {
    content: String,
    preview: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            content: String::new(),
            preview: false, 
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

            // ui.add_sized(
            //     [half_width, available.y],
            //     egui::ScrollArea::vertical().show(ui |ui| {

            //         })
            //     );
        });   
    }
}