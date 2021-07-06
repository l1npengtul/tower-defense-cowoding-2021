
mod characters;
mod defenses;
mod game;
mod weapon;

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<characters::enemy::enemy::Enemy>();
    handle.add_class::<characters::player::Player>();
    handle.add_class::<defenses::still::barracade::Barracade>();
    handle.add_class::<defenses::traps::landmine::Landmine>();
    handle.add_class::<game::Game>();
    handle.add_class::<weapon::rifle::Rifle>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
