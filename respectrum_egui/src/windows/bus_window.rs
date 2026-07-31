use egui::*;
use librespectrum::{devs::{BusLogger, DeviceManager}, core::Ctrl};
use std::rc::Rc;

use super::{SubWindow, draw_window};

pub struct BusWindow {
    logger: Rc<BusLogger>,
}

impl BusWindow {
    pub fn new(logger: &Rc<BusLogger>, device_manager: &Rc<DeviceManager>) -> Self {
        let _ = device_manager;
        Self {
            logger: Rc::clone(logger),
        }
    }

    fn draw_digital_lane(
        &self,
        painter: &Painter,
        wave_rect: Rect,
        lane_top: f32,
        values: &[Option<bool>],
        color: Color32,
        stroke_w: f32,
    ) {
        if values.is_empty() {
            return;
        }

        let y_high = lane_top + 4.0;
        let y_low = lane_top + LANE_H - 4.0;
        let y_unknown = lane_top + LANE_H * 0.5;
        let step_x = if values.len() > 1 {
            wave_rect.width() / (values.len() - 1) as f32
        } else {
            0.0
        };

        let lane_rect = Rect::from_min_max(
            pos2(wave_rect.left(), lane_top),
            pos2(wave_rect.right(), lane_top + LANE_H),
        );
        painter.rect_stroke(
            lane_rect,
            0.0,
            Stroke::new(1.0, Color32::from_gray(50)),
        );

        let mut prev = values[0];
        for (idx, curr) in values.iter().copied().enumerate().skip(1) {
            let x_prev = wave_rect.left() + (idx - 1) as f32 * step_x;
            let x_curr = wave_rect.left() + idx as f32 * step_x;

            match (prev, curr) {
                (Some(p), Some(c)) => {
                    let y_prev = if p { y_high } else { y_low };
                    let y_curr = if c { y_high } else { y_low };
                    painter.line_segment(
                        [pos2(x_prev, y_prev), pos2(x_curr, y_prev)],
                        Stroke::new(stroke_w, color),
                    );
                    if p != c {
                        painter.line_segment(
                            [pos2(x_curr, y_prev), pos2(x_curr, y_curr)],
                            Stroke::new(stroke_w, color),
                        );
                    }
                }
                (Some(p), None) => {
                    let y_prev = if p { y_high } else { y_low };
                    painter.line_segment(
                        [pos2(x_prev, y_prev), pos2(x_curr, y_prev)],
                        Stroke::new(stroke_w, color),
                    );
                    painter.line_segment(
                        [pos2(x_curr, y_prev), pos2(x_curr, y_unknown)],
                        Stroke::new(1.0, Color32::DARK_GRAY),
                    );
                }
                (None, Some(c)) => {
                    let y_curr = if c { y_high } else { y_low };
                    painter.line_segment(
                        [pos2(x_prev, y_unknown), pos2(x_curr, y_unknown)],
                        Stroke::new(1.0, Color32::DARK_GRAY),
                    );
                    painter.line_segment(
                        [pos2(x_curr, y_unknown), pos2(x_curr, y_curr)],
                        Stroke::new(1.0, Color32::DARK_GRAY),
                    );
                }
                (None, None) => {
                    painter.line_segment(
                        [pos2(x_prev, y_unknown), pos2(x_curr, y_unknown)],
                        Stroke::new(1.0, Color32::DARK_GRAY),
                    );
                }
            }

            prev = curr;
        }
    }

    fn draw_bus_lane_u16(
        &self,
        painter: &Painter,
        wave_rect: Rect,
        lane_top: f32,
        values: &[Option<u16>],
        color: Color32,
    ) {
        let lane_rect = Rect::from_min_max(
            pos2(wave_rect.left(), lane_top),
            pos2(wave_rect.right(), lane_top + LANE_H),
        );
        painter.rect_stroke(
            lane_rect,
            0.0,
            Stroke::new(1.0, Color32::from_gray(50)),
        );

        if values.is_empty() {
            return;
        }

        let y_mid = lane_top + LANE_H * 0.5;
        painter.line_segment(
            [pos2(wave_rect.left(), y_mid), pos2(wave_rect.right(), y_mid)],
            Stroke::new(1.0, Color32::from_gray(70)),
        );

        let step_x = if values.len() > 1 {
            wave_rect.width() / (values.len() - 1) as f32
        } else {
            0.0
        };

        let mut last_labeled: Option<u16> = None;
        for (idx, value) in values.iter().copied().enumerate() {
            let x = wave_rect.left() + idx as f32 * step_x;
            if let Some(v) = value {
                if last_labeled != Some(v) {
                    painter.line_segment(
                        [pos2(x, lane_top + 3.0), pos2(x, lane_top + LANE_H - 3.0)],
                        Stroke::new(1.0, color),
                    );
                    painter.text(
                        pos2((x + 2.0).min(wave_rect.right() - 36.0), lane_top + 3.0),
                        Align2::LEFT_TOP,
                        format!("{:04X}", v),
                        FontId::monospace(10.0),
                        color,
                    );
                    last_labeled = Some(v);
                }
            }
        }
    }

