use serde::{Deserialize, Serialize};
use rand::Rng;

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::env;

#[derive(Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize)]
struct Points {
    points: Vec<Point>,
}

fn getPointsCount() -> u32 {
    let mut input = String::new();

    let mut count: u32 = 0;

    loop {
        input.clear();

        println!("How many points do you need?");
        
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Failed to read line.");
            continue;
        }

        match input.trim().parse::<u32>() {
            Ok(num) if num > 0 && num < 1000000 => {
                count = num;
                break;
            },
            _ =>{
                println!("Invalid input. Try again.");
            }
        }
    }

    count
}

fn writeToFile(jsonString: String, fileName: String) {
    let path = Path::new(fileName.as_str());
    let display = path.display();

    let mut file = match File::create(&path){
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(jsonString.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn round(number: f64) -> f64 {
    (number * 100.0).round() / 100.0
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut filename: String = String::from("points.json");

    if args.len() > 1 {
        filename = args[1].clone();

        if filename.ends_with(".txt") {
            filename = filename.replace(".txt", ".json");
        } else if !filename.ends_with(".json") {
            filename.push_str(".json");
        }

    }

    let count: u32 = getPointsCount();

    let mut points = Points{ 
        points: Vec::new(),
    };
    
    for _ in 0..count {
        points.points.push( Point {
            x: round(rand::thread_rng().gen_range(0.0..10000.0)),
            y: round(rand::thread_rng().gen_range(0.0..10000.0)),
        });
    }

    let json_string = serde_json::to_string_pretty(&points).expect("Error converting to JSON");

    writeToFile(json_string, filename);
}