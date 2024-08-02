use std::{fs, thread};

use chrono::Local;
use email_sender::create_mailer;
use lettre::SmtpTransport;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::time::Duration;

mod email_sender;

fn get_course_url(course_name: &str) -> String {
    // course_name should be in the format of "CS 146"
    let words: Vec<&str> = course_name.split_whitespace().collect();
    assert!(words.len() == 2);

    let term_code = fs::read_to_string("term_code.txt").unwrap();
    let (subject, num) = (words[0], words[1]);
    format!("https://classes.uwaterloo.ca/cgi-bin/cgiwrap/infocour/salook.pl?level=under&sess={}&subject={}&cournum={}", term_code, subject, num)
}

fn check_course_availability(mailer: &SmtpTransport, course_name: &str) {
    println!("Checking availability for: {}", course_name);

    let url = get_course_url(course_name);
    let response = get(url.clone()).expect("Failed to send request");
    let body = response.text().expect("Failed to read response text");
    let document = Html::parse_document(&body);

    let cells: Vec<_> = document
        .select(&Selector::parse("table table tr:nth-of-type(2) td").unwrap())
        .collect();

    // Get enrl_cap and enrl_tot
    let [enrl_cap, enrl_tot] = [cells[6], cells[7]].map(|cell| {
        cell.text()
            .collect::<String>()
            .trim()
            .parse::<i32>()
            .unwrap()
    });

    if enrl_tot < enrl_cap {
        email_sender::send_email(mailer, course_name);
    }
}

fn main() {
    let from_email = fs::read_to_string("from_email.txt").unwrap();
    let mailer = create_mailer(from_email);

    let courses = fs::read_to_string("courses.txt").unwrap();
    let words: Vec<&str> = courses.split_whitespace().collect::<Vec<_>>();
    let course_names: Vec<_> = words
        .chunks(2)
        .map(|chunk| format!("{} {}", chunk[0], chunk[1]))
        .collect();

    // Check for availability every 30 mins
    loop {
        println!("Running at {}", Local::now());

        for course_name in &course_names {
            check_course_availability(&mailer, &course_name)
        }

        thread::sleep(Duration::from_secs(30 * 60));
    }
}
