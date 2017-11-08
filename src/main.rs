#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_xml_rs;

use std::fs::File;
use std::io::{BufReader};

#[derive(Debug, Deserialize)]
struct Question {
    #[serde(rename="$value")]
    text: String,
    // #[serde(default)]
    // measure: String,
}

#[derive(Debug, Deserialize)]
struct VoteTally {
    yeas: String,
    nays: String,
}

#[derive(Debug, Deserialize)]
struct Vote {
    vote_number: String,
    vote_date: String,
    issue: String,
    question: Question,
    result: String,
    vote_tally: VoteTally,
    title: String,
}

#[derive(Debug, Deserialize)]
struct Votes {
    vote: Vec<Vote>,
}

#[derive(Debug, Deserialize)]
struct VoteSummary {
    congress: String,
    session: String,
    congress_year: String,
    votes: Votes,
}

fn main() {
    let file = BufReader::new(
        File::open("./vote_menu_115_1.xml").unwrap()
    );

    let vote_summary: VoteSummary = serde_xml_rs::deserialize(file).unwrap();

    println!("{:#?}", vote_summary);
}
