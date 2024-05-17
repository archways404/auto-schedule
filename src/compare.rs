use std::fs;
use std::io::{self, BufReader};
use ical::IcalParser;
use ical::parser::ical::component::IcalEvent;
use std::collections::HashSet;

const FILTERED_FILE: &str = "filtered_calendar.ics";
const LATEST_FILE: &str = "latest_calendar.ics";
const ORIGINAL_FILE: &str = "calendar.ics";

pub fn compare_and_update_files() -> io::Result<()> {
    let filtered_events = parse_ics(FILTERED_FILE)?;
    let latest_events = parse_ics(LATEST_FILE)?;

    if filtered_events == latest_events {
        println!("Files contain the same events.");
        fs::remove_file(FILTERED_FILE)?;
        fs::remove_file(ORIGINAL_FILE)?;
    } else {
        println!("Files do not match. Updating the latest file.");
        fs::remove_file(LATEST_FILE)?;
        fs::rename(FILTERED_FILE, LATEST_FILE)?;
        fs::remove_file(ORIGINAL_FILE)?;
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
                    if let Some(event_str) = extract_event_data(&event) {
                        events.insert(event_str);
                    }
                }
            }
            Err(e) => println!("Error parsing {}: {:?}", file_path, e),
        }
    }

    Ok(events)
}

fn extract_event_data(event: &IcalEvent) -> Option<String> {
    let uid = event.properties.iter().find(|prop| prop.name == "UID")?.value.clone()?;
    let dtstart = event.properties.iter().find(|prop| prop.name == "DTSTART")?.value.clone()?;
    let dtend = event.properties.iter().find(|prop| prop.name == "DTEND")?.value.clone()?;
    let summary = event.properties.iter().find(|prop| prop.name == "SUMMARY")?.value.clone()?;
    let location = event.properties.iter().find(|prop| prop.name == "LOCATION")?.value.clone()?;

    Some(format!("{}|{}|{}|{}|{}", uid, dtstart, dtend, summary, location))
}
