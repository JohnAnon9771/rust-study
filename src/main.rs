mod guessing_game;
mod introdution;

fn main() {
    // variables
    let _name = "João"; // immutable variable
    let mut _age = 30; // mutable variable
    const _PI: f32 = 3.14; // constant variable

    // scalar types
    // integer types, float types, boolean types, char types

    // compound types
    // tuple types, array types

    introdution::print("Hello, Mom!");
    introdution::shadowing();
    guessing_game::exec();
}
