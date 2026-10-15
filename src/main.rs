mod neuron;
mod ui;

fn main() -> eframe::Result {
    eframe::run_ui_native(
        "Neural Network Lab",
        eframe::NativeOptions::default(),
        move |root, _frame| {
            ui::show(root);
        },
    )
}
