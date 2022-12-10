#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

struct State {}

// Implementing a trait for a structure
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

fn main() -> BError {

    // bracket-lib (over-)uses build pattern (quite common in Rust)
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;  // ? operator passes errors to the parrent function. Notice function signature!

    // Call to start executing game loop.
    main_loop(context, State{})
}
