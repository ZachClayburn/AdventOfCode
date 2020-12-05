use std::fmt::{Display, Formatter, Result};

use regex::Regex;

const INPUT: &str = include_str!("../../inputs/Day5.txt");

fn main() {
    let seat_numbers = process_input(INPUT);
    let part_1 = seat_numbers
        .iter()
        .map(Seat::get_seat_id)
        .max()
        .unwrap();
    println!("Part 1: Max id = {}", part_1);

    let mut sorted_seats = seat_numbers
        .to_vec();
    sorted_seats.sort();
    let sorted_seats = sorted_seats;

    let mut last_seat = sorted_seats.first().unwrap();
    let mut my_seat = None;
    for seat in sorted_seats.iter().skip(1) {
        let new_id = seat.get_seat_id();
        if new_id != (last_seat.get_seat_id() + 1) {
            my_seat = Some(last_seat.get_seat_id() + 1);
            break;
        }
        last_seat = seat;
    }
    if let Some(seat) = my_seat {
        println!("Part two: My seat {}", seat)
    } else { println!("No solution found!") }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Seat {
    row: u8,
    col: u8,
}

impl Seat {
    fn get_seat_id(&self) -> u32 {
        const ROW_FACTOR: u32 = 8;
        (self.row) as u32 * ROW_FACTOR + (self.col) as u32
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:3},{}) {}", self.row, self.col, self.get_seat_id())
    }
}

fn process_input(input: &str) -> Vec<Seat> {
    let re = Regex::new(r"([FB]{7})([LR]{3})").unwrap();
    re.captures_iter(input)
        .map(|x| (x.get(1).unwrap().as_str(), x.get(2).unwrap().as_str()))
        .map(translate_seat)
        .collect()
}

fn translate_seat((row, col): (&str, &str)) -> Seat {
    let row = &row.replace("F", "0").replace("B", "1");
    let col = &col.replace("R", "1").replace("L", "0");

    Seat {
        row: u8::from_str_radix(&row, 2).unwrap(),
        col: u8::from_str_radix(&col, 2).unwrap(),
    }
}
