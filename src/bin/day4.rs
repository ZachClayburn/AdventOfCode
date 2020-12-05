#[macro_use]
extern crate lazy_static;

use regex::Regex;
use termion::color::{Fg, Red, Green, Reset};

const INPUT: &str = include_str!("../../inputs/Day4.txt");

fn main() {
    let data = get_data(INPUT);
    // for record in data.iter() {
    //     if record.is_valid() {
    //         println!("{}{:?}{}", Fg(Green), record, Fg(Reset));
    //     } else {
    //         println!("{}{:?}{}", Fg(Red), record, Fg(Reset));
    //     }
    // }
    let valid = data
        .iter()
        .filter(|x| x.is_valid())
        .count();
    println!("Part 1: {} valid entries", valid)
}

fn get_data(input: &str) -> Vec<PassportData> {
    let re = Regex::new(r"\n\n").unwrap();
    re.split(input).map(|x| PassportData::new(x)).collect()
}

#[derive(Debug)]
struct PassportData<'a> {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>,
}

impl<'a> PassportData<'a> {
    fn new(record: &'a str) -> Self {
        lazy_static! {
            static ref BIRTH_YEAR_RE: Regex = Regex::new(r"byr:(\d+)").unwrap();
            static ref ISSUE_YEAR_RE: Regex = Regex::new(r"iyr:(\d+)").unwrap();
            static ref EXPIRATION_YEAR_RE: Regex = Regex::new(r"eyr:(\d+)").unwrap();
            static ref HEIGHT_RE: Regex = Regex::new(r"hgt:(\d+)(cm|in)?").unwrap();
            static ref EYE_COLOR_RE: Regex = Regex::new(r"ecl:(#?\w+)").unwrap();
            static ref PASSPORT_ID_RE: Regex = Regex::new(r"pid:(\d+)").unwrap();
            static ref COUNTRY_ID_RE: Regex = Regex::new(r"cid:(\d+)").unwrap();
        }

        let birth_year = BIRTH_YEAR_RE
            .captures(record)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str())
            .and_then(|x| x.parse::<u32>().ok());
        // if birth_year.is_none() {
        //     println!("Missing birth year in \n{}\n", record)
        // }

        let issue_year = ISSUE_YEAR_RE
            .captures(record)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str())
            .and_then(|x| x.parse::<u32>().ok());
        // if issue_year.is_none() {
        //     println!("Missing Issue year in \n{}\n", record)
        // }

        let expiration_year = EXPIRATION_YEAR_RE
            .captures(record)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str())
            .and_then(|x| x.parse::<u32>().ok());
        // if expiration_year.is_none() {
        //     println!("Missing Expiration year in \n{}\n", record)
        // }

        let height = HEIGHT_RE
            .captures(record)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str());
        if height.is_none() {
            println!("Missing Height in \n{}\n", record)
        }

        let eye_color = EYE_COLOR_RE
            .captures(record)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str());
        // if eye_color.is_none() {
        //     println!("Missing eye color in \n{}\n", record)
        // }

        let passport_id = PASSPORT_ID_RE
            .captures(record)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str());
        // if passport_id.is_none() {
        //     println!("Missing passport id in \n{}\n", record)
        // }

        let country_id = COUNTRY_ID_RE
            .captures(record)
            .and_then(|x| x.get(1))
            .map(|x| x.as_str());
        // if country_id.is_none() {
        //     println!("Missing Country Id in \n{}\n", record)
        // }

        PassportData {
            birth_year,
            issue_year,
            expiration_year,
            height,
            eye_color,
            passport_id,
            country_id,
        }
    }

    fn is_valid(&self) -> bool {
        return self.birth_year.is_some() &&
            self.issue_year.is_some() &&
            self.expiration_year.is_some() &&
            self.height.is_some() &&
            self.eye_color.is_some() &&
            self.passport_id.is_some();
    }
}