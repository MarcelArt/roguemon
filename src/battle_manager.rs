use godot::{classes::{INode2D, Node2D}, prelude::*};

#[derive(GodotConvert, Var, Export, Debug, Copy, Clone)]
#[godot(via = GString)]
pub enum TurnState {
    Player,
    Enemy,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct BattleManager {
    base: Base<Node2D>,

    #[export]
    #[var(get = get_turn_state, set = set_turn_state)]
    turn_state: TurnState,
}

#[godot_api]
impl INode2D for BattleManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            turn_state: TurnState::Player,
        }
    }

    fn ready(&mut self) {
    }

    fn process(&mut self, _delta: f64) {
        
    }

    // fn physics_process(&mut self, _delta: f64) {
    //     // This is where you would put your physics logic
    //     godot_print!("Hello from Rust in physics process!");
    // }    
}     

#[godot_api]
impl BattleManager {
    #[func]
    pub fn get_turn_state(&self) -> TurnState {
        self.turn_state
    }

    #[func]
    pub fn set_turn_state(&mut self, turn_state: TurnState) {
        self.turn_state = turn_state;
    }
}