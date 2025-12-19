mod vnscript;
mod engine;
mod media;
mod ui;
mod save;
mod localization;

use anyhow::Result;
use eframe::egui;
use log::info;

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting StoryForge Visual Novel Engine");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("StoryForge - Visual Novel Engine"),
        ..Default::default()
    };

    eframe::run_native(
        "StoryForge",
        options,
        Box::new(|cc| Ok(Box::new(ui::StoryForgeApp::new(cc)))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run app: {}", e))?;

    Ok(())
}
