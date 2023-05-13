use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::iter;
use std::sync::{Arc, Mutex};
use std::{collections::VecDeque, thread};

use crate::options::Options;
use crate::{game::Game, player::Player, rules::Rules, strategy::Strategy};

#[derive(Clone)]
struct GameDaemon {
    players: VecDeque<Player>,
    rules: Rules,
    verbose: bool,
}

impl GameDaemon {
    fn run(&self) -> Player {
        Game::new(self.players.clone(), self.rules, self.verbose).play_until_winner()
    }
}

pub struct ScoreBoard {
    scores: Arc<Mutex<Vec<u32>>>,
}

impl ScoreBoard {
    pub fn display_results(&self) {
        let scores = self.scores.clone();
        let scores = scores.lock().unwrap();
        for (i, score) in scores.iter().enumerate() {
            println!("Player {} won {} times", i, score)
        }
    }
}

/// Runs game simulations.
/// Should be constructed using `GameMasterBuilder`
pub struct GameMaster {
    players: Vec<Player>,
    rules: Rules,
    options: Options,
}

impl GameMaster {
    fn new(players: Vec<Player>, rules: Rules, options: Options) -> Self {
        Self {
            players,
            rules,
            options,
        }
    }

    fn spawn_daemon(&self) -> GameDaemon {
        let players = VecDeque::from(self.players.clone());
        GameDaemon {
            players: VecDeque::from(players),
            rules: self.rules,
            verbose: self.options.verbose,
        }
    }

    /// Plays a game `n` times
    ///
    /// # Arguments
    ///
    /// * `n` - A usize that indicates the amount of games to run
    pub fn run(self, n: usize) -> ScoreBoard {
        let mut handles = Vec::with_capacity(self.options.num_threads);
        let num_threads = self.options.num_threads;

        let mut counters: Vec<u32> = iter::repeat(0).take(self.players.len()).collect();
        let shared_counters = Arc::new(Mutex::new(counters));

        for i in 0..num_threads {
            let daemon = self.spawn_daemon();
            let shared_counters = shared_counters.clone();

            let handle = thread::spawn(move || {
                let start = i * n / num_threads;
                let end = (i + 1) * n / num_threads;
                for _ in start..end {
                    let winner = daemon.run();

                    let mut counters = shared_counters.lock().unwrap();
                    counters[winner.id as usize] += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        ScoreBoard {
            scores: shared_counters,
        }
    }
}

/// A builder for the `GameMaster`
pub struct GameMasterBuilder {
    players: Vec<Player>,
    rules: Rules,
    options: Options,

    last_id: u32,
}

impl GameMasterBuilder {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            rules: Rules::default(),
            options: Options::default(),
            last_id: 0,
        }
    }

    fn new_player(&mut self, name: String, strategy: Strategy) -> Player {
        let player = Player::new(self.last_id, name, strategy);
        self.last_id += 1;
        player
    }

    /// Add a player with the given name and strategy
    ///
    /// # Arguments
    ///
    /// * `name` - A `String` that represents the name of the player
    /// * `strategy` A `Strategy` struct that represents how the player plays
    pub fn add_player(mut self, name: String, strategy: Strategy) -> Self {
        let player = self.new_player(name, strategy);
        self.players.push(player);
        self
    }

    /// Set the options to be used
    ///
    /// # Arguments
    ///
    /// * `options` - An `Options` struct
    pub fn with_options(mut self, options: Options) -> Self {
        self.options = options;
        self
    }

    /// Sets the rules to be used during the games of Uno
    ///
    /// # Arguments
    ///
    /// * `rules` - A `Rules` struct
    pub fn with_rules(mut self, rules: Rules) -> Self {
        self.rules = rules;
        self
    }

    pub fn build(self) -> GameMaster {
        GameMaster::new(self.players, self.rules, self.options)
    }
}