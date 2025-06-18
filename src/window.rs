
use eframe::{egui, epaint::Vec2};
use egui::widgets::{Button, Label};


struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello!");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(Label::new(format!(
                "Hello '{}', age {}",
                self.name, self.age
            )));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
        });
    }
}



//
// Import eframe (egui's platform integration library)
use eframe::egui;



// Define the application struct
struct MyApp {
    name: String, // User input state
    age: u32,     // User's age
}

// Default state for the application
impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: String::from("User"),
            age: 30,
        }
    }
}

// Define GUI logic for the app
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Create the GUI layout
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to egui!"); // Add a heading

            // Input field for the name
            ui.label("Enter your name:");
            ui.text_edit_singleline(&mut self.name);

            // Input field for the age
            ui.label("Enter your age:");
            ui.add(egui::Slider::new(&mut self.age, 0..=100).text("years"));

            // Show a button and display a message when clicked
            if ui.button("Submit").clicked() {
                println!("Hello {}, you are {} years old!", self.name, self.age);
            }
        });
    }
}
// adding side panel
egui::SidePanel::left("my_side_panel").show(ctx, |ui| {
    ui.label("This is a side panel!");
});


EnhancedVolcano(read2sel, lab= rownames(read2sel), x= "log2FoldChange", y="pvalue", title= "aso4vscontrol", pointSize = 2.0, labSize = 4.0, col = c('black', 'green', 'blue', 'red'), colAlpha = 1, gridlines.major = FALSE, gridlines.minor = FALSE, legendPosition = "right", legendLabSize = 14, legendIconSize = 3.0)
