use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]),
            ..Default::default()
    };

    eframe::run_native(
        "My Todo List",
        options, 
        Box::new(|cc| {
            Ok(Box::new(TodoApp::default()))
        }),
    )
}
