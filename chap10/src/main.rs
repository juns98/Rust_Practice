trait Player {
    fn get_player_name(&self);
    fn get_player_num(&self);
    fn get_special_trait(&self);
    fn common_function() {
        println!("This is common function");
    }
}
struct Forward {
    number: u8,
    name: String,
    goals: u8,
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

// 각각 struct에 제네릭을 넣고 싶은데
impl Player for Forward {
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
// 어떻게 바꿔야 할 지 모르겠습니다
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
    let player2 = Midfielder {
        number: 5,
        name: String::from("lee"),
        assits: 6
    };

    //goal 에 string이 들어감
    let player3 = Midfielder {
        number: 8,
        name: String::from("lee"),
        assits: String::from("none"),
    };
    
    player1.get_player_num();
    player1.get_player_name();
    player1.get_special_trait();

    player2.get_player_num();
    player2.get_player_name();
    player2.get_special_trait();

    player3.get_player_num();
    player3.get_player_name();
    player3.get_special_trait();

}