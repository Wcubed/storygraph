#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
// Allow pedantic lints that are a little _too_ pedantic.
#![allow(clippy::cast_precision_loss)]

mod lex;
mod parse;
mod tokens;

use std::collections::HashMap;

use color_eyre::eyre::{eyre, Result};
use eframe::{
    egui::{self, Color32, Pos2, Stroke, Ui},
    epaint::{CubicBezierShape, Hsva},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Storygraph",
        native_options,
        Box::new(|cc| Box::new(GraphApp::new(cc))),
    )
    .map_err(|error| eyre!("Failed to start eframe window: {error}"))?;

    Ok(())
}

struct GraphApp {
    story: Story,
}

impl GraphApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            story: Story {
                beats: vec![
                    Beat::Groups(vec![vec!["klaas", "piet"], vec!["henk"]]),
                    Beat::Groups(vec![vec!["klaas"], vec!["piet", "henk"]]),
                ],
            },
        }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| display_story(ui, &self.story));
    }
}

fn display_story(ui: &mut Ui, story: &Story) {
    const X_PADDING: f32 = 10.0;
    const Y_PADDING: f32 = 10.0;
    /// How far each story beat progresses
    const BEAT_X_DISTANCE: f32 = 50.0;
    /// Distance between people in a group.
    const IN_GROUP_Y_DISTANCE: f32 = 6.0;
    /// Distance between groups.
    const INTER_GROUP_Y_DISTANCE: f32 = 24.0;
    const STROKE_WIDTH: f32 = 3.0;

    // Keeps track of current "on stage" people, and their last y coordinate.
    let mut persons: HashMap<&'static str, (Color32, f32)> = HashMap::new();
    let mut current_x = X_PADDING;

    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let paint = ui.painter();

            for beat in &story.beats {
                match beat {
                    Beat::Groups(groups) => {
                        let mut current_y = Y_PADDING;
                        let old_x = current_x;
                        current_x += BEAT_X_DISTANCE;

                        // Middle position between old_x and current_x, for drawing a bezier curve.
                        let middle_x = old_x + (current_x - old_x) / 2.0;

                        for group in groups {
                            for (index, name) in group.iter().enumerate() {
                                let (color, old_y) = persons
                                    .get(name)
                                    .copied()
                                    // A new person's line appears at the desired y location.
                                    .unwrap_or((auto_color(persons.len()), current_y));
                                persons.insert(name, (color, current_y));

                                let bezier = CubicBezierShape::from_points_stroke(
                                    [
                                        Pos2::new(old_x, old_y),
                                        Pos2::new(middle_x, old_y),
                                        Pos2::new(middle_x, current_y),
                                        Pos2::new(current_x, current_y),
                                    ],
                                    false,
                                    Color32::TRANSPARENT,
                                    Stroke::new(STROKE_WIDTH, color),
                                );

                                paint.add(bezier);

                                if index < group.len() {
                                    current_y += IN_GROUP_Y_DISTANCE;
                                }
                            }

                            current_y += INTER_GROUP_Y_DISTANCE;
                        }
                    }
                }
            }
        });
}

#[derive(Debug)]
struct Story {
    beats: Vec<Beat>,
}

#[derive(Debug)]
enum Beat {
    /// A list of all groups of currently existing characters, in the order that they should be displayed.
    Groups(Vec<Vec<&'static str>>),
}

/// Algorithm from [egui_plot](https://github.com/emilk/egui/blob/master/crates/egui_plot/src/plot_ui.rs#L16)
fn auto_color(index: usize) -> Color32 {
    let golden_ratio = (5.0_f32.sqrt() - 1.0) / 2.0; // 0.61803398875
    let h = index as f32 * golden_ratio;
    Hsva::new(h, 0.85, 0.5, 1.0).into()
}
