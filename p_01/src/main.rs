use std::path;

use td_prototype_01::{GameConfig, GameState};
use ggez::{
    conf::{FullscreenType, WindowMode, WindowSetup},
    ContextBuilder, event,
};

fn main() {
    let resource_dir = path::PathBuf::from("./resources");

    let game_config = match GameConfig::load_from_file("game_config.json") {
        Err(error) => {
            eprintln!("Failed to load game configuration: {:?}", error);
            std::process::exit(1);
        }
        Ok(game_config) => game_config,
    };
    let window_setup = WindowSetup::default()
        .title(game_config.game_title.as_str())
        .vsync(game_config.vsync)
        //TODO: use the gameconfig samples for this. This might require some weird conversions, which could probably be done within the game_config file upon instatiation.
        .samples(ggez::conf::NumSamples::Four);

    let full_screen = WindowMode::default().fullscreen_type(FullscreenType::Desktop);

    let (mut ctx, event_loop) = ContextBuilder::new(game_config.game_title.as_str(), "Joshua Wise")
        .add_resource_path(resource_dir)
        .window_setup(window_setup)
        .window_mode(full_screen)
        .build()
        .expect("aieeeeee, could not create a game context!!!!!");
    
    let my_game = GameState::new(&mut ctx, game_config);

    event::run(ctx, event_loop, my_game);
}
