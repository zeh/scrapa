use html_escape::decode_html_entities;
use json;
use regex::Regex;
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::io;
use std::io::Write;
use std::thread;
use std::time;
use termion;
use termion::{color, event::Key, input::TermRead, raw::IntoRawMode};

const SOURCE_URL: &str = "https://www.microsoft.com/en-us/surface/devices/compare-devices";
const SAVED_FILE: &str = "past_results.txt";
const WAIT_TIME_SECONDS: u64 = 5 * 60;

struct Device {
    pub name: String,
    pub url: String,
    pub start_price: u16,
    pub year: String,
}

enum ComparisonResult {
    IsSame,
    MustOverwrite,
    MustIgnore,
    MustQuit,
}

enum ReadResult {
    MustQuit,
    MustContinue,
}

fn cleanup_labels(value: String) -> String {
    value.replace("&nbsp;", " ")
}

fn read_source() -> String {
    reqwest::blocking::get(SOURCE_URL).unwrap().text().unwrap()
}

fn gen_results_from_source(source: String) -> String {
    // Read the device data, which is an HTML-escaped JSON object
    let data_regex =
        Regex::new("(?s)<div class=\"d-none\" id=\"ConsumerData\" data-json=\"(.*?)\"></div>")
            .unwrap();
    let data_source = data_regex
        .captures(source.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    // Unescape HTML data and parse the JSON as objects
    let data = json::parse(&decode_html_entities(data_source)).unwrap();

    let devices = data["Devices"]
        .members()
        .filter(|d| d["Product"]["SkuID"] != "")
        .map(|d| {
            let device_details = d["Product"]["DeviceDetails"].clone();
            Device {
                name: cleanup_labels(device_details["DeviceName"].as_str().unwrap().to_string()),
                url: device_details["ShopNowCTA"]["Url"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                year: device_details["Sortfilters"]["year"].as_str().unwrap()[..4].to_string(),
                start_price: 0,
            }
        })
        .collect::<Vec<_>>();

    let mut devices_list = devices
        .iter()
        .map(|d| format!("[{}] {} - ${} - {}", d.year, d.name, d.start_price, d.url))
        .collect::<Vec<_>>();
    devices_list.sort();

    format!("{}\n", devices_list.join("\n"))
}

fn read_results() -> String {
    fs::read_to_string(SAVED_FILE).unwrap_or(String::from(""))
}

fn write_results(results: &str) {
    fs::write(SAVED_FILE, results).unwrap();
}

fn get_terminal_char() -> char {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    loop {
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                Key::Char(letter) => {
                    stdout.lock().flush().unwrap();
                    return letter;
                }
                _ => {}
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }
}

fn compare_results(existing_results: &str, new_results: &str) -> ComparisonResult {
    if existing_results != new_results {
        println!("New results detected!");

        let diff = TextDiff::from_lines(existing_results, new_results);

        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => {
                    print!("{}", color::Fg(color::Red));
                    print!("-{}", change);
                    print!("{}", color::Fg(color::Reset));
                }
                ChangeTag::Insert => {
                    print!("{}", color::Fg(color::Green));
                    print!("+{}", change);
                    print!("{}", color::Fg(color::Reset));
                }
                ChangeTag::Equal => {
                    print!(" {}", change);
                }
            };
        }

        println!("(O)verwrite, (I)gnore, (Q)uit?");

        let mut key: char;
        loop {
            key = get_terminal_char();

            match key {
                'o' => return ComparisonResult::MustOverwrite,
                'i' => return ComparisonResult::MustIgnore,
                'q' => return ComparisonResult::MustQuit,
                _ => {}
            }
        }
    }

    ComparisonResult::IsSame
}

fn read_once() -> ReadResult {
    let page_source = read_source();
    let new_results = gen_results_from_source(page_source);
    let existing_results = read_results();

    let comparison = compare_results(&existing_results, &new_results);

    match comparison {
        ComparisonResult::MustOverwrite => {
            write_results(&new_results);
            println!("Overwriting and continuing.");
        }
        ComparisonResult::MustIgnore => {
            println!("Ignoring results and continuing.");
        }
        ComparisonResult::IsSame => {
            println!("Results are the same.");
        }
        ComparisonResult::MustQuit => return ReadResult::MustQuit,
    }

    ReadResult::MustContinue
}

fn main() {
    let mut count = 0;

    loop {
        match read_once() {
            ReadResult::MustQuit => break,
            _ => {}
        }

        count += 1;

        println!(
            "Attempted {} times; waiting {} seconds until next request.",
            count, WAIT_TIME_SECONDS
        );
        thread::sleep(time::Duration::from_secs(WAIT_TIME_SECONDS));
    }
}
