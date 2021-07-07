use gdnative::api::*;
use gdnative::prelude::*;

/// The Bullet "class"
#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_builder)]
pub struct Bullet {
    damage: i64,
    velocity: f32,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl Bullet {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {}

    /// The "constructor" of the class.
    fn new(_owner: &Area2D) -> Self {
        Bullet {
            damage: 1,
            velocity: 64_f32,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Area2D) {}

    #[export]
    fn _physics_process(&self, owner: &Area2D, delta: f64) {
        let current_pos = owner.position();
        let move_vec = Vector2::new(self.velocity as f32, 0_f32)
            .rotated(Angle::radians(owner.rotation() as f32));

        owner.set_position(current_pos + (move_vec * delta as f32));
    }

    #[export]
    fn _on_bullet_collision(&self, _owner: &Area2D, node: Variant) {
        let node = unsafe { node.try_to_object::<Node>().unwrap().assume_safe() };
        if node.has_method("take_damage") {
            unsafe {
                node.call("take_damage", &[Variant::from_i64(self.damage)]);
            }
        }
    }
}
