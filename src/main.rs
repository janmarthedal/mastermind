struct Board {
    color_count: u32,
    hole_count: u32,
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
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
        if self.pattern_list.len() == 1 {
            return (self.pattern_list[0], 1);
        }

        let mut best: Option<(u32, u32, u32)> = None;

        for &guess in &self.pattern_list {
            let match_row = &self.matches[guess as usize];
            let mut possibles: Vec<MatchKeys> = self
                .pattern_list
                .iter()
                .map(|&pattern| match_row[pattern as usize])
                .collect();
            // Sort to group the same MatchKeys together
            possibles.sort();

            let mut row_max = 0;
            let mut total_length: u32 = 0; // Sum of all group lengths
            let mut tail = possibles.as_slice();
            while !tail.is_empty() {
                let key = tail[0];
                let group_length = tail.iter().position(|&k| k != key).unwrap_or(tail.len());
                row_max = row_max.max(group_length as u32);
                total_length += (group_length as u32).pow(2);
                tail = &tail[group_length..];
            }

            if let Some((best_row_max, best_total_length, _)) = best {
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
        self.pattern_list = self
            .pattern_list
            .iter()
            .filter(|&p| guess_row[*p as usize] == match_keys)
            .copied()
            .collect();
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

fn main() {
    let color_chars = vec!['P', 'R', 'G', 'Y', 'B'];
    let hole_count = 4;
    let mut game = Game::new(color_chars.len() as u32, hole_count);
    loop {
        let (guess, possibles) = game.get_guess();
        if possibles == 1 {
            println!(
                "Answer: {}",
                game.board.pattern_to_string(guess, &color_chars)
            );
            break;
        } else {
            println!(
                "Guess: {} ({} possibles)",
                game.board.pattern_to_string(guess, &color_chars),
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
