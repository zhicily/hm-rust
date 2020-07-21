fn main() {

    let mut game = Hangman {
        word: String::from("penguin"),
        guessed: vec![],
        discovered: vec![],
        lives: 5
    };

    let char_vec: Vec<char> = game.word.to_ascii_lowercase().chars().collect();
    let mut char_vec_check = char_vec.clone();

    loop {
        println!("===================================================");
        println!("HANGMAN: Guess the word!");

        println!("LIVES LEFT: {}    GUESSED LETTERS: {:?}", game.lives, game.guessed);

        display(&game);

        for char in &char_vec {
            if game.discovered.contains(char) {
                print!(" {} ", char);
            } else {
                print!(" _ ");
            }
        } 

        println!("\n\nGuess a letter:");

        let guess = process_guess().unwrap();

        if ALPHABET_LOWER.contains(&guess) {

            match check_guess(&game, guess) {
                
                LetterStatus::NewCorrect => {
                    game.guessed.push(guess);
                    game.discovered.push(guess);

                    char_vec_check.retain(|&c|c != guess);

                    println!("{} is in the hidden word!", guess);

                    if char_vec_check.len() == 0 {
                        println!("\nYOU GUESSED THE WORD: {}!", game.word);
                        break;
                    }
                },

                LetterStatus::AlreadyCorrect =>{
                    println!("You have already correctly guessed {} before.", guess);
                },

                LetterStatus::Guessed => {
                    println!("You have already guessed {} before.", guess);
                },

                LetterStatus::Wrong => {
                    game.guessed.push(guess);

                    println!("{} is not in the hidden word!", guess);   
                    game.lives = game.lives - 1;    
                    
                    if game.lives == 0 && char_vec_check.len() != 0 {
                        println!("YOU LOST!");
                        display(&game);
                        break;
                    }
                }
            }
            
        } else { println!("Invalid input; Please try again."); }
    }
}

static ALPHABET_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

struct Hangman {
    word: String,
    guessed: Vec<char>,
    discovered: Vec<char>,
    lives: i32,
}

enum LetterStatus {
    NewCorrect,
    AlreadyCorrect,
    Guessed,
    Wrong
}

fn process_guess() -> Option<char> {
    let mut input = String::new();
    
    let _ = std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let char = input.chars().nth(0).expect("No byte read");

    Some(char.to_ascii_lowercase())
}

fn check_guess(game: &Hangman, guess: char) -> LetterStatus {
    let guess = guess.to_ascii_lowercase();

    if game.word.to_ascii_lowercase().contains(guess) && !game.discovered.contains(&guess) {

        LetterStatus::NewCorrect

    } else if game.word.to_ascii_lowercase().contains(guess) && game.discovered.contains(&guess) {

        LetterStatus::AlreadyCorrect

    } else if game.guessed.contains(&guess) {

        LetterStatus::Guessed

    } else { LetterStatus::Wrong }
}

fn display(game: &Hangman) {
    match game.lives {
        5 => {
            println!("_____   ");
            println!("|   :   ");
            println!("|   O   ");
            println!("|       ");
            println!("|       ");
            println!("        ");
        },

        4 => {
            println!("_____   ");
            println!("|   :   ");
            println!("|   O   ");
            println!("|  /    ");
            println!("|       ");
            println!("        ");
        },

        3 => {
            println!("_____   ");
            println!("|   :   ");
            println!("|   O   ");
            println!("|  /|   ");
            println!("|       ");
            println!("        ");
        },

        2 => {
            println!("_____   ");
            println!("|   :   ");
            println!("|   O   ");
            println!("|  /|\\ ");
            println!("|       ");
            println!("        ");
        },

        1 => {
            println!("_____   ");
            println!("|   :   ");
            println!("|   O   ");
            println!("|  /|\\ ");
            println!("|  /    ");
            println!("        ");
        },

        _ => {
            println!("_____   ");
            println!("|   :   ");
            println!("|   O   ");
            println!("|  /|\\ ");
            println!("|  / \\ ");
            println!("        ");
        }
    }
}