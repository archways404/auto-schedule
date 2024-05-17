use std::fs;
use std::io::{self, BufReader};
use ical::IcalParser;
use std::collections::HashSet;

const FILTERED_FILE: &str = "filtered_calendar.ics";
const LATEST_FILE: &str = "latest_calendar.ics";

pub fn compare_and_update_files() -> io::Result<()> {
    let filtered_events = parse_ics(FILTERED_FILE)?;
    let latest_events = parse_ics(LATEST_FILE)?;

    if filtered_events == latest_events {
        println!("Files contain the same events.");
    } else {
        println!("Files do not match. Updating the latest file.");
        fs::remove_file(LATEST_FILE)?;
        fs::rename(FILTERED_FILE, LATEST_FILE)?;
    }

    Ok(())
}

fn parse_ics(file_path: &str) -> io::Result<HashSet<String>> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut events = HashSet::new();
    let ical = IcalParser::new(reader);

    for calendar in ical {
        match calendar {
            Ok(calendar) => {
                for event in calendar.events {
                    let event_str = format!("{:?}", event);
                    events.insert(event_str);
                }
            }
            Err(e) => println!("Error parsing {}: {:?}", file_path, e),
        }
    }

    Ok(events)
}
