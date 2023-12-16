use bracket_lib::prelude::*;

mod game;

pub use game::game_state::State;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const FRAME_DURATION: f32 = 20.0;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon 🔥️")
        //.with_fullscreen(true)
        .build()?;

    main_loop(context, State::new())
}
