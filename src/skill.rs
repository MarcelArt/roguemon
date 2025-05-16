use godot::{classes::{INode2D, Node2D}, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Skill {
    base: Base<Node2D>,

    // Change or add your own properties here
    #[export]
    damage: f64,
    #[export]
    name: GString,
}

#[godot_api]
impl INode2D for Skill {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            damage: 10.0,
            name: GString::from("Fireball"),  
        }
    }

    fn ready(&mut self) {
    }

    fn process(&mut self, _delta: f64) {
    } 
}     
        