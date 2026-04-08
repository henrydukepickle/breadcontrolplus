use eframe::egui::CentralPanel;

use crate::{game::Game, timed_button::Timer};

pub struct App {
    gs: Game,
    gain_timer: Timer
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { gs: Game::new(), gain_timer: Timer::new(2.0) }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            ui.label(format!("crobits: {}", self.gs.crobits));
            ui.label(format!("Crobbits: {}", self.gs.crobbits_big));
            ui.label(format!("Crobbit size: {}", self.gs.crobbit_size));
            if ui.button("gain").clicked() && self.gain_timer.done() {
                self.gs.gain();
                self.gain_timer.start();
            }
            
        });
        ui.request_repaint();
    }
}
