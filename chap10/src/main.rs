trait Player {
    fn get_player_name(&self);
    fn get_player_num(&self);
    fn common_function() {
        println!("This is common function");
    }
    fn get_special_trait(&self);
}

struct Forward<T, U> {
    number: u8,
    name: T,
    goals: U,
}
struct Midfielder<T,U> {
    number: u8,
    name: T,
    assits: U,
}
struct Defender<T,U> {
    number: u8,
    name: T,
    tackles: U,
}

// 어떻게 바꿔야 할 지 모르겠습니다...
impl<T,U> Player for Forward<T,U> {
    fn get_player_name(&self) {
        println!("name: {}", self.name) ;
    }
    fn get_player_num(&self) {
        println!("number: {}", self.number);
    }
    fn get_special_trait(&self) {
        println!("goal: {}", self.goals);
    }
}
impl<T,U> Player for Midfielder<T,U> {
    fn get_player_name(&self) {
        println!("name: {}", self.name);
    }
    fn get_player_num(&self) {
        println!("number: {}", self.number);
    }
    fn get_special_trait(&self) {
        println!("goal: {}", self.assits);
    }
}
impl<T,U> Player for Defender<T,U> {
    fn get_player_name(&self) {
        println!("name: {}", self.name);
    }
    fn get_player_num(&self) {
        println!("number: {}", self.number);
    }
    fn get_special_trait(&self) {
        println!("goal: {}", self.tackles);
    }
}

fn main() {
    //goal에 int가 들어감
    let player1 = Forward {
        number: 10,
        name: String::from("kim"),
        goals: 18,
    };

    //goal 에 string이 들어감
    let player2 = Forward {
        number: 10,
        name: String::from("kim"),
        goals: String::from("none"),
    };
    
    player1.get_player_num();
    player1.get_player_name();
    player1.get_special_trait();

    player2.get_player_num();
    player2.get_player_name();
    player2.get_special_trait();

}