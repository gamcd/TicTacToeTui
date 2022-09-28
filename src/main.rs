
fn main() {
    interactive_game();
}

fn interactive_game() {
    let mut oturn = false;

    loop {
        let mut board = Board::new();
        if oturn {
            board.turn = Owner::O;
        }

        println!("\n==========================================\n");
        println!("\nWelcome to Tic Tac Toe");
        println!("To play, give each turn prompt a number between 1 and 9 that looks like this:\n");
        println!("  1 | 2 | 3\n  -----------\n  4 | 5 | 6\n  -----------\n  7 | 8 | 9\n");
        println!("Player {} will go first\n", board.turn.as_char());
        println!("{}", board.to_string());

        while !board.game_over().to_bool() {
            println!("\n\n==========================================");
            let mut good_turn = false;
            let mut turn: usize = 0;

            while !good_turn {
                println!("\nPlayer {}'s turn, input a number:", board.turn.as_char());

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                input.remove(input.len() - 1);
                let num = input.parse::<u8>();

                match num {
                    Ok(n) if 0 < n && n < 10 => {
                        turn = n as usize;
                        good_turn = true;
                    }
                    Ok(_) => println!("Please give a number between 1 and 9"),
                    Err(_) => {
                        println!("Make sure you are putting in a number, please try again")
                    }
                }
            }

            match board.play(turn - 1) {
                TurnResponse::Good(msg) => println!("\n{}\n", msg),
                TurnResponse::Bad(msg) => {
                    println!("\n{}\n", msg);
                    continue;
                }
            };

            println!("\n{}", board.to_string());
            println!("==========================================\n\n");
        }

        println!("\nGame is Over\n");

        if let TurnResponse::Good(msg) = board.game_over()  {
            println!("{}\n", msg)
        }


        let mut good_ans = false;
        while !good_ans {
            println!("play again?");
            println!("y or n");
            let mut buff: String = String::new();
            std::io::stdin()
                .read_line(&mut buff).unwrap();

            match buff.as_ref() {
                "y\n" | "yes\n" => good_ans = true,
                "n\n" | "no\n" => break,
                "enter" | "e" => println!("testing"),
                _ => println!("\ntry again\n"),
            }
        }

        if good_ans {
            oturn = !oturn;
            continue;
        } else {
            println!("Thanks for playing");
            break;
        }
    }
}

struct Board {
    board: [Owner; 9],
    turn: Owner,
    count: u8,
}

#[derive(Clone, Copy)]
enum Owner {
    Empty,
    X,
    O,
}

impl Owner {
    fn as_char(&self) -> &str {
        match self {
            Owner::Empty => " ",
            Owner::O => "O",
            Owner::X => "X",
        }
    }
}

enum TurnResponse {
    Good(String),
    Bad(String),
}

impl TurnResponse {

    fn to_bool(&self) -> bool {
        match self {
            TurnResponse::Good(_) => true,
            TurnResponse::Bad(_) => false,
        }
    }
}

impl Board {
    fn new() -> Board {
        Board {
            board: [Owner::Empty; 9],
            turn: Owner::X,
            count: 0,
        }
    }

    fn game_over(&self) -> TurnResponse {
       

        let win_conditions = [
            [0, 2, 0, 1, 2, 0, 3, 6],
            [4, 4, 3, 4, 5, 1, 4, 7],
            [8, 6, 6, 7, 8, 2, 5, 8],
        ];

        for i in 0..8 {
            let mut countx: u8 = 0;
            let mut counto: u8 = 0;

            for j in win_conditions {
                match self.board[j[i]] {
                    Owner::X => countx += 1,
                    Owner::O => counto += 1,
                    Owner::Empty => continue,
                }
            }

            if countx == 3 {
                return TurnResponse::Good(String::from("Player X has Won"));
            } else if counto == 3 {
                return TurnResponse::Good(String::from("Player O has Won"));
            }
        }

        if self.count == 9 {
            return TurnResponse::Good(String::from("Game is a Tie"));
        }

        TurnResponse::Bad(String::from("No Winners this turn"))
    }

    fn play(&mut self, place: usize) -> TurnResponse {
        match self.board[place] {
            Owner::X | Owner::O => {
                return TurnResponse::Bad(format!(
                    "\nCan't play in spot {}, Occupied by Player {}\n",
                    place + 1,
                    self.board[place as usize].as_char()
                ));
            }

            Owner::Empty => {
                match self.turn {
                    Owner::X => {
                        self.board[place] = Owner::X;
                        self.turn = Owner::O;
                        self.count += 1;
                        TurnResponse::Good(format!("\nPlayed in spot {}\n", place + 1))
                        // RETURN VAL^^
                    }
                    Owner::O => {
                        self.board[place] = Owner::O;
                        self.turn = Owner::X;
                        self.count += 1;
                        TurnResponse::Good(format!("\nPlayed in spot {}\n", place + 1))
                        // RETURN VAL^^
                    }
                    Owner::Empty => {
                        TurnResponse::Bad(String::from(""))
                        // RETURN VAL^^
                    }
                }
            }
        }
    }

    #[allow(clippy::inherent_to_string)]
    fn to_string(&self) -> String {
        format!(
            "\n              {} | {} | {}
             -----------
              {} | {} | {}
             -----------
              {} | {} | {}\n",
            self.board[0].as_char(),
            self.board[1].as_char(),
            self.board[2].as_char(),
            self.board[3].as_char(),
            self.board[4].as_char(),
            self.board[5].as_char(),
            self.board[6].as_char(),
            self.board[7].as_char(),
            self.board[8].as_char(),
        )
    }
}
