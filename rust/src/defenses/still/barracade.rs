use gdnative::api::*;
use gdnative::prelude::*;
use gdnative::nativescript::ClassBuilder;
use gdnative::api::KinematicBody2D;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_builder)]
pub struct Barracade {
    health: i64
}

#[methods]
impl Barracade {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
    }

    /// The "constructor" of the class.
    fn new(_owner: &KinematicBody2D) -> Self {
        godot_print!("Barracade is created!");
        Barracade {
            health: 5
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &KinematicBody2D) {
    }

    #[export]
    fn take_damage(&mut self, _owner: &KinematicBody2D, damage: Variant) {
        let current_hp = self.health;

    }
}
