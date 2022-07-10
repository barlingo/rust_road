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
    let player = game.add_sprite("player", SpritePreset::RacingCarBlack);
    player.translation = Vec2::new(-200.0, 100.0);
    player.collision = true;
    let car1 = game.add_sprite("car1", SpritePreset::RacingCarRed);
    car1.translation = Vec2::new(400.0, 100.0);
    car1.collision = true;
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
        println!("{:#?}", event);
    }
    // game_state.current_score += 1;
    let player = engine.sprites.get_mut("player").unwrap();
    player.translation.x += 100.0 * engine.delta_f32;
    // println!("Current score: {}", game_state.current_score);
}
