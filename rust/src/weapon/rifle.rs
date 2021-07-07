use gdnative::api::*;
use gdnative::prelude::*;

/// The Rifle "class"
#[derive(NativeClass)]
#[inherit(Sprite)]
#[register_with(Self::register_builder)]
pub struct Rifle {
    bullet: Ref<PackedScene>,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl Rifle {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {}

    /// The "constructor" of the class.
    fn new(_owner: &Sprite) -> Self {
        let resource_loader = ResourceLoader::godot_singleton();
        let bullet = unsafe {
            resource_loader
                .assume_unique()
                .load("res://scenes/weapon/projectiles/Bullet.tscn", "", false)
                .unwrap()
        }
        .cast::<PackedScene>()
        .unwrap();
        Rifle { bullet }
    }

    #[export]
    fn _ready(&mut self, _owner: &Sprite) {}

    #[export]
    fn on_shoot_gun(&self, owner: &Sprite) {
        let global_pos = owner.global_position();
        let global_rot = owner.global_rotation();
        let bullet_instance =
            unsafe { self.bullet.assume_safe().instance(0).unwrap().assume_safe() };
        unsafe {
            owner
                .get_tree()
                .unwrap()
                .assume_safe()
                .root()
                .unwrap()
                .assume_safe()
                .get_child(0)
                .unwrap()
                .assume_safe()
        }
        .add_child(bullet_instance, false);
        bullet_instance.set("Position", global_pos.to_variant());
        bullet_instance.set("Rotation", global_rot.to_variant());
    }
}
