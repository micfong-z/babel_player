#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use eframe::egui;
use tokio::runtime::Runtime;

fn main() -> eframe::Result {
    let rt = Runtime::new().expect("Unable to create Tokio Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                // Exactly 1 year!
                tokio::time::sleep(Duration::from_secs(31556952)).await;
            }
        })
    });

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "Babel Player",
        native_options,
        Box::new(|cc| Ok(Box::new(babel_player::BabelPlayerApp::new(cc)))),
    )
}
