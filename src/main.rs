use eframe::egui;
use egui::{Color32, FontFamily, FontId, ImageSource, RichText, Vec2};
use egui_extras;

fn add_fonts(ctx: &egui::Context) {
    // Load fonts
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "roboto".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/Roboto-Regular.ttf")),
    );

    fonts
        .families
        .insert(FontFamily::Name("roboto".into()), vec!["roboto".to_owned()]);

    ctx.set_fonts(fonts);
}

// Main function obviously ;)
fn main() {
    // Window options: default (size, resizable, etc)
    let mut win_option = eframe::NativeOptions::default();
    // Set the window size
    win_option.viewport.inner_size = Some(Vec2::new(400.0, 300.0));

    // Run the app
    let _ = eframe::run_native(
        "My app",
        win_option,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            add_fonts(&cc.egui_ctx);
            // Create the app
            Box::<Myapp>::default()
        }),
    );
}

//
struct Myapp {
    name: String,
    age: u32,
}

impl Default for Myapp {
    fn default() -> Self {
        Self {
            name: "John".to_owned(),
            age: 32,
        }
    }
}

impl eframe::App for Myapp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }

            ui.label(format!("Hello {}, aged {}", self.name, self.age));
            // ui.label(RichText::new("This is a simple egui application").font(FontId::proportional(10.0)));
            ui.label(
                RichText::new("This is a simple egui application")
                    .font(FontId::new(20.0, FontFamily::Name("roboto".into()))),
            );
            ui.label(RichText::new("This is a simple egui application").color(Color32::RED));
            // add an image
            ui.image(ImageSource::from("../assets/icon.png"));

            egui::Grid::new("some_unique_id").show(ui, |ui| {
                if ui.button("Button 1").clicked() {
                    println!("Button 1 clicked");
                }
                ui.label("First row, first column");
                ui.label("First row, second column");
                ui.end_row();

                ui.label("Second row, first column");
                ui.label("Second row, second column");
                ui.label("Second row, third column");
                ui.end_row();

                ui.horizontal(|ui| { ui.label("Same"); ui.label("cell"); });
                ui.label("Third row, second column");
                ui.end_row();
            });
        });
    }
}
