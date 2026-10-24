use egui::{Color32, Context, Stroke, Theme, Visuals};

mod palette {
    use egui::Color32;
    pub const BG_HARD: Color32 = Color32::from_rgb(0x1d, 0x20, 0x21);
    pub const BG: Color32 = Color32::from_rgb(0x28, 0x28, 0x28);
    pub const BG1: Color32 = Color32::from_rgb(0x3c, 0x38, 0x36);
    pub const BG2: Color32 = Color32::from_rgb(0x50, 0x49, 0x45);
    pub const BG3: Color32 = Color32::from_rgb(0x66, 0x5c, 0x54);
    pub const FG: Color32 = Color32::from_rgb(0xeb, 0xdb, 0xb2);
    pub const FG2: Color32 = Color32::from_rgb(0xd5, 0xc4, 0xa1);
    pub const FG3: Color32 = Color32::from_rgb(0xbd, 0xae, 0x93);
    pub const GRAY: Color32 = Color32::from_rgb(0x92, 0x83, 0x74);
    pub const RED: Color32 = Color32::from_rgb(0xfb, 0x49, 0x34);
    pub const GREEN: Color32 = Color32::from_rgb(0xb8, 0xbb, 0x26);
    pub const YELLOW: Color32 = Color32::from_rgb(0xfa, 0xbd, 0x2f);
    pub const BLUE: Color32 = Color32::from_rgb(0x83, 0xa5, 0x98);
    pub const PURPLE: Color32 = Color32::from_rgb(0xd3, 0x86, 0x9b);
    pub const AQUA: Color32 = Color32::from_rgb(0x8e, 0xc0, 0x7c);
    pub const ORANGE: Color32 = Color32::from_rgb(0xfe, 0x80, 0x19);
}

use palette::*;

pub const TEXT: Color32 = FG;
pub const MUTED_TEXT: Color32 = FG2;
pub const FLAG_SET: Color32 = RED;
pub const FLAG_UNSET: Color32 = GRAY;
pub const CURRENT_INSTRUCTION: Color32 = YELLOW;
pub const TRANSPARENT: Color32 = Color32::TRANSPARENT;

pub const BUS_BACKGROUND: Color32 = BG_HARD;
pub const BUS_WAVE_BACKGROUND: Color32 = BG;
pub const BUS_NO_SIGNAL: Color32 = GRAY;
pub const BUS_ADDRESS: Color32 = YELLOW;
pub const BUS_DATA: Color32 = BLUE;
pub const BUS_LANES: [(&str, Color32); 13] = [
    ("RD", GREEN),
    ("WR", RED),
    ("MREQ", ORANGE),
    ("IORQ", YELLOW),
    ("RFSH", BLUE),
    ("M1", AQUA),
    ("BUSRQ", ORANGE),
    ("BUSAK", YELLOW),
    ("WAIT", RED),
    ("HALT", PURPLE),
    ("INT", BLUE),
    ("NMI", AQUA),
    ("RESET", PURPLE),
];

pub fn window_header(focused: bool) -> Color32 {
    if focused { AQUA } else { BG1 }
}

pub fn window_title(focused: bool) -> Color32 {
    if focused { BG } else { FG3 }
}

pub fn cursor_color(focused: bool) -> Color32 {
    if focused { BG2 } else { BG1 }
}

pub fn visuals() -> Visuals {
    let mut visuals = Visuals::dark();
    visuals.weak_text_color = Some(MUTED_TEXT);
    visuals.panel_fill = BG_HARD;
    visuals.window_fill = BG;
    visuals.window_stroke = Stroke::new(1.0, BG3);
    visuals.window_shadow.color = BG_HARD.gamma_multiply(0.3);
    visuals.popup_shadow.color = BG_HARD.gamma_multiply(0.3);
    visuals.window_shadow.blur = 8;
    visuals.window_shadow.spread = 0;
    visuals.faint_bg_color = BG1;
    visuals.extreme_bg_color = BG_HARD;
    visuals.text_edit_bg_color = Some(BG_HARD);
    visuals.code_bg_color = BG1;
    visuals.hyperlink_color = BLUE;
    visuals.warn_fg_color = YELLOW;
    visuals.error_fg_color = RED;
    visuals.selection.bg_fill = BG2;
    visuals.selection.stroke = Stroke::new(1.0, FG);
    visuals.text_cursor.stroke.color = AQUA;
    visuals.ime_composition.active_underline_stroke.color = AQUA;
    visuals.ime_composition.inactive_underline_stroke.color = GRAY;

    for widget in [
        &mut visuals.widgets.noninteractive,
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
    ] {
        widget.bg_fill = BG1;
        widget.weak_bg_fill = BG1;
        widget.bg_stroke.color = BG3;
        widget.fg_stroke.color = TEXT;
    }
    visuals.widgets.noninteractive.bg_fill = BG;
    visuals.widgets.noninteractive.weak_bg_fill = BG;
    visuals.widgets.hovered.bg_fill = BG2;
    visuals.widgets.hovered.weak_bg_fill = BG2;
    visuals.widgets.hovered.bg_stroke.color = AQUA;
    visuals.widgets.active.bg_fill = BG2;
    visuals.widgets.active.weak_bg_fill = BG2;
    visuals.widgets.active.bg_stroke.color = AQUA;
    visuals.widgets.active.fg_stroke.color = AQUA;
    visuals.widgets.open.bg_stroke.color = AQUA;
    visuals
}

pub fn apply(ctx: &Context) {
    ctx.set_theme(Theme::Dark);
    ctx.all_styles_mut(|style| {
        style.visuals = visuals();
        style.override_text_style = Some(egui::TextStyle::Monospace);
        style.interaction.selectable_labels = false;
    });
}
