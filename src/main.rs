use std::fmt::Debug;

struct Board {
    color_count: u32,
    hole_count: u32,
}

#[derive(PartialEq)]
struct MatchKeys {
    exact_count: u32,
    color_count: u32,
}

impl Debug for MatchKeys {
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
        // TODO - Encode in u32 instead of Vec
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

fn main() {
    const COLOR_CHARS: &[char] = &['A', 'B', 'C'];
    let board = Board::new(COLOR_CHARS.len() as u32, 2);
    let pattern_count = board.total_pattern_count();
    println!("Total pattern count: {}", pattern_count);
    for pattern in 0..pattern_count {
        let pattern_str = board.pattern_to_string(pattern, COLOR_CHARS);
        println!("Pattern: {}", pattern_str);
    }
    println!(
        "Match of {} and {}: {:?}",
        board.pattern_to_string(1, COLOR_CHARS),
        board.pattern_to_string(3, COLOR_CHARS),
        board.compute_match(1, 3)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    impl MatchKeys {
        fn new(exact_count: u32, color_count: u32) -> MatchKeys {
            MatchKeys {
                exact_count,
                color_count,
            }
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
