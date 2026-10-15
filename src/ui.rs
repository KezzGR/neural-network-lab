use eframe::egui::{
    self, Button, CentralPanel, Color32, Frame, Panel, RichText, Slider, Stroke, Ui,
};

use crate::neuron::Neuron;

const DATASET: [([f64; 2], f64); 4] = [
    ([0.0, 0.0], 0.0),
    ([1.0, 0.0], 1.0),
    ([0.0, 1.0], 1.0),
    ([1.0, 1.0], 1.0),
];

const CONTROLS_BG: Color32 = Color32::from_rgb(58, 31, 39);
const VISUALIZATION_BG: Color32 = Color32::from_rgb(23, 15, 19);
const TRAIN_STEP_BUTTON: Color32 = Color32::from_rgb(156, 65, 83);

pub fn show(root: &mut Ui) {
    Panel::right("controls")
        .default_size(240.0)
        .resizable(true)
        .frame(Frame::new().fill(CONTROLS_BG).inner_margin(10.0))
        .show(root, |ui| {
            ui.take_available_width();

            ui.heading("Controls");

            let button = Button::new(RichText::new("Train Step").size(16.0));

            ui.add_sized([ui.available_width(), 48.0], button);
        });

    CentralPanel::default()
        .frame(Frame::new().fill(VISUALIZATION_BG).inner_margin(10.0))
        .show(root, |ui| {
            ui.heading("Visualization");
        });
}
