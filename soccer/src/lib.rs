// crate는 항상 최상위
mod players {
    // private is default
    pub struct Status {
        pub name: String,
        pub age: i8,
    }
    pub struct Strength {
        pub kick: u32,
        pub pass: u32,
        pub speed: u32,
        pub stamina: u32,
    }
    impl Status {
        pub fn junior (name: &str, age: i8) -> Status {
            Status { name: String::from(name), age: i8::from(age) }
        }
        pub fn senior (name: &str, age: i8) -> Status {
            Status { name: String::from(name), age: i8::from(age) }
        }
    } 
    pub mod attacker {
        pub fn take_shot(name: &str) { 
            println!("{name} took a shot");
        }
    }
    pub mod midfielder {
        pub fn take_pass(name: &str) { 
            println!("{name} took a pass");
        }
    }
    pub mod defender {
        pub fn take_tackle(name: &str) { 
            println!("{name} took a tackle");
        }
    }
}

mod player;

//bring path 
use self::players:: {attacker, midfielder, defender };
use self::players:: {Status, Strength};
use crate::player::init_player;


pub fn play_game() {
    // Absolute path
    init_player();

    crate::players::attacker::take_shot("Son");
    // Relative path
    players::attacker::take_shot("Son");
    // use 
    attacker::take_shot("Son");
    midfielder::take_pass("kim");
    defender::take_tackle("park");

    let player_One_Status = Status::junior("kim", 13);
    let playerTwoStatus = Status::senior("baek", 25);

    println!("{} {}",player_One_Status.name, player_One_Status.age);
}