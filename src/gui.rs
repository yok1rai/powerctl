use std::process;

use crate::Options;
use eframe::egui;

pub fn run(option: Options) -> eframe::Result<()> {
    if option == Options::Help {
        eprintln!("help command can only be used in cli");
        process::exit(5);
    }
    eframe::run_native(
        "Powerctl",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([320.0, 180.0]),
            ..Default::default()
        },
        Box::new(move |_cc| {
            Box::new(App {
                option,
                executed: false,
            })
        }),
    )
}

struct App {
    option: Options,
    executed: bool,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                ui.heading("Confirm Action");
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                ui.label("Are you sure you want to continue?");
            });

            ui.add_space(20.0);

            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {

                    if ui.button("Yes").clicked() {
                        let _ = self.option.execute();
                        self.executed = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    if ui.button("No").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                });
            });

            ui.add_space(10.0);

            if self.executed {
                ui.vertical_centered(|ui| {
                    ui.label("✔ Executed");
                });
            }
        });
    }
}
