use eframe::egui;
use egui::{FontFamily, Vec2};
// use egui::{Color32, FontFamily, FontId, Image, RichText, Vec2};
use egui_extras;

// TODO: Check if someone won the game
// TODO: Handle the case where the game is a draw
// TODO: Handle the end of the game
// DONE: Add a way to restart the game 
// TODO: (later) Add a way to play against the computer

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
    win_option.viewport.inner_size = Some(Vec2::new(500.0, 400.0));

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
    board: GameBoard,
    turn: bool,
    show_warning: bool,
}

impl Default for Myapp {
    fn default() -> Self {
        Self {
            board: GameBoard::default(),
            turn: false,
            show_warning: false,
        }
    }
}

struct GameCase {
    // No player = 0, Player 1 = 1, Player 2 = 2
    // We're gonna go through like this:
    // 0 1 2
    // 3 4 5
    // 6 7 8
    pos: Vec2,
    player: i32,
}

struct GameBoard {
    cases: Vec<GameCase>,
}

impl Default for GameBoard {
    fn default() -> Self {
        let mut cases = Vec::new();
        for i in 0..9 {
            cases.push(GameCase {
                pos: Vec2::new((i % 3) as f32, (i / 3) as f32),
                player: 0,
            });
        }
        Self { cases }
    }
}

fn draw_grid(ui: &mut egui::Ui, board: &mut GameBoard, turn: &mut bool, show_warning: &mut bool) {
    egui::Grid::new("grid")
        .spacing(Vec2::new(10., 10.))
        .show(ui, |ui| {
            for j in 0..3 {
                for i in 0..3 {
                    let case = &mut board.cases[i + j * 3];
                    let bouton = match case.player {
                        0 => egui::ImageButton::new(egui::include_image!("../assets/T.png")),
                        1 => egui::ImageButton::new(egui::include_image!("../assets/X.png")),
                        2 => egui::ImageButton::new(egui::include_image!("../assets/O.png")),
                        _ => egui::ImageButton::new(egui::include_image!("../assets/T.png")),
                    };
                    let bouton2 = ui.add_sized(Vec2::new(100.0, 100.0), bouton);

                    if bouton2.clicked() && case.player == 0 {
                        case.player = 1 + *turn as i32;
                        *turn = !*turn;
                    } else if bouton2.clicked() && case.player != 0 {
                        *show_warning = true;
                    }
                }
                ui.end_row();
            }
        });
}

impl eframe::App for Myapp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(format!(
                    "Jouez au morpion ! \nC'est au tour du joueur {}",
                    (self.turn as i32 + 1)
                ));
                let play_again = ui.add_sized(
                    [40., 40.],
                    egui::ImageButton::new(egui::include_image!("../assets/R.png")),
                );

                if play_again.clicked() {
                    self.board = GameBoard::default();
                    self.turn = false;
                }
            });

            draw_grid(ui, &mut self.board, &mut self.turn, &mut self.show_warning);
            if self.show_warning {
                egui::Window::new("Warning")
                    .collapsible(false)
                    .title_bar(true)
                    .show(ctx, |ui| {
                        ui.label("You should only play on empty cases!");
                        if ui.button("Close").clicked() {
                            self.show_warning = false; // Set this to false to close the warning box
                        }
                    });
            }
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Increment").clicked() {
            //     self.age += 1;
            // }

            // ui.label(format!("Hello {}, aged {}", self.name, self.age));
            // // ui.label(RichText::new("This is a simple egui application").font(FontId::proportional(10.0)));
            // ui.label(
            //     RichText::new("This is a simple egui application")
            //         .font(FontId::new(20.0, FontFamily::Name("roboto".into()))),
            // );
            // ui.label(RichText::new("This is a simple egui application").color(Color32::RED));
            // // add an image
            // ui.add(Image::new(egui::include_image!("../assets/icon.png")).max_width(100.0));

            // if ui.button("Button 1").clicked() {
            //     println!("Button 1 clicked");
            // }
            // egui::Grid::new("some_unique_id").show(ui, |ui| {
            //     ui.label("First row, first column");
            //     ui.label("First row, second column");
            //     ui.image(egui::include_image!("../assets/icon.png"));
            //     ui.end_row();

            //     ui.label("Second row, first column");
            //     ui.label("Second row, second column");
            //     ui.label("Second row, third column");
            //     ui.end_row();

            //     ui.horizontal(|ui| {
            //         ui.label("Same");
            //         ui.label("cell");
            //     });
            //     ui.label("Third row, second column");
            //     ui.end_row();
            // });
        });
    }
}
