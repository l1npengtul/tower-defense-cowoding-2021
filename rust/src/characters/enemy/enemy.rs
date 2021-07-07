use gdnative::api::*;
use gdnative::prelude::*;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum States {
    Idle,
    MoveToTarget,
    Shooting,
    Die,
    Dead,
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
#[register_with(Self::register_builder)]
pub struct Enemy {
    current_state: States,
}

#[methods]
impl Enemy {
    fn register_builder(_builder: &ClassBuilder<Self>) {}

    fn new(_owner: &KinematicBody2D) -> Self {
        Enemy {
            current_state: States::Idle,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &KinematicBody2D) {}

    #[export]
    fn _on_visual_area_body_entered(&self, _owner: &KinematicBody2D, _node: Variant) {}
}
