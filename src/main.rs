#![allow(dead_code)]
use gtts::save_to_file;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::io;
use std::io::BufReader;
use std::path::Path;

// mod chat;

#[derive(Deserialize)]
struct Responses {
    greetings: Vec<String>,
    hi_msg: Vec<String>,
    hi_rply: Vec<String>,
    bye_msg: Vec<String>,
    bye_rply: Vec<String>,
    yes_msg: Vec<String>,
    yes_rply: Vec<String>,
    nice_msg: Vec<String>,
    nice_rply: Vec<String>,
}

// prompt function
pub fn readline(msg: &str) -> String {
    print!("{}", msg);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    user_input
}

// play_mp3 function
fn play_mp3(mp3: &str) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(mp3).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
}

// T.T.S function
fn welcome() {
    let pat = Path::new("./welcome.mp3").exists();
    if pat {
        save_to_file("Welcome back!", "well.mp3");
        play_mp3("well.mp3");
    } else {
        save_to_file("Hello ! My name is 'gimme your liver'. Thank you for choosing me!, You can do exlamation 'discord' for a discord link for my support server! I would be glad to help you along!!", "welcome.mp3");
        readline("Enter your name:");
        play_mp3("welcome.mp3");
    };
}

fn tts(input: &str) {
    save_to_file(input, "tts.mp3");
    play_mp3("tts.mp3");
}
// Main Function::true
fn main() {
    // debug
    let data = include_str!("text.json");
    let responses: Responses = serde_json::from_str(data).unwrap();
    // ---------------
    welcome();

    let greet_choice = responses.greetings.choose(&mut rand::thread_rng()).unwrap();
    // debug

    println!("{}", greet_choice);
    tts(&greet_choice);
    // Commands section
    loop {
        let inp = readline("> ").to_lowercase();

        if inp.trim() == "!help" {
            println!("This is in progress");
            tts("Sorry I did not understand what you said.");
        };
        if responses.hi_msg.contains(&inp.trim().to_owned()) {
            let message = responses.hi_rply.choose(&mut rand::thread_rng()).unwrap();
            println!("{}", message);
            tts(&message);
        };
        if responses.bye_msg.contains(&inp.trim().to_owned()) {
            let message = responses.bye_rply.choose(&mut rand::thread_rng()).unwrap();
            println!("{}", message);
            tts(&message);
        };
        if responses.yes_msg.contains(&inp.trim().to_owned()) {
            let message = responses.yes_rply.choose(&mut rand::thread_rng()).unwrap();
            println!("{}", message);
            tts(&message);
        };
        if responses.nice_msg.contains(&inp.trim().to_owned()) {
            let message = responses.nice_rply.choose(&mut rand::thread_rng()).unwrap();
            println!("{}", message);
            tts(&message);
        };
        if inp.trim() == "why" {
            println!("Oh did I do somehing wrong! sorry!");
        };

    }
}
