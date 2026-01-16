use eframe::egui;

fn main() -> eframe::Result<()> {
    // 1. Set window options (Title, initial size, etc.)
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // 2. Start the application
    eframe::run_native(
        "My First egui App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

// 3. Define the application state
struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Rust Learner".to_owned(),
            age: 20,
        }
    }
}

// 4. Implement eframe::App trait (UI logic goes here)
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Use a CentralPanel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, egui!");

            // Horizontal layout for name input
            ui.horizontal(|ui| {
                let name_label = ui.label("Enter your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });

            // Slider component
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("years old"));

            // Button and logic
            if ui.button("Add a year").clicked() {
                self.age += 1;
            }

            ui.separator(); // Divider line

            // Display the result
            ui.label(format!("Hello '{}', you are {} years old.", self.name, self.age));

            // Footer info
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.weak("Powered by Rust and egui");
            });
        });
    }
}