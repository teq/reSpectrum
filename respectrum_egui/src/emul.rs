#![feature(coroutine_trait)]

extern crate librespectrum;

use librespectrum::{
    core::{Clock, CpuBus, Scheduler},
    devs::{BreakpointManager, Device, DeviceManager, mem::Memory}
};

use std::{
    rc::Rc,
    vec::Vec,
    fs::File,
    io::Read,
    cell::RefCell,
};

mod theme;
mod windows;
use windows::{SubWindow, CpuWindow, DisassmWindow, MemoryWindow, BusWindow, DisplayWindow};

struct EmulApp<'a> {
    windows: Vec<(bool, Box<dyn SubWindow + 'a>)>
}

impl eframe::App for EmulApp<'_> {

    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        visuals.panel_fill.to_normalized_gamma_f32()
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        let ctx = ui.ctx().clone();

        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Window", |ui| {
                    for (open, window) in &mut self.windows {
                        ui.checkbox(open, window.name());
                    }
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        for (open, window) in self.windows.iter_mut() {
            if *open {
                window.show(ui, open);
            }
        }

    }

}

fn main() -> eframe::Result {

    let bus: Rc<CpuBus> = Default::default();
    let clock: Rc<Clock> = Default::default();

    let breakpoints = Rc::new(BreakpointManager::default());
    let device_manager = Rc::new(DeviceManager::new(&bus, &clock, &breakpoints));
    let cpu = device_manager.create_cpu();
    let mem: Rc<dyn Memory> = {
        let mem = device_manager.create_48k_memory();
        let mut buffer: Vec<u8> = Vec::new();
        File::open("roms/48.rom").unwrap().read_to_end(&mut buffer).unwrap();
        mem.load(0, &buffer);
        mem
    };
    let logger = device_manager.create_bus_logger();

    let scheduler =  Rc::new(RefCell::new(
        Scheduler::new(&clock, vec![cpu.run(), mem.run(), logger.run()])
    ));

    let app = Box::new(EmulApp {
        windows: vec![
            (true, Box::new(CpuWindow::new(&cpu))),
            (true, Box::new(DisassmWindow::new(&scheduler, &cpu, &mem, &breakpoints))),
            (true, Box::new(MemoryWindow::new(&mem))),
            (true, Box::new(BusWindow::new(&logger, &device_manager))),
            (true, Box::new(DisplayWindow::new(&mem))),
        ]
    });

    run_native(app)

}

fn run_native<'a>(app: Box<dyn eframe::App + 'a>) -> eframe::Result {

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "reSpectrum - ZX Spectrum emulator",
        native_options,
        Box::new(|creation_context| {
            theme::apply(&creation_context.egui_ctx);
            Ok(app)
        }),
    )

}
