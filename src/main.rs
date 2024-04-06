#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
// Allow pedantic lints that are a little _too_ pedantic.
#![allow(clippy::cast_precision_loss)]

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
                    Beat::Appear("John".to_string()),
                    Beat::Appear("Peter".to_string()),
                    Beat::Meet("John".to_string(), "Peter".to_string()),
                    Beat::Appear("Klaas".to_string()),
                    Beat::Meet("Peter".to_string(), "Klaas".to_string()),
                    Beat::Meet("Klaas".to_string(), "John".to_string()),
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
    /// How far each story beat progresses
    const BEAT_X_DISTANCE: f32 = 50.0;
    const STROKE_WIDTH: f32 = 3.0;

    // Keeps track of current "on stage" people, and their last y coordinate.
    let mut persons: HashMap<String, (Color32, f32)> = HashMap::new();
    let mut auto_color_index = 1;
    let mut x_coordinate = 10.0;

    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let paint = ui.painter();

            for beat in &story.beats {
                match beat {
                    Beat::Appear(name) => {
                        persons.insert(
                            name.clone(),
                            (auto_color(auto_color_index), auto_color_index as f32 * 20.0),
                        );
                        auto_color_index += 1;
                    }
                    Beat::Meet(one, other) => {
                        let x_old = x_coordinate;
                        x_coordinate += BEAT_X_DISTANCE;

                        // TODO (2024-04-06): Error handling? Or auto-create a new person.
                        let (color_one, y_one) = persons
                            .get(one)
                            .copied()
                            .expect("Person does not exist at this point");
                        let (color_other, y_other) = persons
                            .get_mut(other)
                            .expect("Person does not exist at this point");

                        paint.line_segment(
                            [Pos2::new(x_old, y_one), Pos2::new(x_coordinate, y_one)],
                            Stroke::new(STROKE_WIDTH, color_one),
                        );

                        let previous_y = *y_other;
                        // TODO (2024-04-06): Determine how far grouped people should be from each other.
                        *y_other = y_one + 5.0;

                        let middle_x = x_old + ((x_coordinate - x_old) / 2.0);

                        let bezier = CubicBezierShape::from_points_stroke(
                            [
                                Pos2::new(x_old, previous_y),
                                Pos2::new(middle_x, previous_y),
                                Pos2::new(middle_x, *y_other),
                                Pos2::new(x_coordinate, *y_other),
                            ],
                            false,
                            Color32::TRANSPARENT,
                            Stroke::new(STROKE_WIDTH, *color_other),
                        );

                        paint.add(bezier);
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
    Appear(String),
    Meet(String, String),
}

/// Algorithm from [egui_plot](https://github.com/emilk/egui/blob/master/crates/egui_plot/src/plot_ui.rs#L16)
fn auto_color(index: usize) -> Color32 {
    let golden_ratio = (5.0_f32.sqrt() - 1.0) / 2.0; // 0.61803398875
    let h = index as f32 * golden_ratio;
    Hsva::new(h, 0.85, 0.5, 1.0).into()
}
