use regex::Regex;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let re = Regex::new(r"^#\d+ @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let mut claims = vec![];

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let x = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let y = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let width = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let height = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
        claims.push((x, y, width, height));
    }

    let mut overlaps = HashSet::new();

    for (index, claim) in claims.iter().enumerate() {
        for claim_other in claims.iter().skip(index + 1) {
            for claim_x in claim.0..claim.0 + claim.2 {
                for claim_y in claim.1..claim.1 + claim.3 {
                    if claim_x >= claim_other.0
                        && claim_x < claim_other.0 + claim_other.2
                        && claim_y >= claim_other.1
                        && claim_y < claim_other.1 + claim_other.3 {
                        overlaps.insert((claim_x, claim_y));
                    }
                }
            }
        }
    }

    overlaps.len()
}

#[cfg(test)]
mod test;
