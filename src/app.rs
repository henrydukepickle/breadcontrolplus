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
            ui.label("AAAA");
        });
    }
}
