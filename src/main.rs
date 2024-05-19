use eframe::egui;
use egui_extras;

fn main() {
    println!("Hello, world!");
    eframe::run_native(
        "My app",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()},
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                
                Box::<Myapp>::default() }
            ),
        );
        
}

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
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

        });
    }
}