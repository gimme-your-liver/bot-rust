#![allow(dead_code)]
use gtts::save_to_file;
use rand::seq::SliceRandom;
use std::io;
use std::io::BufReader;
use std::path::Path;

mod chat;

// prompt function
pub fn readline(msg: &str) -> String {
    println!("{}", msg);

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
    welcome();
    
    let greet_choice = chat::GREET_STAN.choose(&mut rand::thread_rng()).unwrap();
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
        if chat::HI_MSG.contains(&inp.trim()) {
            let message = chat::HI_RPLY.choose(&mut rand::thread_rng()).unwrap();
            println!("{}", message);
            tts(&message);
        };
        if  chat::BYE_MSG.contains(&inp.trim()) {
            let message = chat::BYE_RPLY.choose(&mut rand::thread_rng()).unwrap();
            println!("{}", message);
            tts(&message);
        };
        if chat::YES_MSG.contains(&inp.trim()) {
            let message = chat::YES_RPLY.choose(&mut rand::thread_rng()).unwrap();
            println!("{}", message);
            tts(&message);
        };
        if inp.trim() == "why" {
            println!("Oh did I do somehing wrong! sorry!");
        }
    }
}
