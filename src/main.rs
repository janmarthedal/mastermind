use std::fmt::Display;

struct Board {
    color_count: u32,
    hole_count: u32,
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
struct MatchKeys {
    exact_count: u32,
    color_count: u32,
}

impl MatchKeys {
    fn new(exact_count: u32, color_count: u32) -> MatchKeys {
        MatchKeys {
            exact_count,
            color_count,
        }
    }
}

impl Display for MatchKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.exact_count, self.color_count)
    }
}

impl Board {
    fn new(color_count: u32, hole_count: u32) -> Board {
        Board {
            color_count,
            hole_count,
        }
    }
    fn total_pattern_count(&self) -> u32 {
        self.color_count.pow(self.hole_count)
    }
    fn compute_match(&self, pattern: u32, guess: u32) -> MatchKeys {
        let mut pattern = pattern;
        let mut guess = guess;
        let mut exact_count = 0;
        let mut pattern_colors = vec![0; self.color_count as usize];
        let mut guess_colors = vec![0; self.color_count as usize];
        for _ in 0..self.hole_count {
            let pattern_digit = pattern % self.color_count;
            let guess_digit = guess % self.color_count;
            pattern /= self.color_count;
            guess /= self.color_count;
            if pattern_digit == guess_digit {
                exact_count += 1;
            } else {
                pattern_colors[pattern_digit as usize] += 1;
                guess_colors[guess_digit as usize] += 1;
            }
        }
        let color_count = pattern_colors
            .iter()
            .zip(guess_colors.iter())
            .map(|(p, g)| p.min(g))
            .sum();
        MatchKeys {
            exact_count,
            color_count,
        }
    }
    fn pattern_to_string(&self, pattern: u32, color_chars: &[char]) -> String {
        let mut pattern = pattern;
        (0..self.hole_count)
            .map(|_| {
                let digit = pattern % self.color_count;
                pattern /= self.color_count;
                color_chars[digit as usize]
            })
            .collect::<Vec<char>>()
            .iter()
            .rev()
            .collect()
    }
    fn string_to_pattern(&self, s: &str, color_chars: &[char]) -> u32 {
        s.chars()
            .map(|c| color_chars.iter().position(|&x| x == c).unwrap() as u32)
            .fold(0, |acc, x| acc * self.color_count + x)
    }
}

struct Game {
    board: Board,
    matches: Vec<Vec<MatchKeys>>,
    pattern_list: Vec<u32>,
}

fn compute_all_matches(board: &Board) -> Vec<Vec<MatchKeys>> {
    let pattern_count = board.total_pattern_count();
    (0..pattern_count)
        .map(|pattern| {
            (0..pattern_count)
                .map(|guess| board.compute_match(pattern, guess))
                .collect()
        })
        .collect()
}

impl Game {
    fn new(color_count: u32, hole_count: u32) -> Game {
        let board = Board::new(color_count, hole_count);
        let matches = compute_all_matches(&board);
        let pattern_list = (0..board.total_pattern_count()).collect();
        Game {
            board,
            matches,
            pattern_list,
        }
    }

    fn get_guess(&self) -> (u32, u32) {
        // `pattern_list` contains all possible patterns at this point

        // If there is only one pattern left, we are done
        // If there are two patterns left, we can just guess the first one
        if self.pattern_list.len() <= 2 {
            return (self.pattern_list[0], self.pattern_list.len() as u32);
        }

        // The 3-tuple is
        //   row_max: The current minimum of the maximum group length
        //   total_length: The current minimum of the sum of all group lengths
        //   guess: The pattern that minimizes the above two values
        let mut best: Option<(u32, u32, u32)> = None;

        // Consider *any* patterns as a potential guess
        for guess in 0..self.matches.len() as u32 {
            // Look up *all* `MatchKeys` for this guess
            let match_row = &self.matches[guess as usize];
            // Consider only the `MatchKeys` for the remaining patterns
            let mut possibles: Vec<MatchKeys> = self
                .pattern_list
                .iter()
                .map(|&pattern| match_row[pattern as usize])
                .collect();
            // Sort to group the same MatchKeys together
            possibles.sort();

            let mut row_max = 0;
            let mut total_length: u32 = 0; // Sum of all group lengths
            let mut rest = possibles.as_slice();
            while let Some(&key) = rest.first() {
                // The group length is the index of the first element that is not equal to `key`
                let group_length = rest.iter().position(|&k| k != key).unwrap_or(rest.len());
                row_max = row_max.max(group_length as u32);
                total_length += (group_length as u32).pow(2);
                rest = &rest[group_length..];
            }

            if let Some((best_row_max, best_total_length, _)) = best {
                // Considering the `total_length` in case of a tie-breaker
                // reduces the average number of guesses needed
                if row_max < best_row_max
                    || (row_max == best_row_max && total_length < best_total_length)
                {
                    best = Some((row_max, total_length, guess));
                }
            } else {
                best = Some((row_max, total_length, guess));
            }
        }

        (best.unwrap().2, self.pattern_list.len() as u32)
    }

