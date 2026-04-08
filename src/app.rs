use eframe::egui::CentralPanel;

use crate::game::Game;

pub struct App {
    gs: Game,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { gs: Game::new() }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            ui.label(format!("crobits: {}", self.gs.crobits));
            ui.label(format!("Crobbits: {}", self.gs.crobbits_big));
            ui.label(format!("Crobbit size: {}", self.gs.crobbit_size));
            if ui.button("gain").clicked() {
                self.gs.gain();
            }
            
        });
        ui.request_repaint();
    }
}
