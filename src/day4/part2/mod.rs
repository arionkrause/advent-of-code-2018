use chrono::*;
use regex::Regex;
use std::collections::HashMap;
use crate::day4::part2::EventType::*;
use chrono::NaiveDateTime;

enum EventType {
    BeginsShift,
    FallsAsleep,
    WakesUp,
}

struct Guard {
    minutes_asleep: HashMap<u8, usize>,
}

impl Guard {
    fn log_minutes(&mut self, start: u8, end: u8) {
        for minute in start..=end {
            self.minutes_asleep.entry(minute)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
}

struct Record {
    date_time: NaiveDateTime,
    id: Option<usize>,
    event_type: EventType,
}

pub fn solve(input: &str) -> usize {
    let re = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})] (.+)$").unwrap();
    let mut records = vec![];
    let mut guards = HashMap::new();

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let date_time_raw = captures.get(1).unwrap().as_str();
        let date_time = NaiveDateTime::parse_from_str(date_time_raw, "%Y-%m-%d %H:%M").unwrap();
        let mut id = None;
        let event_type_raw = captures.get(2).unwrap().as_str();

        let event_type = match event_type_raw {
            "falls asleep" => FallsAsleep,
            "wakes up" => WakesUp,
            _ => {
                let re_id = Regex::new(r"^\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}] Guard #(\d+) begins shift$").unwrap();
                let captures = re_id.captures(line).unwrap();
                id = Some(captures.get(1).unwrap().as_str().parse::<usize>().unwrap());

                if !guards.contains_key(&id.unwrap()) {
                    guards.insert(id.unwrap(), Guard {minutes_asleep: HashMap::new()});
                }

                BeginsShift
            }
        };

        records.push(Record {date_time, id, event_type});
    }

    records.sort_by_key(|k| k.date_time.timestamp());
    let mut minutes_asleep: HashMap<usize, usize> = HashMap::new();
    let mut current_id: Option<usize> = None;
    let mut last_falls_asleep_minute: Option<u8> = None;

    for record in records {
        match record.event_type {
            BeginsShift => current_id = record.id,
            FallsAsleep => last_falls_asleep_minute = Some(record.date_time.time().minute() as u8),
            WakesUp => {
                let last_wakes_up_minute = record.date_time.time().minute() as u8;
                let minutes_sleeped = last_wakes_up_minute - last_falls_asleep_minute.unwrap();

                minutes_asleep.entry(current_id.unwrap())
                    .and_modify(|e| { *e += minutes_sleeped as usize })
                    .or_insert(minutes_sleeped as usize);

                let guard = guards.get_mut(&current_id.unwrap()).unwrap();
                guard.log_minutes(last_falls_asleep_minute.unwrap(), last_wakes_up_minute - 1);
            }
        }
    }

    let mut max_id: Option<usize> = None;
    let mut max_minute_most_spent_sleeping: Option<u8> = None;
    let mut max_minutes_spent_sleeping: Option<usize> = None;

    for (id, guard) in &guards {
        if guard.minutes_asleep.is_empty() {
            continue;
        }

        let minutes_asleep = guard.minutes_asleep.iter().max_by(|&x, &y| x.1.cmp(y.1)).unwrap();
        let (minute_most_spent_sleeping, minutes_spent_sleeping) = (*minutes_asleep.0, *minutes_asleep.1);

        if max_id == None || minutes_spent_sleeping > max_minutes_spent_sleeping.unwrap() {
            max_id = Some(*id);
            max_minute_most_spent_sleeping = Some(minute_most_spent_sleeping);
            max_minutes_spent_sleeping = Some(minutes_spent_sleeping);
        }
    }

    max_id.unwrap() * max_minute_most_spent_sleeping.unwrap() as usize
}

#[cfg(test)]
mod test;