    fn apply_match(&mut self, guess: u32, match_keys: MatchKeys) {
        let guess_row = &self.matches[guess as usize];
        self.pattern_list
            .retain(|&p| guess_row[p as usize] == match_keys);
    }
}

fn read_match_keys() -> MatchKeys {
    println!("Enter exact count and color count separated by comma: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split(',');
    let exact_count = parts.next().unwrap().parse().unwrap();
    let color_count = parts.next().unwrap().parse().unwrap();
    MatchKeys::new(exact_count, color_count)
}

fn play_interactive(hole_count: u32, color_chars: &[char]) {
    let mut game = Game::new(color_chars.len() as u32, hole_count);
    loop {
        let (guess, possibles) = game.get_guess();
        if possibles == 1 {
            println!(
                "Answer: {}",
                game.board.pattern_to_string(guess, color_chars)
            );
            break;
        } else {
            println!(
                "Guess: {} ({} possibles)",
                game.board.pattern_to_string(guess, color_chars),
                possibles
            );
        }
        let keys = read_match_keys();
        if keys == MatchKeys::new(hole_count, 0) {
            println!("Lucky guess!");
            break;
        }
        game.apply_match(guess, keys);
    }
}

fn play_auto(hole_count: u32, color_chars: &[char], code: &str) {
    let mut game = Game::new(color_chars.len() as u32, hole_count);
    let code = game.board.string_to_pattern(code, color_chars);
    loop {
        let (guess, possibles) = game.get_guess();
        if possibles == 1 {
            println!(
                "Answer: {}",
                game.board.pattern_to_string(guess, color_chars)
            );
            break;
        } else {
            println!(
                "Guess: {} ({} possibles)",
                game.board.pattern_to_string(guess, color_chars),
                possibles
            );
        }
        let keys = game.board.compute_match(guess, code);
        println!("Match: {}", keys);
        if keys == MatchKeys::new(hole_count, 0) {
            println!("Lucky guess!");
            break;
        }
        game.apply_match(guess, keys);
    }
}

fn count_guesses(hole_count: u32, color_chars: &[char], code: u32) -> u32 {
    let mut game = Game::new(color_chars.len() as u32, hole_count);
    let mut count = 0;
    loop {
        count += 1;
        let (guess, possibles) = game.get_guess();
        if possibles == 1 {
            break;
        }
        let keys = game.board.compute_match(guess, code);
        if keys == MatchKeys::new(hole_count, 0) {
            break;
        }
        game.apply_match(guess, keys);
    }
    count
}

fn play_all_patterns(hole_count: u32, color_chars: &Vec<char>) {
    let mut total_guesses = 0;
    let mut max_guesses = 0;
    let color_count = color_chars.len() as u32;
    let total_patterns = color_count.pow(hole_count);
    for code in 0..total_patterns {
        let guesses = count_guesses(hole_count, color_chars, code);
        max_guesses = max_guesses.max(guesses);
        total_guesses += guesses;
        println!(
            "Code: {} Guesses: {}",
            Board::new(color_count, hole_count).pattern_to_string(code, &color_chars),
            guesses
        );
    }
    println!("Max number of guesses: {}", max_guesses);
    println!(
        "Average guesses: {}",
        total_guesses as f64 / total_patterns as f64
    );
}

/*
fn show_example_match(board: &Board, color_chars: &[char], p1: &str, p2: &str) {
    println!("{}~{}: {}", p1, p2, board.compute_match(board.string_to_pattern(p1, &color_chars), board.string_to_pattern(p2, &color_chars)));
}

// use std::collections::HashSet;
fn show_all_match_combos(board: &Board, color_chars: &[char]) {
    // let mut match_set: HashSet<MatchKeys> = HashSet::new();
    let pattern_count = board.total_pattern_count();
    for pattern in 0..pattern_count {
        for guess in 0..pattern_count {
            let matchkeys = board.compute_match(pattern, guess);
            // if match_set.contains(&matchkeys) {
            //     continue;
            // }
            // match_set.insert(matchkeys);
            println!(
                "{}: {}~{}",
                matchkeys,
                board.pattern_to_string(pattern, color_chars),
                board.pattern_to_string(guess, color_chars)
            );
        }
    }
}

fn show_example_matches() {
    let color_chars = vec!['1', '2', '3', '4', '5'];
    let board = Board::new(color_chars.len() as u32, 4);
    show_all_match_combos(&board, &color_chars);
    // show_example_match(&board, &color_chars, "1234", "1125");
    // show_example_match(&board, &color_chars, "1212", "1122");
}
*/

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("Usage: mastermind <hole count> <peg chars>");
        std::process::exit(1);
    }
    let hole_count = u32::from_str_radix(&args[0], 10).expect("Invalid hole count");
    let color_chars = args[1].chars().collect::<Vec<char>>();
    if args.len() == 2 {
        play_interactive(hole_count, &color_chars);
    } else if args.len() == 3 && args[2] == "all" {
        play_all_patterns(hole_count, &color_chars);
    } else if args.len() == 4 && args[2] == "guess" {
        let code = &args[3];
        play_auto(hole_count, &color_chars, code);
    } else {
        eprintln!("Illegal usage");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Debug;

    impl Debug for MatchKeys {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{},{}", self.exact_count, self.color_count)
        }
    }

    #[test]
    fn test_total_pattern_count() {
        let board = Board::new(3, 2);
        assert_eq!(board.total_pattern_count(), 9);
    }

    #[test]
    fn test_pattern_to_string() {
        const COLOR_CHARS: &[char] = &['A', 'B', 'C'];
        let board = Board::new(COLOR_CHARS.len() as u32, 2);
        assert_eq!(board.pattern_to_string(0, COLOR_CHARS), "AA");
        assert_eq!(board.pattern_to_string(1, COLOR_CHARS), "AB");
        assert_eq!(board.pattern_to_string(2, COLOR_CHARS), "AC");
        assert_eq!(board.pattern_to_string(3, COLOR_CHARS), "BA");
        assert_eq!(board.pattern_to_string(4, COLOR_CHARS), "BB");
        assert_eq!(board.pattern_to_string(5, COLOR_CHARS), "BC");
        assert_eq!(board.pattern_to_string(6, COLOR_CHARS), "CA");
        assert_eq!(board.pattern_to_string(7, COLOR_CHARS), "CB");
        assert_eq!(board.pattern_to_string(8, COLOR_CHARS), "CC");
    }

    #[test]
    fn test_compute_match_symmetry() {
        let board = Board::new(3, 2);
        let pattern_count = board.total_pattern_count();
        for pattern in 0..pattern_count {
            for guess in 0..pattern_count {
                assert_eq!(
                    board.compute_match(pattern, guess),
                    board.compute_match(guess, pattern)
                );
            }
        }
    }

    #[test]
    fn test_compute_match() {
        let board = Board::new(3, 2);
        assert_eq!(board.compute_match(0, 0), MatchKeys::new(2, 0));
        assert_eq!(board.compute_match(0, 1), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(0, 2), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(0, 3), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(0, 4), MatchKeys::new(0, 0));
        assert_eq!(board.compute_match(0, 5), MatchKeys::new(0, 0));
        assert_eq!(board.compute_match(0, 6), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(0, 7), MatchKeys::new(0, 0));
        assert_eq!(board.compute_match(0, 8), MatchKeys::new(0, 0));
        assert_eq!(board.compute_match(1, 1), MatchKeys::new(2, 0));
        assert_eq!(board.compute_match(1, 2), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(1, 3), MatchKeys::new(0, 2));
        assert_eq!(board.compute_match(1, 4), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(1, 5), MatchKeys::new(0, 1));
        assert_eq!(board.compute_match(1, 6), MatchKeys::new(0, 1));
        assert_eq!(board.compute_match(1, 7), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(1, 8), MatchKeys::new(0, 0));
        assert_eq!(board.compute_match(2, 2), MatchKeys::new(2, 0));
        assert_eq!(board.compute_match(2, 3), MatchKeys::new(0, 1));
        assert_eq!(board.compute_match(2, 4), MatchKeys::new(0, 0));
        assert_eq!(board.compute_match(2, 5), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(2, 6), MatchKeys::new(0, 2));
        assert_eq!(board.compute_match(2, 7), MatchKeys::new(0, 1));
        assert_eq!(board.compute_match(2, 8), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(3, 3), MatchKeys::new(2, 0));
        assert_eq!(board.compute_match(3, 4), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(3, 5), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(3, 6), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(3, 7), MatchKeys::new(0, 1));
        assert_eq!(board.compute_match(3, 8), MatchKeys::new(0, 0));
        assert_eq!(board.compute_match(4, 4), MatchKeys::new(2, 0));
        assert_eq!(board.compute_match(4, 5), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(4, 6), MatchKeys::new(0, 0));
        assert_eq!(board.compute_match(4, 7), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(4, 8), MatchKeys::new(0, 0));
        assert_eq!(board.compute_match(5, 5), MatchKeys::new(2, 0));
        assert_eq!(board.compute_match(5, 6), MatchKeys::new(0, 1));
        assert_eq!(board.compute_match(5, 7), MatchKeys::new(0, 2));
        assert_eq!(board.compute_match(5, 8), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(6, 6), MatchKeys::new(2, 0));
        assert_eq!(board.compute_match(6, 7), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(6, 8), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(7, 7), MatchKeys::new(2, 0));
        assert_eq!(board.compute_match(7, 8), MatchKeys::new(1, 0));
        assert_eq!(board.compute_match(8, 8), MatchKeys::new(2, 0));
    }
}
