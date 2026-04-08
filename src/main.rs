use eframe::NativeOptions;

mod app;
mod game;
mod num;
mod timed_button;

fn main() {
    eframe::run_native(
        "BREAD",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
    .expect("Failed to render app! noooo :(");
}
