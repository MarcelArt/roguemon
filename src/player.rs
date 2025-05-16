
use godot::{classes::{INode2D, Node2D}, meta::AsArg, prelude::*};

use crate::{battle_manager::{BattleManager, TurnState}, mon::Mon};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Player {
    base: Base<Node2D>,

    battle_manager: Gd<BattleManager>,
    mon: Gd<Mon>,
}

#[godot_api]
impl INode2D for Player {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,

            battle_manager: BattleManager::new_alloc(),
            mon: Mon::new_alloc(),
        }
    }

    fn ready(&mut self) {
        // let battle_manager = self.base_mut().get_parent();
        // let battle_manager = battle_manager.unwrap();
        // let bm: Gd<BattleManager> = battle_manager.cast();
        let mon_path = NodePath::from("Player/Mon");
        let mon = self.base().get_node_or_null(&mon_path);

        if matches!(mon, None) {
            godot_print!("Player: Mon node not found");
            return;
        }

        let mon = mon.unwrap();
        self.mon = mon.cast();
    }

    fn process(&mut self, _delta: f64) {
        self.handle_keyboard_input();
    }

    // fn physics_process(&mut self, _delta: f64) {
    //     // This is where you would put your physics logic
    //     godot_print!("Hello from Rust in physics process!");
    // }    
}     

#[godot_api]
impl Player {
    #[func]
    fn handle_keyboard_input(&mut self) {
        let input = Input::singleton();
        if input.is_action_pressed("skill1") {
            let skill = self.mon.bind().get_skill1();
            let damage = skill.bind().get_damage();
            let name = skill.bind().get_name();
            godot_print!("Player: Using skill {} with damage {}", name, damage);
        }
    }
}