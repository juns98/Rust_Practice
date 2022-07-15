// crate는 항상 최상위
mod players {
    // private is default
    pub struct Status {
        pub name: String,
        pub age: i8,
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

//bring path 
use self::players::attacker;
use self::players::Status;

pub fn play_game() {
    // Absolute path
    crate::players::attacker::take_shot("Son");
    // Relative path
    players::attacker::take_shot("Son");
    // use 
    attacker::take_shot("Son");

    let playerOneStatus = Status::junior("kim", 13);
    let playerTwoStatus = Status::senior("baek", 25);
}