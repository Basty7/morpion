#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)] // hide console window on Windows in release

use egui_extras;

use eframe;
use eframe::egui;
use egui::{FontFamily, FontId, RichText, Vec2};

#[cfg(target_arch = "wasm32")]
use eframe::web_sys;

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
    fonts.font_data.insert(
        "GaMaamli".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/GaMaamli-Regular.ttf")),
    );

    fonts
        .families
        .insert(FontFamily::Name("roboto".into()), vec!["roboto".to_owned()]);
    fonts.families.insert(
        FontFamily::Name("GaMaamli".into()),
        vec!["GaMaamli".to_owned()],
    );

    ctx.set_fonts(fonts);
}


// Main function obviously ;)
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Window options: default (size, resizable, etc)

    let mut win_option: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(
            eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                .expect("Failed to load icon"),
        ),
        ..Default::default()
    };
    // Set the window size
    win_option.viewport.inner_size = Some(Vec2::new(500.0, 400.0));

    // Run the app
    let _ = eframe::run_native(
        "Tit Tac Toe",
        win_option,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            add_fonts(&cc.egui_ctx);
            // Create the app
            Ok(Box::<Myapp>::default())
        }),
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| {
                    egui_extras::install_image_loaders(&cc.egui_ctx);
                    add_fonts(&cc.egui_ctx);
                    Ok(Box::<Myapp>::default())
                }),
            )
            .await;

        // Remove the loading text and spinner
        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

//
struct Myapp {
    board: GameBoard,
    turn: bool,
    show_warning: bool,
    ended: bool,
}

impl Default for Myapp {
    fn default() -> Self {
        Self {
            board: GameBoard::default(),
            turn: false,
            show_warning: false,
            ended: false,
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
            cases.push(GameCase { player: 0 });
        }
        Self { cases }
    }
}

fn draw_grid(
    ui: &mut egui::Ui,
    board: &mut GameBoard,
    turn: &mut bool,
    show_warning: &mut bool,
    ended: &mut bool,
    dark: bool,
) {
    egui::Grid::new("grid")
        .spacing(Vec2::new(10., 10.))
        .show(ui, |ui| {
            for j in 0..3 {
                for i in 0..3 {
                    let case = &mut board.cases[i + j * 3];
                    let bouton = match (case.player, dark) {
                        (0, true) => {
                            egui::ImageButton::new(egui::include_image!("../assets/T.png"))
                        }
                        (0, false) => {
                            egui::ImageButton::new(egui::include_image!("../assets/T.png"))
                        }
                        (1, true) => {
                            egui::ImageButton::new(egui::include_image!("../assets/X.png"))
                        }
                        (1, false) => {
                            egui::ImageButton::new(egui::include_image!("../assets/X-B.png"))
                        }
                        (2, true) => {
                            egui::ImageButton::new(egui::include_image!("../assets/O.png"))
                        }
                        (2, false) => {
                            egui::ImageButton::new(egui::include_image!("../assets/O-B.png"))
                        }
                        _ => egui::ImageButton::new(egui::include_image!("../assets/T.png")),
                    };
                    let bouton2 = ui.add_sized(Vec2::new(100.0, 100.0), bouton);

                    if bouton2.clicked() && case.player == 0 && !*ended && !*show_warning {
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
        3
    } else {
        0
    }
}

impl eframe::App for Myapp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    RichText::new(format!(
                        "Play Tic-Tac-Toe \nIt's player {}'s turn.",
                        self.turn as i32 + 1
                    ))
                        .font(FontId::new(20.0, FontFamily::Name("GaMaamli".into()))),
                );

                let play_again = ui
                    .add_sized(
                        [40., 40.],
                        match ctx.style().visuals.dark_mode {
                            true => {
                                egui::ImageButton::new(egui::include_image!("../assets/R.png"))
                            }
                            false => {
                                egui::ImageButton::new(egui::include_image!("../assets/R-B.png"))
                            }
                        },
                    )
                    .on_hover_text("Play again");

                if play_again.clicked() {
                    self.board = GameBoard::default();
                    self.turn = false;
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
            draw_grid(
                ui,
                &mut self.board,
                &mut self.turn,
                &mut self.show_warning,
                &mut self.ended,
                ctx.style().visuals.dark_mode,
            );
            if self.show_warning {
                egui::Window::new("Warning")
                    .collapsible(false)
                    .title_bar(true)
                    .show(ctx, |ui| {
                        ui.label(
                            RichText::new("You should only play on empty cases!")
                                .font(FontId::new(20.0, FontFamily::Name("GaMaamli".into()))),
                        );
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
                        ui.label(
                            RichText::new("It's a draw!")
                                .font(FontId::new(20.0, FontFamily::Name("GaMaamli".into()))),
                        );
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
                        ui.label(
                            RichText::new(format!("Player {} won!", winner))
                                .font(FontId::new(20.0, FontFamily::Name("GaMaamli".into()))),
                        );
                        if ui.button("Play again !").clicked() {
                            self.board = GameBoard::default();
                            self.turn = false;
                            self.ended = false;
                        }
                    });
            }
        });
    }
}
