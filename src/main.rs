use std::path::{Component, Path, PathBuf};
use chessy::{self, PieceType};
use crevice::std140::AsStd140;
use ggez::event::{self, EventHandler};
use ggez::graphics::{
    self, Canvas, Color, DrawParam, Drawable, GraphicsContext, Image, Quad, Shader, ShaderBuilder,
    ShaderParams, ShaderParamsBuilder, Text,
};
use ggez::winit::event::{KeyEvent, MouseButton};
use ggez::winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use ggez::{Context, ContextBuilder, GameResult};

#[derive(AsStd140)]
struct BlackShaderParams {
    invert: f32,
}

fn main() {
    let resource_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res");

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = ChessGui::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct ChessGui {
    chess: chessy::Chess,

    choosing_promotion_piece: bool,
    promotion_origin: (u32, u32),
    promotion_tagret: (u32, u32),

    selected_square: Option<(u32, u32)>,

    square_size: u32,
    resources: Resources,
}

struct Resources {
    queen: Image,
    pawn: Image,
    rook: Image,
    bishop: Image,
    knight: Image,
    king: Image,

    black_shader: Shader,
    white_params: ShaderParams<BlackShaderParams>,
    black_params: ShaderParams<BlackShaderParams>,
}

impl ChessGui {
    pub fn new(_ctx: &mut Context) -> ChessGui {
        let black_shader = ShaderBuilder::new()
            .fragment_path("/black.wgsl")
            .build(&_ctx.gfx)
            .unwrap();

        // Load/create resources such as images here.
        ChessGui {
            chess: chessy::Chess::new(),
            square_size: 64 * 3,
            selected_square: None,
            choosing_promotion_piece: false,
            promotion_origin: (0, 0),
            promotion_tagret: (0, 0),
            resources: Resources {
                queen: Image::from_path(&_ctx.gfx, "/wq.png").unwrap(),
                pawn: Image::from_path(&_ctx.gfx, "/wp.png").unwrap(),
                rook: Image::from_path(&_ctx.gfx, "/wr.png").unwrap(),
                bishop: Image::from_path(&_ctx.gfx, "/wb.png").unwrap(),
                knight: Image::from_path(&_ctx.gfx, "/wn.png").unwrap(),
                king: Image::from_path(&_ctx.gfx, "/wk.png").unwrap(),

                black_shader: black_shader,

                white_params: ShaderParamsBuilder::new(&BlackShaderParams { invert: 0.0f32 })
                    .build(_ctx),
                black_params: ShaderParamsBuilder::new(&BlackShaderParams { invert: 1.0f32 })
                    .build(_ctx),
            },
        }
    }

    fn draw_board_square(&self, canvas: &mut Canvas, row: u32, col: u32, color: Color) {
        let base = DrawParam::default()
            .scale([self.square_size as f32, self.square_size as f32])
            .dest([
                (col * self.square_size) as f32,
                (row * self.square_size) as f32,
            ])
            .color(color);

        canvas.draw(&Quad, base);
    }

    fn draw_board_square_index(&self, canvas: &mut Canvas, index: usize, color: Color) {
        let (row, col) = index_to_pos(index);
        self.draw_board_square(canvas, row, col, color);
    }

    fn draw_board_base(&self, canvas: &mut Canvas) {
        // grid
        for i in 0..64 {
            let (row, col) = index_to_pos(i);

            let color = if row % 2 != col % 2 {
                Color::from_rgb( 30,  40, 50)
            } else {
                Color::from_rgb( 230,  230, 230)
            };

            self.draw_board_square(canvas, row, col, color);
        }

        canvas.set_shader(&self.resources.black_shader);

        canvas.set_shader_params(&self.resources.white_params);

        for i in 0..64 {
            let (row, col) = index_to_pos(i);

            if let Some(piece) = self.chess.board[i] {
                let texture = match piece.piece_type {
                    chessy::PieceType::Bishop => &self.resources.bishop,
                    chessy::PieceType::King => &self.resources.king,
                    chessy::PieceType::Knight => &self.resources.knight,
                    chessy::PieceType::Pawn => &self.resources.pawn,
                    chessy::PieceType::Queen => &self.resources.queen,
                    chessy::PieceType::Rook => &self.resources.rook,
                };
                let base = DrawParam::default()
                    .dest([
                        (col * self.square_size) as f32,
                        (row * self.square_size) as f32,
                    ])
                    .scale([
                        (self.square_size as f32 * 0.8 as f32) / texture.width() as f32,
                        (self.square_size as f32 * 0.8 as f32) / texture.height() as f32,
                    ]);

                if piece.color == chessy::Color::Black {
                    canvas.set_shader_params(&self.resources.black_params);
                } else {
                    canvas.set_shader_params(&self.resources.white_params);
                }

                canvas.draw(texture, base);
            }
        }
    }

    fn bilboard(&self, canvas: &mut Canvas, ctx: &mut Context, text: String) {
        let mut text = Text::new(text);
        text.set_scale(50.0f32);

        let dim = text.measure(ctx).unwrap();
        canvas.draw(&Quad, DrawParam::default().scale(dim).color(Color::BLACK));

        canvas.draw(&text, DrawParam::default());
    }
}

impl EventHandler for ChessGui {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        self.draw_board_base(&mut canvas);

        canvas.set_default_shader();

        match self.chess.game_status() {
            chessy::GameStatus::Checkmate => {
                let color = match self.chess.turn {
                    chessy::Color::Black => "black",
                    chessy::Color::White => "white",
                };

                self.bilboard(&mut canvas, ctx, "checkmate, ".to_string() + color + " lost. Press Escape to restart.");
            }
            chessy::GameStatus::Stalemate => {
                self.bilboard(&mut canvas, ctx, "Stalemate. Press Escape to restart".to_string());
            }
            chessy::GameStatus::Check => {
                self.bilboard(&mut canvas, ctx, "Check".to_string());
            }
            _ => {}
        }

        if let Some(square) = self.selected_square {
            self.draw_board_square(
                &mut canvas,
                square.1,
                square.0,
                Color {
                    a: 0.5,
                    b: 0.0,
                    g: 1.0,
                    r: 1.0,
                },
            );

            // show possible moves

            let legal = self.chess.legal_moves(pos_to_index(square));

            for l in legal {
                self.draw_board_square_index(
                    &mut canvas,
                    l,
                    Color {
                        a: 0.5,
                        b: 0.0,
                        g: 1.0,
                        r: 0.0,
                    },
                );
            }
        }

        if self.choosing_promotion_piece {
            self.bilboard(
                &mut canvas,
                ctx,
                "Promotion, press K: Knight, Q: Queen, B: Bishop, R: Rook".to_string(),
            );
        }

        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        if input.event.physical_key == PhysicalKey::Code(KeyCode::Escape)
        {
            self.choosing_promotion_piece = false;
            self.chess = chessy::Chess::new();
        }

        if self.choosing_promotion_piece {
            let piece_type_opt: Option<PieceType> = match input.event.physical_key {
                PhysicalKey::Code(KeyCode::KeyK) => Some(PieceType::Knight),
                PhysicalKey::Code(KeyCode::KeyQ) => Some(PieceType::Queen),
                PhysicalKey::Code(KeyCode::KeyR) => Some(PieceType::Rook),
                PhysicalKey::Code(KeyCode::KeyB) => Some(PieceType::Bishop),
                _ => None,
            };

            if piece_type_opt.is_some() {
                // promote
                if self
                    .chess
                    .move_piece(
                        pos_to_index(self.promotion_origin),
                        pos_to_index(self.promotion_tagret),
                        piece_type_opt,
                    )
                    .is_ok()
                {
                    self.choosing_promotion_piece = false;
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::winit::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        if _button != MouseButton::Left {
            return Ok(());
        }

        if let Some(from) = self.selected_square {
            let to = (
                (_x / self.square_size as f32) as u32,
                (_y / self.square_size as f32) as u32,
            );

            // is promotion move
            if self.chess.board[pos_to_index(from)]
                .is_some_and(|piece| piece.piece_type == PieceType::Pawn)
                && (to.1 == 0 || to.1 == 7)
            {
                self.choosing_promotion_piece = true;
                self.promotion_origin = from;
                self.promotion_tagret = to;
                return Ok(());
            } else if self
                .chess
                .move_piece(pos_to_index(from), pos_to_index(to), None)
                .is_ok()
            {
                return Ok(());
            }
        }

        self.selected_square = Some((
            (_x / self.square_size as f32) as u32,
            (_y / self.square_size as f32) as u32,
        ));

        return Ok(());
    }
}

fn index_to_pos(i: usize) -> (u32, u32) {
    (i as u32 / 8, (i as u32) % 8)
}

fn pos_to_index(pos: (u32, u32)) -> usize {
    (pos.1 * 8 + pos.0) as usize
}
