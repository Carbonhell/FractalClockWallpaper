use std::f32::consts::TAU;

use eframe::egui::{CentralPanel, Color32, CtxRef, emath, emath::pos2, Painter, Pos2, Rect, Shape, Ui, Vec2, Frame};
use eframe::epi::App;
use eframe::wasm_bindgen::{self, prelude::*};
use eframe::epi;

#[wasm_bindgen]
pub struct FractalClockSettings {
    pub zoom: Option<f32>,
    pub start_line_width: Option<f32>,
    pub depth: Option<usize>,
    pub length_factor: Option<f32>,
    pub luminance_factor: Option<f32>,
    pub width_factor: Option<f32>,
}

#[wasm_bindgen]
impl FractalClockSettings {
    pub fn new(zoom: Option<f32>, start_line_width: Option<f32>, depth: Option<usize>, length_factor: Option<f32>, luminance_factor: Option<f32>, width_factor: Option<f32>) -> FractalClockSettings {
        FractalClockSettings {
            zoom,
            start_line_width,
            depth,
            length_factor,
            luminance_factor,
            width_factor,
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub struct FractalClock {
    paused: bool,
    time: f64,
    pub zoom: f32,
    pub start_line_width: f32,
    pub depth: usize,
    pub length_factor: f32,
    pub luminance_factor: f32,
    pub width_factor: f32,
    line_count: usize,
}

impl Default for FractalClock {
    fn default() -> Self {
        Self {
            paused: false,
            time: 0.0,
            zoom: 0.25,
            start_line_width: 2.5,
            depth: 9,
            length_factor: 0.8,
            luminance_factor: 0.8,
            width_factor: 0.9,
            line_count: 0,
        }
    }
}

impl App for FractalClock {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame) {
        CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| self.ui(ui, crate::seconds_since_midnight()));
    }

    fn name(&self) -> &str {
        "🕑 Fractal Clock"
    }
}

impl FractalClock {
    pub fn ui(&mut self, ui: &mut Ui, seconds_since_midnight: Option<f64>) {
        if !self.paused {
            self.time = seconds_since_midnight.unwrap_or_else(|| ui.input().time);
            ui.ctx().request_repaint();
        }

        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        );
        self.paint(&painter);
        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(painter.clip_rect());
    }

    fn paint(&mut self, painter: &Painter) {
        struct Hand {
            length: f32,
            angle: f32,
            vec: Vec2,
        }

        impl Hand {
            fn from_length_angle(length: f32, angle: f32) -> Self {
                Self {
                    length,
                    angle,
                    vec: length * Vec2::angled(angle),
                }
            }
        }

        let angle_from_period =
            |period| TAU * (self.time.rem_euclid(period) / period) as f32 - TAU / 4.0;

        let hands = [
            // Second hand:
            Hand::from_length_angle(self.length_factor, angle_from_period(60.0)),
            // Minute hand:
            Hand::from_length_angle(self.length_factor, angle_from_period(60.0 * 60.0)),
            // Hour hand:
            Hand::from_length_angle(0.5, angle_from_period(12.0 * 60.0 * 60.0)),
        ];

        let mut shapes: Vec<Shape> = Vec::new();

        let rect = painter.clip_rect();
        let to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
            rect,
        );

        let mut paint_line = |points: [Pos2; 2], color: Color32, width: f32| {
            let line = [to_screen * points[0], to_screen * points[1]];

            // culling
            if rect.intersects(Rect::from_two_pos(line[0], line[1])) {
                shapes.push(Shape::line_segment(line, (width, color)));
            }
        };

        let hand_rotations = [
            hands[0].angle - hands[2].angle + TAU / 2.0,
            hands[1].angle - hands[2].angle + TAU / 2.0,
        ];

        let hand_rotors = [
            hands[0].length * emath::Rot2::from_angle(hand_rotations[0]),
            hands[1].length * emath::Rot2::from_angle(hand_rotations[1]),
        ];

        #[derive(Clone, Copy)]
        struct Node {
            pos: Pos2,
            dir: Vec2,
        }

        let mut nodes = Vec::new();

        let mut width = self.start_line_width;

        for (i, hand) in hands.iter().enumerate() {
            let center = pos2(0.0, 0.0);
            let end = center + hand.vec;
            paint_line([center, end], Color32::from_additive_luminance(255), width);
            if i < 2 {
                nodes.push(Node {
                    pos: end,
                    dir: hand.vec,
                });
            }
        }

        let mut luminance = 0.7; // Start dimmer than main hands

        let mut new_nodes = Vec::new();
        for _ in 0..self.depth {
            new_nodes.clear();
            new_nodes.reserve(nodes.len() * 2);

            luminance *= self.luminance_factor;
            width *= self.width_factor;

            let luminance_u8 = (255.0 * luminance).round() as u8;
            if luminance_u8 == 0 {
                break;
            }

            for &rotor in &hand_rotors {
                for a in &nodes {
                    let new_dir = rotor * a.dir;
                    let b = Node {
                        pos: a.pos + new_dir,
                        dir: new_dir,
                    };
                    paint_line(
                        [a.pos, b.pos],
                        Color32::from_additive_luminance(luminance_u8),
                        width,
                    );
                    new_nodes.push(b);
                }
            }

            std::mem::swap(&mut nodes, &mut new_nodes);
        }
        self.line_count = shapes.len();
        painter.extend(shapes);
    }

    pub fn apply_clock_settings(&mut self, settings: &FractalClockSettings) {
        self.zoom = settings.zoom.unwrap_or(self.zoom);
        self.start_line_width = settings.start_line_width.unwrap_or(self.start_line_width);
        self.depth = settings.depth.unwrap_or(self.depth);
        self.length_factor = settings.length_factor.unwrap_or(self.length_factor);
        self.luminance_factor = settings.luminance_factor.unwrap_or(self.luminance_factor);
        self.width_factor = settings.width_factor.unwrap_or(self.width_factor);
    }
}