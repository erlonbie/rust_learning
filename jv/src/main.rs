struct ForkGame {
    players: Vec<Player>,
}

impl ForkGame {
    fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn start(&self) {
        //implement game logic
        for player in &self.players {
            println!("Player: {}", player.name);
        }
        let input = choose_fork();
        println!("You chose fork: {input}");

        // compare to actual word
        
    }
}

fn choose_fork() -> i32 {
    // implement logic
    println!("Choose a fork");
    //read input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // convert input to integer
    let input: i32 = input.trim().parse().unwrap();
    // return input
    input

}

struct Player {
    name: String,
}
impl Player {
    fn new(arg: &str) -> Player {
        Player {
            name: arg.to_string(),
        }
    }
}

fn main() {
    // create fork game
    let mut game = ForkGame::new();
    // create a new player
    let player = Player::new("Player 1");
    // add player to ForkGame
    game.add_player(player);
    // start ForkGame
    game.start();
}
