use bracket_lib::prelude::*;

fn main() -> BError {

struct State {
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print(1,1, "This is a bracket Terminal");
    }
}

    let context = BTermBuilder::simple80x50()
        .with_title("Flappy")
        .build()?; // return Result
    // returns a BError
    main_loop(context, State{})
}
