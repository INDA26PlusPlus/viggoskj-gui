use std::path::{Component, Path, PathBuf};

use chessy;
use crevice::std140::AsStd140;
use ggez::event::{self, EventHandler};
use ggez::graphics::{
    self, Color, DrawParam, Drawable, GraphicsContext, Image, Quad, Shader, ShaderBuilder,
    ShaderParamsBuilder,
};
use ggez::winit::keyboard::NamedKey::ColorF3Blue;
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
            square_size: 64,
            resources: Resources {
                queen: Image::from_path(&_ctx.gfx, "/wq.png").unwrap(),
                pawn: Image::from_path(&_ctx.gfx, "/wp.png").unwrap(),
                rook: Image::from_path(&_ctx.gfx, "/wr.png").unwrap(),
                bishop: Image::from_path(&_ctx.gfx, "/wb.png").unwrap(),
                knight: Image::from_path(&_ctx.gfx, "/wn.png").unwrap(),
                king: Image::from_path(&_ctx.gfx, "/wk.png").unwrap(),

                black_shader: black_shader,
            },
        }
    }
}

impl EventHandler for ChessGui {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        // grid
        for i in 0..64 {
            let (row, col) = index_to_pos(i);

            let mut base = DrawParam::default()
                .scale([self.square_size as f32, self.square_size as f32])
                .dest([
                    (col * self.square_size) as f32,
                    (row * self.square_size) as f32,
                ]);

            if row % 2 != col % 2 {
                base = base.color(Color::BLACK);
            } else {
                base = base.color(Color::RED);
            }

            canvas.draw(&Quad, base);
        }

        canvas.set_shader(&self.resources.black_shader);

        let white_params =
            ShaderParamsBuilder::new(&BlackShaderParams { invert: 0.0f32 }).build(ctx);
        let black_params =
            ShaderParamsBuilder::new(&BlackShaderParams { invert: 1.0f32 }).build(ctx);
        canvas.set_shader_params(&white_params);

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
                let mut base = DrawParam::default()
                    .dest([
                        (col * self.square_size) as f32,
                        (row * self.square_size) as f32,
                    ])
                    .scale([
                        (self.square_size as f32 * 0.8 as f32) / texture.width() as f32,
                        (self.square_size as f32 * 0.8 as f32) / texture.height() as f32,
                    ]);

                if piece.color == chessy::Color::Black {
                    canvas.set_shader_params(&black_params);
                } else {
                    canvas.set_shader_params(&white_params);
                }

                canvas.draw(texture, base);
            }
        }

        canvas.finish(ctx)
    }
}

fn index_to_pos(i: usize) -> (u32, u32) {
    (i as u32 / 8, (i as u32) % 8)
}
