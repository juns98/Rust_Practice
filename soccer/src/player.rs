use rand::Rng;
use crate::Strength;

pub fn init_player() {
    let kick = rand::thread_rng().gen_range(1..=100);
    let pass = rand::thread_rng().gen_range(1..=100);
    let speed = rand::thread_rng().gen_range(1..=100);
    let stamina = rand::thread_rng().gen_range(1..=100);

    let temp_player = Strength {
        kick: kick,
        pass: pass,
        speed: speed,
        stamina: stamina,
    };
    println!("Player info: kick {} pass {} speed {} stamina {}", temp_player.kick, temp_player.pass, temp_player.speed, temp_player.stamina);
}
