use exploring_draw_performance_ggez::MainState;
use eyre::Result;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, ContextBuilder};

fn main() -> Result<()> {
    let window_setup = WindowSetup::default().title("Exploring Draw Peformance");
    let window_mode = WindowMode::default().dimensions(1920.0, 1080.0);

    let (mut context, mut event_loop) =
        ContextBuilder::new("bb_exploring_performance", "Brookzerker")
            .window_mode(window_mode)
            .window_setup(window_setup)
            .build()?;
    let mut main_state = MainState::new(&mut context)?;

    event::run(&mut context, &mut event_loop, &mut main_state)?;

    Ok(())
}
