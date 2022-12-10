#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        //TODO: Finish stub
        self.mode = GameMode::End;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(10, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,  // instructs bracket-lib that user is ready to terminate the app
                _ => {},
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You Died!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(10, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }

    fn restart(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Playing;
    }
}

// Implementing a trait for a structure
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {

    // bracket-lib (over-)uses build pattern (quite common in Rust)
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;  // ? operator passes errors to the parrent function. Notice function signature!

    // Call to start executing game loop.
    main_loop(context, State::new())
}
