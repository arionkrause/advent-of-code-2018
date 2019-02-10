use std::collections::VecDeque;
use regex::Regex;

pub fn solve(input: &str) -> usize {
    let re = Regex::new(r"^(\d+) players; last marble is worth (\d+) points\r?\n?$").unwrap();
    let captures = re.captures(input).unwrap();
    let amount_players = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let last_marble_worth = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() * 100;
    let mut players_score = vec!(0; amount_players);
    let mut deque = VecDeque::with_capacity(last_marble_worth);
    deque.push_front(0);

    for marble in 1..=last_marble_worth {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let value = deque.pop_back().unwrap();
                deque.push_front(value);
            };

            players_score[marble % amount_players] += marble + deque.pop_front().unwrap();
        } else {
            for _ in 0..2 {
                let value = deque.pop_front().unwrap();
                deque.push_back(value);
            };

            deque.push_front(marble);
        }
    }

    *players_score.iter().max().unwrap()
}
