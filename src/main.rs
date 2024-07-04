#![windows_subsystem = "windows"]
use eframe::egui;
use egui::{FontFamily, Vec2};
use egui_extras;
use image;
// DONE: Check if someone won the game
// DONE: Handle the case where the game is a draw
// DONE: Handle the end of the game
// DONE: Add a way to restart the game
// DONE: Add a warning when a player tries to play on a non-empty case
// DONE: Add an icon to the window
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

fn load_icon() -> egui::IconData {
	let (icon_rgba, icon_width, icon_height) = {
		let icon = include_bytes!("../assets/icon.png");
		let image = image::load_from_memory(icon)
			.expect("Failed to open icon path")
			.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		(rgba, width, height)
	};
	
	egui::IconData {
		rgba: icon_rgba,
		width: icon_width,
		height: icon_height,
	}
}

// Main function obviously ;)
fn main() {
    // Window options: default (size, resizable, etc)
    
    let mut win_option: eframe::NativeOptions = eframe::NativeOptions {
        ..Default::default()
    };
    // Set the window size
    win_option.viewport.inner_size = Some(Vec2::new(500.0, 400.0));
    win_option.viewport.icon = Some(load_icon().into());

    // Run the app
    let _ = eframe::run_native(
        "Tit Tac Toe",
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
    ended:bool,
}

impl Default for Myapp {
    fn default() -> Self {
        Self {
            board: GameBoard::default(),
            turn: false,
            show_warning: false,
            ended:false,
        }
    }
}

struct GameCase {
    // No player = 0, Player 1 = 1, Player 2 = 2
    // We're gonna go through like this:
    // 0 1 2
    // 3 4 5
    // 6 7 8
    player: i32,
}

struct GameBoard {
    cases: Vec<GameCase>,
}

impl Default for GameBoard {
    fn default() -> Self {
        let mut cases = Vec::new();
        for _i in 0..9 {
            cases.push(GameCase {
                player: 0,
            });
        }
        Self { cases }
    }
}

fn draw_grid(ui: &mut egui::Ui, board: &mut GameBoard, turn: &mut bool, show_warning: &mut bool, ended: &mut bool) {
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

                    if bouton2.clicked() && case.player == 0 && !*ended && !*show_warning{
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

fn check_game_end(board: &mut GameBoard) -> i32 {
    // Check if someone won the game
    // Returns 0 if no one won, 1 if player 1 won, 2 if player 2 won, 3 if it's a draw
    for j in 1..3 {
        // Check the rows
        for i in 0..3 {
            if board.cases[i * 3].player == j
                && board.cases[i * 3 + 1].player == j
                && board.cases[i * 3 + 2].player == j
            {
                return j;
            }
        }
        // Check the columns
        for i in 0..3 {
            if board.cases[i].player == j
                && board.cases[i + 3].player == j
                && board.cases[i + 6].player == j
            {
                return j;
            }
        }
        // Check the diagonals
        if board.cases[0].player == j && board.cases[4].player == j && board.cases[8].player == j {
            return j;
        } else if board.cases[2].player == j
            && board.cases[4].player == j
            && board.cases[6].player == j
        {
            return j;
        }
    }
    // Check if it's a draw
    let mut draw = true;
    for i in 0..9 {
        if board.cases[i].player == 0 {
            draw = false;
        }
    }
    if draw {
        return 3;
    } else {
        return 0;
    }
}

impl eframe::App for Myapp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(format!(
                    "Play Tic-Tac-Toe \nIt's player {}'s turn.",
                    (self.turn as i32 + 1)
                ));
                let play_again = ui.add_sized(
                    [40., 40.],
                    egui::ImageButton::new(egui::include_image!("../assets/R.png")),
                ).on_hover_text("Play again");

                if play_again.clicked() {
                    self.board = GameBoard::default();
                    self.turn = false;
                }
            });
            draw_grid(ui, &mut self.board, &mut self.turn, &mut self.show_warning, &mut self.ended);
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
            let winner = check_game_end(&mut self.board);
            // Perform actions based on the winner
            if winner == 3 {
                self.ended = true;
                egui::Window::new("Draw")
                    .collapsible(false)
                    .title_bar(true)
                    .show(ctx, |ui| {
                        ui.label("It's a draw!");
                        if ui.button("Play again").clicked() {
                            self.board = GameBoard::default();
                            self.turn = false;
                            self.ended = false;
                        }
                    });
            } else if winner != 0 {
                self.ended = true;
                egui::Window::new("Winner")
                    .collapsible(false)
                    .title_bar(true)
                    .show(ctx, |ui| {
                        ui.label(format!("Player {} won!", winner));
                        if ui.button("Play again").clicked() {
                            self.board = GameBoard::default();
                            self.turn = false;
                            self.ended = false;
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
