pub struct CompanionApp;

impl CompanionApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl eframe::App for CompanionApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {}
}
