use godot::{classes::{INode2D, Node2D}, prelude::*};

use crate::skill::Skill;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Mon {
    base: Base<Node2D>,

    // Change or add your own properties here
    #[export]
    name: GString, 
    #[export]
    hp: f64,
    #[export]
    speed: f64,
    #[export]
    atk: f64,
    #[export]
    skill1: Gd<Skill>,
}

#[godot_api]
impl INode2D for Mon {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            name: GString::from("Monster"),
            hp: 100.0,
            speed: 100.0,
            atk: 100.0,
            skill1: Skill::new_alloc(),
        }
    }

    fn ready(&mut self) {
    }

    fn process(&mut self, _delta: f64) {
    } 
}     
        