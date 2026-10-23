use egui::*;
use librespectrum::{devs::{BusLogger, DeviceManager}, core::Ctrl};
use std::rc::Rc;

use crate::{
    theme::{self, BUS_LANES as LANES},
    windows::{SubWindow, draw_window}
};

const LANE_LABEL_W: f32 = 68.0;
const LANE_H: f32 = 24.0;
const LANE_MARGIN: f32 = 4.0;
const LANE_PADDING: f32 = 4.0;

fn draw_digital_lane(
    painter: &Painter,
    wave_rect: Rect,
    lane_top: f32,
    values: &[Option<bool>],
    color: Color32,
) {
    if values.len() < 2 {
        return;
    }

    let signal_stroke = Stroke::new(1.6_f32, color);
    let no_signal_stroke = Stroke::new(1.0_f32, theme::BUS_NO_SIGNAL);

    let y_high = lane_top + LANE_PADDING;
    let y_low = lane_top + LANE_H - LANE_PADDING;
    let y_mid = lane_top + LANE_H * 0.5;

    let step_x = wave_rect.width() / (values.len() - 1) as f32;

    for (idx, pair) in values.windows(2).enumerate() {
        let (prev, curr) = (pair[0], pair[1]);
        let x_prev = wave_rect.left() + idx as f32 * step_x;
        let x_curr = wave_rect.left() + (idx + 1) as f32 * step_x;

        match (prev, curr) {
            (Some(p), Some(c)) => { // Known state (0 or 1) or transition between them
                let y_prev = if p { y_high } else { y_low };
                let y_curr = if c { y_high } else { y_low };
                painter.hline(x_prev..=x_curr, y_prev, signal_stroke);
                if p != c {
                    painter.vline(x_curr, y_prev..=y_curr, signal_stroke);
                }
            }
            (Some(p), None) => { // Transition from known state (0 or 1) to unknown (high impedance state)
                let y_prev = if p { y_high } else { y_low };
                painter.hline(x_prev..=x_curr, y_prev, signal_stroke);
                painter.vline(x_curr, y_prev..=y_mid, signal_stroke);
            }
            (None, Some(c)) => { // Transition from unknown (high impedance state) to known state (0 or 1)
                let y_curr = if c { y_high } else { y_low };
                painter.hline(x_prev..=x_curr, y_mid, no_signal_stroke);
                painter.vline(x_curr, y_mid..=y_curr, signal_stroke);
            }
            (None, None) => { // No transition, we're in the unknown (high impedance) state
                painter.hline(x_prev..=x_curr, y_mid, no_signal_stroke);
            }
        }
    }
}

fn draw_bus_lane<T>(
    painter: &Painter,
    wave_rect: Rect,
    lane_top: f32,
    values: &[Option<T>],
    color: Color32,
) where
    T: Copy + PartialEq + std::fmt::UpperHex,
{
    if values.len() < 2 {
        return;
    }

    let hex_digits = std::mem::size_of::<T>() * 2;

    let signal_stroke = Stroke::new(1.6_f32, color);
    let no_signal_stroke = Stroke::new(1.0_f32, theme::BUS_NO_SIGNAL);

    let y_high = lane_top + LANE_PADDING;
    let y_low = lane_top + LANE_H - LANE_PADDING;
    let y_mid = lane_top + LANE_H * 0.5;

    let step_x = wave_rect.width() / (values.len() - 1) as f32;

    for (idx, pair) in values.windows(2).enumerate() {
        let (prev, curr) = (pair[0], pair[1]);
        let x_prev = wave_rect.left() + idx as f32 * step_x;
        let x_curr = wave_rect.left() + (idx + 1) as f32 * step_x;

        match (prev, curr) {
            (prev, Some(c)) if prev != Some(c) => { // Transition from unknown to known or to a different known state
                if prev.is_some() {
                    painter.hline(x_prev..=x_curr, y_high, signal_stroke);
                    painter.hline(x_prev..=x_curr, y_low, signal_stroke);
                } else {
                    painter.hline(x_prev..=x_curr, y_mid, no_signal_stroke);
                }
                painter.vline(x_curr, y_high..=y_low, signal_stroke);
                painter.text(
                    pos2(x_curr + 2.0, y_mid),
                    Align2::LEFT_CENTER,
                    format!("{:0width$X}", c, width = hex_digits),
                    FontId::monospace(10.0),
                    color,
                );
            },
            (Some(p), Some(c)) if p == c => { // No transition, same known state
                painter.hline(x_prev..=x_curr, y_high, signal_stroke);
                painter.hline(x_prev..=x_curr, y_low, signal_stroke);
            },
            (Some(_), None) => { // Transition from known to unknown state
                painter.hline(x_prev..=x_curr, y_high, signal_stroke);
                painter.vline(x_curr, y_low..=y_high, signal_stroke);
                painter.hline(x_prev..=x_curr, y_low, signal_stroke);
            },
            (None, None) => { // No transition, we're in the unknown (high impedance) state
                painter.hline(x_prev..=x_curr, y_mid, no_signal_stroke);
            },
            _ => {}
        }
    }
}

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
}

