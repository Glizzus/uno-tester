use std::{
    fs::File,
    io::BufReader,
    time::Instant,
};

use core::game_master::GameMasterBuilder;
use core::options::Options;
use core::strategy::Strategy;

use serde::Deserialize;
use serde_json::from_reader;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "console\\uno.json")]
    file: String,

    #[arg(short, long, default_value_t = 4)]
    threads: usize,

    #[arg(short, long, default_value_t = 1)]
    games: usize,

    #[arg(short, long, default_value_t = 0)]
    pause: u64,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Deserialize)]
struct PlayerConfig {
    name: String,
    strategy: String,
}

#[derive(Deserialize)]
struct GameConfig {
    pause_duration: u64,
    verbose: bool,
}

#[derive(Deserialize, Default)]
struct Config {
    players: Vec<PlayerConfig>,

    #[serde(default)]
    threads: Option<usize>,

    #[serde(default)]
    games: Option<usize>,

    #[serde(default)]
    game_options: Option<GameConfig>,
}

fn parse_config(filename: &String) -> Config {
    match File::open(filename) {
        Err(_) => Config::default(),
        Ok(f) => {
            let reader = BufReader::new(f);
            from_reader(reader).expect("failed to parse config")
        }
    }
}

fn main() {

    let args = Args::parse();
    let config = parse_config(&args.file);

    let opts = Options::new(config.threads.unwrap_or(args.threads), false, true);
    let mut builder = GameMasterBuilder::new();
    for player in config.players {
        builder = builder.add_player(player.name, Strategy::get(player.strategy));
    }
    let master = builder.with_options(opts).build();

    let start = Instant::now();
    master
        .run(config.games.unwrap_or(args.games))
        .display_results();
    let end = Instant::now();

    let elapsed = end - start;
    println!("Elapsed time: {:?}", elapsed)
}

