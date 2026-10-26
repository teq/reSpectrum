use eframe::egui::*;

use crate::theme;

mod bus_window;
pub use bus_window::BusWindow;
mod cpu_window;
pub use cpu_window::CpuWindow;
mod disassm_window;
pub use disassm_window::DisassmWindow;
mod display_window;
pub use display_window::DisplayWindow;
mod memory_window;
pub use memory_window::MemoryWindow;

pub trait SubWindow {

    /// Window name
    fn name(&self) -> String;

    /// Window draw function
    fn show(&mut self, ui: &mut Ui, open: &mut bool) -> Option<Response>;

}

/// Draw widow with title
pub fn draw_window(
    name: &String,
    ui: &mut Ui,
    open: &mut bool,
    add_contents: impl FnOnce(&mut Ui, bool) -> (),
) -> Option<Response> {

    let id = Id::unique(name);

    let focused = ui.ctx().top_layer_id().map(|l_id| l_id.id) == Some(id);

    Window::new(RichText::new(name).color(theme::window_title(focused)))
        .id(id)
        .open(open)
        .resizable(false)
        .collapsible(false)
        .constrain_to(ui.available_rect_before_wrap())
        .title_frame(Frame::window(ui.style()).fill(theme::window_header(focused)))
        .show(ui.ctx(), |ui| add_contents(ui, focused))
        .map(|response| response.response)

}
