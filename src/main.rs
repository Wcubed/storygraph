#![deny(unsafe_code)]

mod lex;
mod parse;
mod tokens;

use std::collections::HashMap;

use color_eyre::eyre::{eyre, Result};
use eframe::{
    egui::{self, Align2, Color32, FontId, Pos2, Sense, Stroke, Ui, Vec2},
    epaint::{text, CubicBezierShape, Hsva},
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
        let mut colors = HashMap::new();
        colors.insert("t-rex", Color32::RED);
        colors.insert("raptor1", Color32::RED);
        colors.insert("raptor2", Color32::RED);
        colors.insert("raptor3", Color32::RED);

        Self {
            story: Story {
                // Test data: Jurassic park, taken from the xkcd comic.
                colors,
                beats: vec![
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        vec!["raptor1", "raptor2", "raptor3"],
                        vec!["malcolm"],
                        vec!["grant", "sattler"],
                        vec!["gennaro"],
                        vec!["hammond"],
                        vec!["kids"],
                        vec!["muldoon", "arnold", "nedry"],
                    ]),
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        vec!["raptor1", "raptor2", "raptor3"],
                        vec!["malcolm", "gennaro"],
                        vec!["grant", "sattler", "hammond"],
                        vec!["kids"],
                        vec!["muldoon", "arnold", "nedry"],
                    ]),
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        vec!["raptor1", "raptor2", "raptor3"],
                        vec!["malcolm", "gennaro", "grant", "sattler", "hammond"],
                        vec!["kids"],
                        vec!["muldoon", "arnold", "nedry"],
                    ]),
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        vec!["raptor1", "raptor2", "raptor3"],
                        vec![
                            "malcolm", "gennaro", "grant", "sattler", "hammond", "kids", "muldoon",
                            "arnold", "nedry",
                        ],
                    ]),
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        vec!["raptor1", "raptor2", "raptor3"],
                        vec![
                            "malcolm", "gennaro", "grant", "sattler", "hammond", "kids", "muldoon",
                        ],
                        vec!["arnold", "nedry"],
                    ]),
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        vec!["raptor1", "raptor2", "raptor3"],
                        vec!["malcolm", "gennaro", "grant", "sattler", "kids"],
                        vec!["hammond", "muldoon", "arnold"],
                        vec!["nedry"],
                    ]),
                    Beat::Groups(vec![
                        // Attack on cars
                        vec!["t-rex", "malcolm", "gennaro", "grant", "kids"],
                        vec!["raptor1", "raptor2", "raptor3"],
                        vec!["sattler", "hammond", "muldoon", "arnold"],
                        vec!["nedry"],
                    ]),
                    Beat::Groups(vec![
                        vec!["grant", "kids"],
                        // Must go faster
                        vec!["t-rex", "malcolm", "sattler", "muldoon"],
                        vec!["raptor1", "raptor2", "raptor3"],
                        vec!["arnold", "hammond"],
                        // Nedry eaten
                        vec!["dilophosaurus", "nedry"],
                    ]),
                    Beat::Groups(vec![
                        // Gallimimus
                        vec!["grant", "kids", "t-rex"],
                        vec!["raptor1", "raptor2"],
                        vec!["raptor3"],
                        vec!["malcolm", "sattler", "muldoon", "arnold", "hammond"],
                        vec!["dilophosaurus"],
                    ]),
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        vec!["grant", "kids"],
                        vec!["raptor1", "raptor2"],
                        // Shed
                        vec!["raptor3", "arnold"],
                        vec!["malcolm", "sattler", "muldoon", "hammond"],
                    ]),
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        vec!["kids", "grant"],
                        // Clever girl
                        vec!["raptor1", "muldoon", "raptor2"],
                        vec!["raptor3", "sattler"],
                        vec!["malcolm", "hammond"],
                    ]),
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        // Kitchen
                        vec!["raptor1", "kids", "raptor2"],
                        vec!["grant", "sattler"],
                        vec!["raptor3"],
                        vec!["malcolm", "hammond"],
                    ]),
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        vec!["raptor1"],
                        vec!["kids", "grant", "sattler"],
                        vec!["raptor3"],
                        vec!["malcolm", "hammond"],
                    ]),
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        // Visitor center
                        vec!["raptor1", "kids", "grant", "sattler", "raptor3"],
                        vec!["malcolm", "hammond"],
                    ]),
                    Beat::Groups(vec![
                        // Visitor center
                        vec!["t-rex", "raptor1", "kids", "grant", "sattler", "raptor3"],
                        vec!["malcolm", "hammond"],
                    ]),
                    Beat::Groups(vec![
                        vec!["t-rex"],
                        vec!["kids", "grant", "sattler", "malcolm", "hammond"],
                    ]),
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
    const CURVE_X_DISTANCE: f32 = 70.0;
    const STRAIGHT_X_DISTANCE: f32 = 50.0;
    /// Distance between people in a group.
    const IN_GROUP_Y_DISTANCE: f32 = 10.0;
    /// Distance between groups.
    const INTER_GROUP_Y_DISTANCE: f32 = 50.0;
    const STROKE_WIDTH: f32 = 5.0;
    const BACKGROUND_STROKE_WIDTH: f32 = STROKE_WIDTH + 4.0;
    const FONT_SIZE: f32 = 10.0;
    const NAME_EVERY_N_STRAIGHTS: usize = 4;

    let background_stroke = Stroke::new(BACKGROUND_STROKE_WIDTH, ui.visuals().window_fill);

    let mut colors = story.colors.clone();
    // Keeps track of current "on stage" people, and their last y coordinate.
    let mut persons: HashMap<&'static str, f32> = HashMap::new();

    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let width = story.beats.len() as f32 * (CURVE_X_DISTANCE + STRAIGHT_X_DISTANCE);
            let (rect, _) = ui.allocate_at_least(
                // TODO (2024-04-07): How do we determine how tall we should allocate?
                Vec2::new(width, 10.0),
                Sense::hover(),
            );

            let mut current_x = rect.left() + X_PADDING;

            let paint = ui.painter();
            // We paint the text as last, so it draws over lines.
            let mut texts = vec![];

            for (beat_num, beat) in story.beats.iter().enumerate() {
                match beat {
                    Beat::Groups(groups) => {
                        let mut current_y = Y_PADDING;
                        let old_x = current_x;
                        current_x += CURVE_X_DISTANCE;

                        // Middle position between old_x and current_x, for drawing a bezier curve.
                        let middle_x = old_x + (current_x - old_x) / 2.0;

                        for group in groups {
                            for (index, name) in group.iter().enumerate() {
                                // TODO (2024-04-07): Fade in a new persons line.
                                let old_y = persons
                                    .get(name)
                                    .copied()
                                    // A new person's line appears at the desired y location.
                                    .unwrap_or(current_y);
                                persons.insert(name, current_y);

                                let known_color_len = colors.len();
                                let color =
                                    *colors.entry(name).or_insert(auto_color(known_color_len));

                                let stroke = Stroke::new(STROKE_WIDTH, color);
                                let points = [
                                    Pos2::new(old_x, old_y),
                                    Pos2::new(middle_x, old_y),
                                    Pos2::new(middle_x, current_y),
                                    Pos2::new(current_x, current_y),
                                ];

                                let background_bezier = CubicBezierShape::from_points_stroke(
                                    points,
                                    false,
                                    Color32::TRANSPARENT,
                                    background_stroke,
                                );
                                paint.add(background_bezier);

                                let bezier = CubicBezierShape::from_points_stroke(
                                    points,
                                    false,
                                    Color32::TRANSPARENT,
                                    stroke,
                                );
                                paint.add(bezier);

                                // Add a straight segment in between each curved section.
                                let old_x = current_x;
                                let final_x = old_x + STRAIGHT_X_DISTANCE;
                                paint.line_segment(
                                    [Pos2::new(old_x, current_y), Pos2::new(final_x, current_y)],
                                    stroke,
                                );

                                if beat_num % NAME_EVERY_N_STRAIGHTS == 0 {
                                    // Add the name of the person for later drawing
                                    let galley = paint.layout_no_wrap(
                                        name.to_string(),
                                        FontId::proportional(FONT_SIZE),
                                        ui.visuals().text_color(),
                                    );
                                    let rect = Align2::LEFT_CENTER
                                        .anchor_size(Pos2::new(old_x, current_y), galley.size());
                                    texts.push((galley, rect));
                                }

                                if index < group.len() {
                                    current_y += IN_GROUP_Y_DISTANCE;
                                }
                            }

                            current_y += INTER_GROUP_Y_DISTANCE;
                        }

                        current_x += STRAIGHT_X_DISTANCE;
                    }
                }
            }

            // First draw the backgrounds for the texts, that way, one text background does not
            // draw over another text.
            for (_, rect) in texts.iter() {
                paint.rect_filled(*rect, 0.0, ui.visuals().window_fill);
            }
            for (galley, rect) in texts.into_iter() {
                paint.galley(rect.min, galley, ui.visuals().text_color());
            }
        });
}

#[derive(Debug)]
struct Story {
    /// Map of persons and the color of their line.
    /// If a person is not in the map, they will get a color auto-assigned.
    colors: HashMap<&'static str, Color32>,
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
