use gdnative::api::*;
use gdnative::prelude::*;

/// The Landmine "class"
#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_builder)]
pub struct Landmine {
    name: String,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl Landmine {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("Landmine builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &Area2D) -> Self {
        godot_print!("Landmine is created!");
        Landmine {
            name: "".to_string(),
        }
    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&mut self, _owner: &Area2D) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        self.name = "Landmine".to_string();
        godot_print!("{} is ready!", self.name);
    }

    // This function will be called in every frame
    #[export]
    unsafe fn _process(&self, _owner: &Area2D, delta: f64) {
        godot_print!("Inside {} _process(), delta is {}", self.name, delta);
    }
}
