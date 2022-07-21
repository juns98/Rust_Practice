// crate는 항상 최상위
mod player_module {
    // private is default
    pub struct Status {
        pub name: String,
        pub age: i8,
        pub position: String,
    }
    pub struct Strength {
        pub kick: u32,
        pub pass: u32,
        pub speed: u32,
        pub stamina: u32,
    }
    pub struct Player {
        pub status: Status,
        pub strength: Strength, 
    }
    pub(crate) enum Position {
        FW,
        MF,
        DF,
        GK,
    }
    impl Status {
        pub fn junior (name: &str, age: i8, position: &str) -> Status {
            Status { name: String::from(name), age: i8::from(age), position: String::from(position) }
        }
        pub fn senior (name: &str, age: i8, position: &str) -> Status {
            Status { name: String::from(name), age: i8::from(age), position: String::from(position) }
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
// use core::num;
use std::io::stdin;

use rand::Error;

//bring path 
// use self::player_module:: {attacker, midfielder, defender };
use self::player_module:: {Status, Strength, Position};
use crate::player::init_player;


pub fn play_game() {
    // Absolute path
    let mut players = Vec::new();
    let num_players = 3;
    for _i in 0..num_players {
        let mut name = String::new();
        let mut tempAge = String::new();
        let mut position = String::new();
        println!("Enter name: ");
        let _len_name = stdin().read_line(&mut name);
        println!("Enter age: ");
        let _len_age = stdin().read_line(&mut tempAge);
        let age = tempAge.trim().parse().expect("input not integer");
        println!("Enter Position(only FW, MF, DF): ");
        let _len_pos = stdin().read_line(&mut position);
        
        // 해결하기

        // let check_position =  match position {
        //     Err(Error) => panic!("Not proper position: {}", Error);
        // }

        players.push(init_player(&name, age, &position))
    }

    println!("---Player List---");
    for i in 0..num_players {
        println!("Player Status   | name: {} age: {} position: {} ", &players[i].status.name, &players[i].status.age, &players[i].status.position);
        println!("Player Strength | kick: {} pass: {} speed: {} stamina: {} ", &players[i].strength.kick, &players[i].strength.pass,&players[i].strength.speed,&players[i].strength.stamina);
    }
   
    // crate::player_module::attacker::take_shot("Son");
    // // Relative path
    // player_module::attacker::take_shot("Son");
    // // use 
    // attacker::take_shot("Son");
    // midfielder::take_pass("kim");
    // defender::take_tackle("park");

    // let player_One_Status = Status::junior("kim", 13);
    // let playerTwoStatus = Status::senior("baek", 25);

    // println!("{} {}",player_One_Status.name, player_One_Status.age);
}