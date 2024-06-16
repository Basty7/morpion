use eframe::egui;
use egui::{FontId, RichText, Color32, FontFamily};
use egui_extras;


// Main function obviously ;)
fn main() {
    // Window options: default (size, resizable, etc)
    let win_options = eframe::NativeOptions::default();
    // Run the app
    let _ = eframe::run_native(
        "My app",
        win_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Load fonts
            // Start with the default fonts (we will be adding to them rather than replacing them).
            let mut fonts = egui::FontDefinitions::default();

            // Install my own font (maybe supporting non-latin characters).
            // .ttf and .otf files supported.
            fonts.font_data.insert(
                "roboto".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/Roboto-Regular.ttf")),
            );

            fonts.families.insert(
                FontFamily::Name("roboto".into()),
                vec!["roboto".to_owned(), "sans-serif".to_owned()],
            );


            cc.egui_ctx.set_fonts(fonts);
            // Create the app
            Box::<Myapp>::default()
        }
        ),
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
            ui.label(RichText::new("This is a simple egui application").font(FontId::new(20.0, FontFamily::Name ("roboto".into()))));
            ui.label(RichText::new("This is a simple egui application").color(Color32::RED));
        });
    }
}