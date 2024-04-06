#[deny(unsafe_code)]
#[warn(clippy::all, clippy::pedantic)]
use color_eyre::eyre::{eyre, Result};
use eframe::egui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Storygraph",
        native_options,
        Box::new(|cc| Box::new(GraphApp::new(cc))),
    )
    .map_err(|error| eyre!("Failed to start eframe window: {error}"));

    Ok(())
}

#[derive(Default)]
struct GraphApp {}

impl GraphApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