impl SubWindow for BusWindow {

    fn name(&self) -> String { String::from("Bus") }

    fn show(&mut self, ctx: &Context, focused: bool) -> Response {

        draw_window(self.name(), focused, ctx, |ui| {

            let readings: Vec<_> = self.logger.readings.borrow().iter_to_head().collect();

            let (rect, _) = {
                let total_h = (LANES.len() + 2) as f32 * (LANE_H + LANE_MARGIN) + 28.0;
                ui.allocate_exact_size(vec2(1600.0, total_h), Sense::hover())
            };

            let wave_rect = Rect::from_min_max(
                pos2(rect.left() + LANE_LABEL_W, rect.top() + 8.0),
                pos2(rect.right() - 8.0, rect.bottom() - 18.0),
            );

            let addr_values: Vec<Option<u16>> = readings.iter().map(|r| r.addr.map(|(_, v)| v)).collect();
            let data_values: Vec<Option<u8>> = readings.iter().map(|r| r.data.map(|(_, v)| v)).collect();

            let painter = ui.painter_at(rect);

            painter.rect_filled(rect, 4.0, theme::BUS_BACKGROUND);
            painter.rect_filled(wave_rect, 4.0, theme::BUS_WAVE_BACKGROUND);

            let mut y = rect.top() + 8.0;

            painter.text(
                pos2(rect.left() + 8.0, y + 2.0),
                Align2::LEFT_TOP,
                "ADDR",
                FontId::monospace(11.0),
                theme::TEXT,
            );
            draw_bus_lane(
                &painter,
                wave_rect,
                y,
                &addr_values,
                theme::BUS_ADDRESS,
            );
            y += LANE_H + LANE_MARGIN;

            painter.text(
                pos2(rect.left() + 8.0, y + 2.0),
                Align2::LEFT_TOP,
                "DATA",
                FontId::monospace(11.0),
                theme::TEXT,
            );
            draw_bus_lane(
                &painter,
                wave_rect,
                y,
                &data_values,
                theme::BUS_DATA,
            );
            y += LANE_H + LANE_MARGIN;

            for (name, color) in LANES {
                painter.text(
                    pos2(rect.left() + 8.0, y + 2.0),
                    Align2::LEFT_TOP,
                    name,
                    FontId::monospace(11.0),
                    theme::TEXT,
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

                draw_digital_lane(&painter, wave_rect, y, &values, color);
                y += LANE_H + LANE_MARGIN;
            }

            if let (Some(first), Some(last)) = (readings.first(), readings.last()) {
                painter.text(
                    pos2(wave_rect.left(), rect.bottom() - 4.0),
                    Align2::LEFT_BOTTOM,
                    format!("T={} ", first.htcycles / 2),
                    FontId::monospace(10.0),
                    theme::MUTED_TEXT,
                );
                painter.text(
                    pos2(wave_rect.right(), rect.bottom() - 4.0),
                    Align2::RIGHT_BOTTOM,
                    format!("T={} ", last.htcycles / 2),
                    FontId::monospace(10.0),
                    theme::MUTED_TEXT,
                );
            }

        })

    }

}
