use std::hash::Hash;
use eframe::egui::*;

mod bus_window;
pub use bus_window::BusWindow;
mod cpu_window;
pub use cpu_window::CpuWindow;
mod disassm_window;
pub use disassm_window::DisassmWindow;
mod memory_window;
pub use memory_window::MemoryWindow;
mod display_window;
pub use display_window::DisplayWindow;

pub trait SubWindow {

    /// Window name
    fn name(&self) -> String;

    /// Window draw function
    fn show(&mut self, ctx: &Context, focused: bool) -> Response;

}

/// Draw widow with title
pub fn draw_window(
    name: impl Into<String> + Hash,
    focused: bool,
    ctx: &Context,
    add_contents: impl FnOnce(&mut Ui) -> (),
) -> Response {

    let name = name.into();

    Area::new(Id::unique(&name)).show(ctx, |ui| {

        Frame::window(ui.style())
            .inner_margin(Margin::same(2))
            .fill(if focused {Color32::LIGHT_BLUE} else {Color32::LIGHT_GRAY})
            .show(ui, |ui|
        {

            Frame::window(ui.style())
                .shadow(Shadow::NONE)
                .stroke(Stroke::NONE)
                .fill(Color32::TRANSPARENT)
                .show(ui, |ui|
            {
                ui.add(Label::new(RichText::new(name).color(
                    if focused {Color32::BLACK} else {Color32::GRAY}
                )));
            });

            Frame::window(ui.style())
                .shadow(Shadow::NONE)
                .corner_radius(CornerRadius::ZERO)
                .stroke(Stroke::NONE)
                .show(ui, |ui|
            {
                add_contents(ui);
            });

        });

    }).response

}

pub fn cursor_color(focused: bool) -> Color32 {
    if focused {Color32::LIGHT_BLUE} else {Color32::LIGHT_GRAY}
}
