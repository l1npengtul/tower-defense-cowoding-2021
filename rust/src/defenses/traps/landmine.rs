use gdnative::api::*;
use gdnative::prelude::*;

/// The Landmine "class"
#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_builder)]
pub struct Landmine {
    damage: i64,
    exploded: bool,
}

#[methods]
impl Landmine {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
    }

    /// The "constructor" of the class.
    fn new(_owner: &Area2D) -> Self {
        Landmine {
            damage: 5,
            exploded: false
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Area2D) {
        let animation_player = unsafe {
            owner.get_node("AnimationPlayer").unwrap().assume_safe().cast::<AnimationPlayer>().unwrap()
        };
        animation_player.play("IDLE", -1.0, 1.0, false);
    }

    #[export]
    fn _on_land_mine_activated(&mut self, owner: &Area2D, node: Variant) {
        let node = unsafe {
            node.try_to_object().unwrap().cast::<Node>().unwrap().assume_safe()
        };

        if node.is_in_group("enemy") {
            self.explode(owner);
        }
    }

    #[export]
    fn _on_animation_finished(&mut self, owner: &Area2D, animation: Variant) {
        let animation = animation.to_string();
        match animation.as_str() {
            "IDLE" => {}
            "EXPLODE" => {
                self.exploded = true;
                owner.queue_free();
            }
            _ => {}
        }
    }

    #[export]
    fn _on_explode_area_damage(&self, owner: &Area2D, node: Variant) {
        let node = unsafe {
            node.try_to_object().unwrap().assume_safe().cast::<Node>().unwrap()
        };
        let explosion_area = unsafe {
            owner.get_node("ExplosionArea").unwrap().assume_safe().cast::<Area2D>().unwrap()
        };

        let bodies = explosion_area.get_overlapping_bodies();

        for idx in 0..bodies.len() {
            let node = unsafe { bodies.get(idx).try_to_object().unwrap().assume_safe().cast::<Node>().unwrap() };
            if node.has_method("take_damage") {
                unsafe {
                    node.call("take_damage", &[Variant::from_i64(self.damage)])
                }
            }
        }

    }


    #[export]
    fn explode(&mut self, owner: &Area2D) {
        let animation_player = unsafe {
            owner.get_node("AnimationPlayer").unwrap().assume_safe().cast::<AnimationPlayer>().unwrap()
        };
        animation_player.play("EXPLODE", -1.0, 1.0, false);
    }


}
