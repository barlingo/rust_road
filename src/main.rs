use rusty_engine::prelude::*;

struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawm_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_labels: Vec::new(),
            spawm_timer: Timer::from_seconds(1.0, false),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut game = Game::new();
    // setup game here
    game.run(GameState::default());
}
