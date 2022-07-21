use rand::Rng;
use crate::{Status, Strength, player_module::Player};

pub fn init_player(name: &str, age: i8, position: &str) -> Player {
    let kick = rand::thread_rng().gen_range(1..=100);
    let pass = rand::thread_rng().gen_range(1..=100);
    let speed = rand::thread_rng().gen_range(1..=100);
    let stamina = rand::thread_rng().gen_range(1..=100);
    
    let player_strength = Strength {
        kick: kick,
        pass: pass,
        speed: speed,
        stamina: stamina,
    };
    let player_status = Status {
        name: String::from(name),
        age: age,
        position: String::from(position),
    };

    // Player 초기화
    let player = Player {
        strength: player_strength,
        status: player_status
    };

    return player;

    // println!("Player info: kick {} pass {} speed {} stamina {}", temp_player.kick, temp_player.pass, temp_player.speed, temp_player.stamina);
}
