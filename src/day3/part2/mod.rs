use regex::Regex;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let mut claims = vec![];

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let x = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let y = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let width = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let height = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();
        claims.push((id, x, y, width, height));
    }

    let mut overlaps = HashSet::new();
    let mut overlapping_claims_ids = HashSet::new();

    for (index, claim) in claims.iter().enumerate() {
        for claim_other in claims.iter().skip(index + 1) {
            for claim_x in claim.1..claim.1 + claim.3 {
                for claim_y in claim.2..claim.2 + claim.4 {
                    if claim_x >= claim_other.1
                        && claim_x < claim_other.1 + claim_other.3
                        && claim_y >= claim_other.2
                        && claim_y < claim_other.2 + claim_other.4 {
                        overlaps.insert((claim_x, claim_y));
                        overlapping_claims_ids.insert(claim.0);
                        overlapping_claims_ids.insert(claim_other.0);
                    }
                }
            }
        }
    }

    for claim in claims {
        if !overlapping_claims_ids.contains(&claim.0) {
            return claim.0
        }
    }

    panic!("Should not have reached here.");
}

#[cfg(test)]
mod test;