    fn draw_bus_lane_u8(
        &self,
        painter: &Painter,
        wave_rect: Rect,
        lane_top: f32,
        values: &[Option<u8>],
        color: Color32,
    ) {
        let lane_rect = Rect::from_min_max(
            pos2(wave_rect.left(), lane_top),
            pos2(wave_rect.right(), lane_top + LANE_H),
        );
        painter.rect_stroke(
            lane_rect,
            0.0,
            Stroke::new(1.0, Color32::from_gray(50)),
        );

        if values.is_empty() {
            return;
        }

        let y_mid = lane_top + LANE_H * 0.5;
        painter.line_segment(
            [pos2(wave_rect.left(), y_mid), pos2(wave_rect.right(), y_mid)],
            Stroke::new(1.0, Color32::from_gray(70)),
        );

        let step_x = if values.len() > 1 {
            wave_rect.width() / (values.len() - 1) as f32
        } else {
            0.0
        };

        let mut last_labeled: Option<u8> = None;
        for (idx, value) in values.iter().copied().enumerate() {
            let x = wave_rect.left() + idx as f32 * step_x;
            if let Some(v) = value {
                if last_labeled != Some(v) {
                    painter.line_segment(
                        [pos2(x, lane_top + 3.0), pos2(x, lane_top + LANE_H - 3.0)],
                        Stroke::new(1.0, color),
                    );
                    painter.text(
                        pos2((x + 2.0).min(wave_rect.right() - 26.0), lane_top + 3.0),
                        Align2::LEFT_TOP,
                        format!("{:02X}", v),
                        FontId::monospace(10.0),
                        color,
                    );
                    last_labeled = Some(v);
                }
            }
        }
    }
}

const LANES: [(&str, egui::Color32); 13] = [
    ("RD", Color32::LIGHT_GREEN),
    ("WR", Color32::LIGHT_RED),
    ("MREQ", Color32::from_rgb(255, 196, 128)),
    ("IORQ", Color32::from_rgb(255, 226, 120)),
    ("RFSH", Color32::from_rgb(140, 220, 255)),
    ("M1", Color32::from_rgb(160, 255, 170)),
    ("BUSRQ", Color32::from_rgb(255, 170, 120)),
    ("BUSAK", Color32::from_rgb(255, 210, 120)),
    ("WAIT", Color32::from_rgb(255, 140, 140)),
    ("HALT", Color32::from_rgb(230, 170, 255)),
    ("INT", Color32::from_rgb(130, 220, 255)),
    ("NMI", Color32::from_rgb(130, 255, 220)),
    ("RESET", Color32::from_rgb(255, 150, 180)),
];

const LABEL_W: f32 = 68.0;
const LANE_H: f32 = 24.0;
const LANE_GAP: f32 = 4.0;

impl SubWindow for BusWindow {

    fn name(&self) -> String { String::from("Bus") }

