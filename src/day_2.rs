use crate::utils::read_file;

// Puzzle is true for first and false for second
pub fn eval_strat(puzzle: bool) -> i32 {
    let input = read_file("input/2.input");
    let lines: Vec<&str> = input.lines().collect();
    let mut you: Vec<&str> = Vec::new();
    let mut them: Vec<&str> = Vec::new();

    for line in lines.iter() {
        // Push the first char
        them.push(&line[0..1]);
        you.push(&line[2..3]);
    }

    assert_eq!(you.len(), them.len()); // As they say, fail fast if something's wrong

    let mut total_points = 0;

    if puzzle {
        for i in 0..you.len() {
            let you_char = you[i];
            let them_char = them[i];

            total_points += winner(them_char, you_char)
        }
    } else {
        for i in 0..you.len() {
            let you_char = play(them[i], you[i]);
            let them_char = them[i];

            total_points += winner(them_char, you_char)
        }
    }

    total_points
}

fn winner(them: &str, you: &str) -> i32 {
    let chars: (&str, &str) = (them, you);
    // A is Rock
    // B is Paper
    // C is Scissors
    // X is Rock
    // Y is Paper
    // Z is Scissors
    // Return 6 if win, 3 if tie, 0 if loss plus 1 for rock, 2 for paper, and 3 for scissors
    match chars {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,
        _ => 0,
    }
}

fn play(play: &str, outcome: &str) -> &'static str {
    let chars: (&str, &str) = (play, outcome);
    // The first char is what they played and the second char is the outcome you need to achieve
    // X means you need to lose
    // Y means you need to tie
    // Z means you need to win
    // Return the move you need to make

    match chars {
        ("A", "X") => "Z",
        ("A", "Y") => "X",
        ("A", "Z") => "Y",
        ("B", "X") => "X",
        ("B", "Y") => "Y",
        ("B", "Z") => "Z",
        ("C", "X") => "Y",
        ("C", "Y") => "Z",
        ("C", "Z") => "X",
        _ => "X",
    }
}
