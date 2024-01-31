use std::collections::HashMap;
use std::env;
use std::process::{Command, Stdio};
use std::io;
use csv;

fn csv_to_dict() -> HashMap<String, String> {
    let csv_data = include_str!("../resources/stations.csv");
    let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
    let mut csv_dict = HashMap::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let key = record[0].to_lowercase();
        let value = record[1].to_string();
        csv_dict.insert(key, value);
    }

    csv_dict
}


fn play_radio(radio_station: &str, radio_station_dict: &HashMap<String, String>) {
    let radio_station_lower = radio_station.to_lowercase();
    if let Some(url) = radio_station_dict.get(&radio_station_lower) {
        let _ = Command::new("pkill")
            .arg("mpv")
            .output();

        println!("Playing: {}", radio_station);
        let _ = Command::new("mpv")
            .arg(url)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    } else {
        eprintln!("Radio station not found.");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let radio_station_dict = csv_to_dict();

    let user_input = if args.len() == 1 {
        println!("Enter a radio station name: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_lowercase()
    } else {
        args[1].to_lowercase()
    };

    if radio_station_dict.contains_key(&user_input) {
        play_radio(&user_input, &radio_station_dict);
    } else if user_input == "list" {
        for station in radio_station_dict.keys() {
            println!("{}", station);
        }
    } else if user_input == "none" {
        Command::new("sh").arg("-c").arg("killall mpv").spawn().unwrap();
    } else {
        println!("This radio station is not in our list.");
    }
}
