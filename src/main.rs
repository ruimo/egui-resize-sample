#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::containers::ScrollArea;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    );
}

struct MyApp {
    name: String,
    age: u32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").resizable(true).show(ctx, |ui| {
            ScrollArea::both()
                .show_viewport(ui, |ui, viewport| {
                    ui.heading("Top");
                    ui.heading("Top");
                    ui.heading("Top");
                    ui.heading("Top");
                    ui.heading("Top");
                    ui.heading("Top");
                    ui.heading("Top");
                    ui.heading("Top");
                    ui.heading("Top");
                });
        });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ScrollArea::both()
                    .show_viewport(ui, |ui, viewport| {
                        ui.heading("Bottom");
                        ui.heading("Bottom");
                        ui.heading("Bottom");
                        ui.heading("Bottom");
                        ui.heading("Bottom");
                        ui.heading("Bottom");
                        ui.heading("Bottom");
                        ui.heading("Bottom");
                        ui.heading("Bottom");
                    });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both()
                .show_viewport(ui, |ui, viewport| {
                    ui.heading("My egui Application");
                    ui.horizontal(|ui| {
                        ui.label("Your name: ");
                        ui.text_edit_singleline(&mut self.name);
                    });
                    ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
                    if ui.button("Click each year").clicked() {
                        self.age += 1;
                    }
                    ui.label(format!("Hello '{}', age {}", self.name, self.age));
                });
        });
    }
}