    fn show(&mut self, ctx: &Context, focused: bool) -> Response {

        draw_window(self.name(), focused, ctx, |ui| {

            let readings: Vec<_> = self.logger.readings.borrow().iter_to_tail().take(96).collect();
            if readings.is_empty() {
                ui.colored_label(Color32::GRAY, "No bus samples yet");
                return;
            }

            ui.label(format!(
                "Oscilloscope view: {} samples (oldest -> newest)",
                readings.len()
            ));

            let total_h = 24.0
                + LANE_H
                + LANE_GAP
                + LANE_H
                + LANE_GAP
                + LANES.len() as f32 * (LANE_H + LANE_GAP)
                + 28.0;
            let desired = vec2(ui.available_width().max(320.0), total_h);
            let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
            let painter = ui.painter_at(rect);

            painter.rect_filled(rect, 4.0, Color32::from_rgb(17, 20, 24));
            painter.rect_stroke(
                rect,
                4.0,
                Stroke::new(1.0, Color32::from_gray(55)),
            );

            let mut y = rect.top() + 8.0;
            let wave_rect = Rect::from_min_max(
                pos2(rect.left() + LABEL_W, rect.top() + 8.0),
                pos2(rect.right() - 8.0, rect.bottom() - 18.0),
            );

            let addr_values: Vec<Option<u16>> = readings.iter().map(|r| r.addr.map(|(_, v)| v)).collect();
            let data_values: Vec<Option<u8>> = readings.iter().map(|r| r.data.map(|(_, v)| v)).collect();

            painter.text(
                pos2(rect.left() + 8.0, y + 2.0),
                Align2::LEFT_TOP,
                "ADDR",
                FontId::monospace(11.0),
                Color32::LIGHT_GRAY,
            );
            self.draw_bus_lane_u16(
                &painter,
                wave_rect,
                y,
                &addr_values,
                Color32::from_rgb(255, 220, 150),
            );
            y += LANE_H + LANE_GAP;

            painter.text(
                pos2(rect.left() + 8.0, y + 2.0),
                Align2::LEFT_TOP,
                "DATA",
                FontId::monospace(11.0),
                Color32::LIGHT_GRAY,
            );
            self.draw_bus_lane_u8(
                &painter,
                wave_rect,
                y,
                &data_values,
                Color32::from_rgb(180, 230, 255),
            );
            y += LANE_H + LANE_GAP;

            for (name, color) in LANES {
                painter.text(
                    pos2(rect.left() + 8.0, y + 2.0),
                    Align2::LEFT_TOP,
                    name,
                    FontId::monospace(11.0),
                    Color32::LIGHT_GRAY,
                );

                let values: Vec<Option<bool>> = match name {
                    "RD" => readings
                        .iter()
                        .map(|r| r.ctrl.map(|(_, c)| c.contains(Ctrl::RD)))
                        .collect(),
                    "WR" => readings
                        .iter()
                        .map(|r| r.ctrl.map(|(_, c)| c.contains(Ctrl::WR)))
                        .collect(),
                    "MREQ" => readings
                        .iter()
                        .map(|r| r.ctrl.map(|(_, c)| c.contains(Ctrl::MREQ)))
                        .collect(),
                    "IORQ" => readings
                        .iter()
                        .map(|r| r.ctrl.map(|(_, c)| c.contains(Ctrl::IORQ)))
                        .collect(),
                    "RFSH" => readings
                        .iter()
                        .map(|r| r.ctrl.map(|(_, c)| c.contains(Ctrl::RFSH)))
                        .collect(),
                    "M1" => readings.iter().map(|r| r.m1.map(|(_, v)| v)).collect(),
                    "BUSRQ" => readings.iter().map(|r| r.busrq.map(|(_, v)| v)).collect(),
                    "BUSAK" => readings.iter().map(|r| r.busak.map(|(_, v)| v)).collect(),
                    "WAIT" => readings.iter().map(|r| r.wait.map(|(_, v)| v)).collect(),
                    "HALT" => readings.iter().map(|r| r.halt.map(|(_, v)| v)).collect(),
                    "INT" => readings.iter().map(|r| r.int.map(|(_, v)| v)).collect(),
                    "NMI" => readings.iter().map(|r| r.nmi.map(|(_, v)| v)).collect(),
                    "RESET" => readings.iter().map(|r| r.reset.map(|(_, v)| v)).collect(),
                    _ => Vec::new(),
                };

                self.draw_digital_lane(&painter, wave_rect, y, &values, color, 1.6);
                y += LANE_H + LANE_GAP;
            }

            if let (Some(first), Some(last)) = (readings.first(), readings.last()) {
                painter.text(
                    pos2(wave_rect.left(), rect.bottom() - 4.0),
                    Align2::LEFT_BOTTOM,
                    format!("T={} ", first.htcycles / 2),
                    FontId::monospace(10.0),
                    Color32::GRAY,
                );
                painter.text(
                    pos2(wave_rect.right(), rect.bottom() - 4.0),
                    Align2::RIGHT_BOTTOM,
                    format!("T={} ", last.htcycles / 2),
                    FontId::monospace(10.0),
                    Color32::GRAY,
                );
            }

        })

    }

}
